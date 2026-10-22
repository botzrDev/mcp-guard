// Copyright (c) 2025 Austin Green
// SPDX-License-Identifier: AGPL-3.0
//
// This file is part of MCP-Guard.
//
// MCP-Guard is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// MCP-Guard is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with MCP-Guard. If not, see <https://www.gnu.org/licenses/>.

//! Flight recorder: tamper-evident, redacted, per-session recording.
//!
//! One JSONL file per session. Every record carries `prev`, the hash of the
//! record before it, and `hash`, the hash of itself (computed over the record
//! with `hash` removed, prefixed by `prev`). Removing, reordering or editing a
//! record breaks the chain at that point, which `verify` reports. With a
//! signing key the hash is an HMAC, so the chain cannot be recomputed by
//! someone who edits the file but lacks the key.
//!
//! Payloads are redacted (built-in secret patterns plus the user's audit
//! redaction rules) and truncated to a configurable size before hashing, so a
//! recording can be shared without leaking credentials. `payload_sha256` is
//! the hash of the original, so a payload you still hold can be matched to it.

use crate::audit::CompiledRedactionRules;
use crate::config::{RecorderConfig, RedactionRule};
use crate::pin::canonical_json;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Format version written into the session header.
pub const RECORD_FORMAT_VERSION: u32 = 1;

/// `prev` of the first record.
pub const GENESIS: &str = "sha256:0000000000000000000000000000000000000000000000000000000000000000";

/// Which way a message was travelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    /// From the agent (client) into the guard.
    ClientToGuard,
    /// From the guard back to the agent.
    GuardToClient,
    /// A guard-internal event: session boundary or decision.
    Guard,
}

/// What a record describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordKind {
    SessionStart,
    SessionEnd,
    Request,
    Response,
    Notification,
    /// A policy or pin decision that changed what the agent saw or could do.
    Decision,
}

/// One line of a session file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub seq: u64,
    /// RFC 3339 with milliseconds, UTC.
    pub ts: String,
    pub dir: Direction,
    pub kind: RecordKind,
    /// JSON-RPC id of the request or response, when there is one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Tool name for tools/call requests, responses and decisions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    /// Redacted, possibly truncated payload (params, result, error, or decision detail).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    /// SHA-256 of the full, unredacted payload as serialized.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub truncated: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub redacted: bool,
    /// Hash of the previous record (`GENESIS` for the first).
    pub prev: String,
    /// Hash of this record: `sha256:` or `hmac-sha256:` prefix.
    pub hash: String,
}

/// Session-level facts, stored in the `session_start` payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub started_at: String,
    pub guard_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_mode: Option<String>,
    #[serde(default)]
    pub signed: bool,
}

// ----------------------------------------------------------------------------
// Hashing
// ----------------------------------------------------------------------------

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// SHA-256 of a payload's canonical JSON, as `sha256:<hex>`.
pub fn payload_hash(payload: &Value) -> String {
    let digest = Sha256::digest(canonical_json(payload).as_bytes());
    format!("sha256:{}", hex(&digest))
}

/// Hash of a record given its predecessor's hash. The record's own `hash`
/// field is excluded from the input.
pub fn record_hash(record: &Record, key: Option<&[u8]>) -> String {
    let mut without_hash = serde_json::to_value(record).unwrap_or(Value::Null);
    if let Some(obj) = without_hash.as_object_mut() {
        obj.remove("hash");
    }
    let body = canonical_json(&without_hash);
    match key {
        Some(k) => {
            let mut mac = Hmac::<Sha256>::new_from_slice(k).expect("HMAC accepts any key length");
            mac.update(record.prev.as_bytes());
            mac.update(b"\n");
            mac.update(body.as_bytes());
            format!("hmac-sha256:{}", hex(&mac.finalize().into_bytes()))
        }
        None => {
            let mut hasher = Sha256::new();
            hasher.update(record.prev.as_bytes());
            hasher.update(b"\n");
            hasher.update(body.as_bytes());
            format!("sha256:{}", hex(&hasher.finalize()))
        }
    }
}

// ----------------------------------------------------------------------------
// Redaction and truncation
// ----------------------------------------------------------------------------

/// Secret shapes redacted whether or not the user configured rules.
pub fn builtin_redaction_rules() -> Vec<RedactionRule> {
    let rule = |name: &str, pattern: &str| RedactionRule {
        name: name.to_string(),
        pattern: pattern.to_string(),
        replacement: format!("[REDACTED:{}]", name),
    };
    vec![
        rule(
            "private_key",
            r"-----BEGIN [A-Z ]*PRIVATE KEY-----[\s\S]*?-----END [A-Z ]*PRIVATE KEY-----",
        ),
        rule("aws_access_key", r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b"),
        rule("github_token", r"\bgh[pousr]_[A-Za-z0-9]{36,}\b"),
        rule("github_pat", r"\bgithub_pat_[A-Za-z0-9_]{60,}\b"),
        rule("slack_token", r"\bxox[abprs]-[A-Za-z0-9-]{10,}\b"),
        rule("openai_key", r"\bsk-[A-Za-z0-9_-]{20,}\b"),
        rule("anthropic_key", r"\bsk-ant-[A-Za-z0-9_-]{20,}\b"),
        rule("google_api_key", r"\bAIza[0-9A-Za-z_-]{35}\b"),
        rule("stripe_key", r"\b[sr]k_(?:live|test)_[A-Za-z0-9]{16,}\b"),
        rule("mcp_guard_key", r"\bmcp_[A-Za-z0-9_-]{30,}\b"),
        rule(
            "jwt",
            r"\beyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\b",
        ),
        rule("bearer", r"(?i)\bbearer\s+[A-Za-z0-9._~+/=-]{16,}"),
        rule("basic_auth_url", r"://[^/\s:@]+:[^/\s@]+@"),
    ]
}

/// Redactor combining built-in patterns with the user's rules.
#[derive(Clone)]
pub struct Redactor {
    rules: CompiledRedactionRules,
}

impl Redactor {
    pub fn new(user_rules: &[RedactionRule]) -> Result<Self, regex::Error> {
        let mut all = builtin_redaction_rules();
        all.extend(user_rules.iter().cloned());
        Ok(Self {
            rules: CompiledRedactionRules::new(&all)?,
        })
    }

    /// Redact every string inside a JSON value. Returns the value and whether
    /// anything changed.
    pub fn redact_value(&self, value: &Value) -> (Value, bool) {
        let mut changed = false;
        let out = self.walk(value, &mut changed);
        (out, changed)
    }

    fn walk(&self, value: &Value, changed: &mut bool) -> Value {
        match value {
            Value::String(s) => {
                let r = self.rules.redact(s);
                if &r != s {
                    *changed = true;
                }
                Value::String(r)
            }
            Value::Array(items) => {
                Value::Array(items.iter().map(|v| self.walk(v, changed)).collect())
            }
            Value::Object(map) => Value::Object(
                map.iter()
                    .map(|(k, v)| (k.clone(), self.walk(v, changed)))
                    .collect(),
            ),
            other => other.clone(),
        }
    }
}

/// Replace an oversized payload with a preview. Returns (payload, truncated).
pub fn truncate_payload(payload: Value, max_bytes: usize) -> (Value, bool) {
    let serialized = payload.to_string();
    if serialized.len() <= max_bytes {
        return (payload, false);
    }
    let preview_len = max_bytes.min(1024);
    let mut end = preview_len;
    while end > 0 && !serialized.is_char_boundary(end) {
        end -= 1;
    }
    (
        json!({
            "_truncated": true,
            "original_bytes": serialized.len(),
            "preview": &serialized[..end]
        }),
        true,
    )
}

// ----------------------------------------------------------------------------
// Writer
// ----------------------------------------------------------------------------

struct WriterState {
    file: std::io::BufWriter<std::fs::File>,
    seq: u64,
    prev: String,
}

/// Appends records for one session to a file.
pub struct Recorder {
    path: PathBuf,
    session_id: String,
    max_payload_bytes: usize,
    include_results: bool,
    redactor: Option<Redactor>,
    key: Option<Vec<u8>>,
    state: Mutex<WriterState>,
}

/// File name for a new session: sortable timestamp plus a short random id.
pub fn session_file_name(started: &chrono::DateTime<chrono::Utc>, session_id: &str) -> String {
    format!("{}-{}.jsonl", started.format("%Y%m%d-%H%M%S"), session_id)
}

impl Recorder {
    /// Open a new session file in `dir` and write the `session_start` record.
    pub fn start(
        config: &RecorderConfig,
        dir: &Path,
        mut info: SessionInfo,
        user_redaction_rules: &[RedactionRule],
    ) -> std::io::Result<Self> {
        std::fs::create_dir_all(dir)?;
        if let Err(e) = prune_sessions(dir, config.retention_days, config.max_sessions) {
            tracing::warn!(error = %e, dir = %dir.display(), "Could not prune old recordings");
        }
        let started = chrono::Utc::now();
        let session_id = uuid::Uuid::new_v4().simple().to_string()[..12].to_string();
        let path = dir.join(session_file_name(&started, &session_id));
        let file = std::fs::OpenOptions::new()
            .create_new(true)
            .append(true)
            .open(&path)?;

        let key = config.signing_key();
        let redactor = if config.redact {
            Some(Redactor::new(user_redaction_rules).map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string())
            })?)
        } else {
            None
        };

        info.session_id = session_id.clone();
        info.started_at = started.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        info.signed = key.is_some();

        let recorder = Self {
            path,
            session_id,
            max_payload_bytes: config.max_payload_bytes,
            include_results: config.include_results,
            redactor,
            key,
            state: Mutex::new(WriterState {
                file: std::io::BufWriter::new(file),
                seq: 0,
                prev: GENESIS.to_string(),
            }),
        };
        let payload = serde_json::to_value(&info).unwrap_or(Value::Null);
        recorder.append(
            Direction::Guard,
            RecordKind::SessionStart,
            None,
            None,
            None,
            Some(payload),
        )?;
        Ok(recorder)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Record a message arriving from the client.
    pub fn record_client_message(
        &self,
        message: &crate::transport::Message,
    ) -> std::io::Result<()> {
        let kind = if message.is_response() {
            RecordKind::Response
        } else if message.is_notification() {
            RecordKind::Notification
        } else {
            RecordKind::Request
        };
        if kind == RecordKind::Response && !self.include_results {
            return Ok(());
        }
        let tool = tool_of(message);
        let payload = message_payload(message);
        self.append(
            Direction::ClientToGuard,
            kind,
            message.id.clone(),
            message.method.clone(),
            tool,
            payload,
        )
    }

    /// Record a message the guard is about to send to the client. `method` and
    /// `tool` describe the request this answers, when known.
    pub fn record_outgoing_message(
        &self,
        message: &crate::transport::Message,
        method: Option<&str>,
        tool: Option<&str>,
    ) -> std::io::Result<()> {
        let kind = if message.is_response() {
            RecordKind::Response
        } else if message.is_notification() {
            RecordKind::Notification
        } else {
            RecordKind::Request
        };
        if kind == RecordKind::Response && !self.include_results {
            // Still record errors: a denial is evidence even when results are skipped.
            if message.error.is_none() {
                return Ok(());
            }
        }
        let payload = message_payload(message);
        self.append(
            Direction::GuardToClient,
            kind,
            message.id.clone(),
            message.method.clone().or_else(|| method.map(String::from)),
            tool.map(String::from).or_else(|| tool_of(message)),
            payload,
        )
    }

    /// Record a guard decision (policy denial, observe-mode would-deny, hidden tool).
    pub fn record_decision(
        &self,
        decision: &str,
        tool: Option<&str>,
        detail: Value,
    ) -> std::io::Result<()> {
        self.append(
            Direction::Guard,
            RecordKind::Decision,
            None,
            Some(decision.to_string()),
            tool.map(String::from),
            Some(detail),
        )
    }

    /// Write the `session_end` record and flush.
    pub fn finish(&self, reason: &str) -> std::io::Result<()> {
        self.append(
            Direction::Guard,
            RecordKind::SessionEnd,
            None,
            None,
            None,
            Some(json!({ "reason": reason })),
        )?;
        let mut state = self.state.lock().map_err(|_| poisoned())?;
        state.file.flush()
    }

    fn append(
        &self,
        dir: Direction,
        kind: RecordKind,
        id: Option<Value>,
        method: Option<String>,
        tool: Option<String>,
        payload: Option<Value>,
    ) -> std::io::Result<()> {
        let (payload, payload_sha256, truncated, redacted) = match payload {
            None => (None, None, false, false),
            Some(original) => {
                let sha = payload_hash(&original);
                let (value, redacted) = match &self.redactor {
                    Some(r) => r.redact_value(&original),
                    None => (original, false),
                };
                let (value, truncated) = truncate_payload(value, self.max_payload_bytes);
                (Some(value), Some(sha), truncated, redacted)
            }
        };

        let mut state = self.state.lock().map_err(|_| poisoned())?;
        let mut record = Record {
            seq: state.seq,
            ts: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            dir,
            kind,
            id,
            method,
            tool,
            payload,
            payload_sha256,
            truncated,
            redacted,
            prev: state.prev.clone(),
            hash: String::new(),
        };
        record.hash = record_hash(&record, self.key.as_deref());
        let line = serde_json::to_string(&record)?;
        state.file.write_all(line.as_bytes())?;
        state.file.write_all(b"\n")?;
        state.file.flush()?;
        state.seq += 1;
        state.prev = record.hash;
        Ok(())
    }
}

fn poisoned() -> std::io::Error {
    std::io::Error::other("recorder lock poisoned")
}

fn tool_of(message: &crate::transport::Message) -> Option<String> {
    if message.method.as_deref() == Some("tools/call") {
        message
            .params
            .as_ref()
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .map(String::from)
    } else {
        None
    }
}

fn message_payload(message: &crate::transport::Message) -> Option<Value> {
    if let Some(err) = &message.error {
        return Some(json!({ "error": err }));
    }
    if let Some(result) = &message.result {
        return Some(json!({ "result": result }));
    }
    message.params.as_ref().map(|p| json!({ "params": p }))
}

// ----------------------------------------------------------------------------
// Reading and verifying
// ----------------------------------------------------------------------------

/// A session file read back from disk.
#[derive(Debug, Clone)]
pub struct Session {
    pub path: PathBuf,
    pub records: Vec<Record>,
    /// Lines that did not parse as records (corruption or foreign content).
    pub unparsed_lines: Vec<usize>,
}

impl Session {
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut records = Vec::new();
        let mut unparsed_lines = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            match serde_json::from_str::<Record>(line) {
                Ok(r) => records.push(r),
                Err(_) => unparsed_lines.push(i + 1),
            }
        }
        Ok(Self {
            path: path.to_path_buf(),
            records,
            unparsed_lines,
        })
    }

    pub fn info(&self) -> Option<SessionInfo> {
        self.records
            .iter()
            .find(|r| r.kind == RecordKind::SessionStart)
            .and_then(|r| r.payload.clone())
            .and_then(|p| serde_json::from_value(p).ok())
    }

    pub fn ended(&self) -> bool {
        self.records
            .last()
            .map(|r| r.kind == RecordKind::SessionEnd)
            .unwrap_or(false)
    }
}

/// Outcome of checking a session's hash chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VerifyResult {
    pub records: usize,
    pub ok: bool,
    /// Sequence number of the first record whose chain link is wrong.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_bad_seq: Option<u64>,
    pub problems: Vec<String>,
    /// Whether the records claim to be HMAC-signed.
    pub signed: bool,
    /// Whether a key was available to check signatures.
    pub key_checked: bool,
}

/// Recompute every link in the chain.
///
/// Without a key, HMAC-signed records can only be checked for sequence and
/// `prev` continuity; the result says so through `key_checked = false`.
pub fn verify(session: &Session, key: Option<&[u8]>) -> VerifyResult {
    let signed = session
        .records
        .first()
        .map(|r| r.hash.starts_with("hmac-"))
        .unwrap_or(false);
    let mut result = VerifyResult {
        records: session.records.len(),
        ok: true,
        first_bad_seq: None,
        problems: Vec::new(),
        signed,
        key_checked: !signed || key.is_some(),
    };
    for line in &session.unparsed_lines {
        result.ok = false;
        result
            .problems
            .push(format!("line {} is not a valid record", line));
    }
    if session.records.is_empty() {
        result.ok = false;
        result.problems.push("no records".into());
        return result;
    }

    let mut expected_prev = GENESIS.to_string();
    for (i, record) in session.records.iter().enumerate() {
        let mut bad = Vec::new();
        if record.seq != i as u64 {
            bad.push(format!("seq {} at position {}", record.seq, i));
        }
        if record.prev != expected_prev {
            bad.push("prev does not match the previous record's hash".to_string());
        }
        let recompute = if record.hash.starts_with("hmac-") {
            key.map(|k| record_hash(record, Some(k)))
        } else {
            Some(record_hash(record, None))
        };
        if let Some(expected) = recompute {
            if expected != record.hash {
                bad.push("hash does not match record contents".to_string());
            }
        }
        if !bad.is_empty() {
            result.ok = false;
            if result.first_bad_seq.is_none() {
                result.first_bad_seq = Some(record.seq);
            }
            result
                .problems
                .push(format!("record {}: {}", record.seq, bad.join("; ")));
        }
        expected_prev = record.hash.clone();
    }
    if !session.ended() {
        result
            .problems
            .push("session has no session_end record (still running, or cut off)".into());
    }
    result
}

/// Session files in a directory, newest first.
pub fn list_sessions(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = match std::fs::read_dir(dir) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|x| x == "jsonl").unwrap_or(false))
            .collect(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(e) => return Err(e),
    };
    files.sort();
    files.reverse();
    Ok(files)
}

/// Delete session files beyond the retention settings. Returns how many were removed.
///
/// Files are judged by the timestamp in their name (`YYYYMMDD-HHMMSS-...`), so
/// a copied file keeps its original age. Files that do not follow the naming
/// scheme are left alone.
pub fn prune_sessions(
    dir: &Path,
    retention_days: Option<u32>,
    max_sessions: Option<usize>,
) -> std::io::Result<usize> {
    if retention_days.is_none() && max_sessions.is_none() {
        return Ok(0);
    }
    let mut sessions = list_sessions(dir)?; // newest first
    sessions.retain(|p| session_started_at(p).is_some());
    let mut removed = 0;
    if let Some(days) = retention_days {
        let cutoff = chrono::Utc::now() - chrono::Duration::days(days as i64);
        let (keep, old): (Vec<PathBuf>, Vec<PathBuf>) = sessions
            .into_iter()
            .partition(|p| session_started_at(p).map(|t| t >= cutoff).unwrap_or(true));
        for p in old {
            std::fs::remove_file(&p)?;
            removed += 1;
        }
        sessions = keep;
    }
    if let Some(max) = max_sessions {
        // The session about to be created counts toward the limit.
        let allowed = max.saturating_sub(1);
        for p in sessions.iter().skip(allowed) {
            std::fs::remove_file(p)?;
            removed += 1;
        }
    }
    Ok(removed)
}

/// Start time encoded in a session file name.
pub fn session_started_at(path: &Path) -> Option<chrono::DateTime<chrono::Utc>> {
    let name = path.file_name()?.to_str()?;
    let stamp = name.get(0..15)?;
    chrono::NaiveDateTime::parse_from_str(stamp, "%Y%m%d-%H%M%S")
        .ok()
        .map(|naive| naive.and_utc())
}

/// Resolve `--session`: `latest`, a full path, a file name, or a session id prefix.
pub fn find_session(dir: &Path, selector: &str) -> std::io::Result<Option<PathBuf>> {
    let as_path = Path::new(selector);
    if as_path.is_file() {
        return Ok(Some(as_path.to_path_buf()));
    }
    let sessions = list_sessions(dir)?;
    if selector == "latest" {
        return Ok(sessions.into_iter().next());
    }
    Ok(sessions.into_iter().find(|p| {
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        name == selector
            || name.trim_end_matches(".jsonl").ends_with(selector)
            || name.contains(selector)
    }))
}

// ----------------------------------------------------------------------------
// Timeline rendering
// ----------------------------------------------------------------------------

/// One line of a human-readable timeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TimelineEntry {
    pub ts: String,
    pub seq: u64,
    pub label: String,
    pub detail: String,
    pub denied: bool,
}

fn short_json(value: &Value, max: usize) -> String {
    let s = value.to_string();
    if s.chars().count() <= max {
        return s;
    }
    let cut: String = s.chars().take(max.saturating_sub(1)).collect();
    format!("{}…", cut)
}

fn time_only(ts: &str) -> String {
    ts.get(11..23)
        .map(|s| s.to_string())
        .unwrap_or_else(|| ts.to_string())
}

/// Turn a session into timeline entries: one per client request, with the
/// matching response folded in, plus decisions and session boundaries.
pub fn timeline(session: &Session) -> Vec<TimelineEntry> {
    let mut out = Vec::new();
    let responses: std::collections::HashMap<String, &Record> = session
        .records
        .iter()
        .filter(|r| r.kind == RecordKind::Response && r.dir == Direction::GuardToClient)
        .filter_map(|r| r.id.as_ref().map(|id| (id.to_string(), r)))
        .collect();

    for record in &session.records {
        match record.kind {
            RecordKind::SessionStart => {
                let info = record
                    .payload
                    .clone()
                    .and_then(|p| serde_json::from_value::<SessionInfo>(p).ok())
                    .unwrap_or_default();
                out.push(TimelineEntry {
                    ts: time_only(&record.ts),
                    seq: record.seq,
                    label: "session start".into(),
                    detail: format!(
                        "{}{}{}",
                        info.upstream.unwrap_or_else(|| "(no upstream)".into()),
                        info.policy_mode
                            .map(|m| format!("  policy={}", m))
                            .unwrap_or_default(),
                        if info.signed { "  signed" } else { "" }
                    ),
                    denied: false,
                });
            }
            RecordKind::SessionEnd => out.push(TimelineEntry {
                ts: time_only(&record.ts),
                seq: record.seq,
                label: "session end".into(),
                detail: record
                    .payload
                    .as_ref()
                    .and_then(|p| p.get("reason"))
                    .and_then(|r| r.as_str())
                    .unwrap_or("")
                    .to_string(),
                denied: false,
            }),
            RecordKind::Request if record.dir == Direction::ClientToGuard => {
                let method = record.method.clone().unwrap_or_default();
                let label = match &record.tool {
                    Some(t) => format!("{} {}", method, t),
                    None => method.clone(),
                };
                let args = record
                    .payload
                    .as_ref()
                    .and_then(|p| p.get("params"))
                    .map(|p| {
                        if method == "tools/call" {
                            p.get("arguments").cloned().unwrap_or(Value::Null)
                        } else {
                            p.clone()
                        }
                    })
                    .filter(|v| !v.is_null())
                    .map(|v| short_json(&v, 80))
                    .unwrap_or_default();
                let response = record
                    .id
                    .as_ref()
                    .and_then(|id| responses.get(&id.to_string()));
                let (outcome, denied) = match response {
                    None => ("(no response recorded)".to_string(), false),
                    Some(r) => match r.payload.as_ref() {
                        Some(p) if p.get("error").is_some() => {
                            let err = &p["error"];
                            let code = err.get("code").and_then(|c| c.as_i64()).unwrap_or(0);
                            let msg = err.get("message").and_then(|m| m.as_str()).unwrap_or("");
                            let denied = matches!(code, -32010 | -32011);
                            (
                                format!("{} {}", if denied { "DENIED" } else { "error" }, msg),
                                denied,
                            )
                        }
                        Some(p) => {
                            let result = p.get("result").cloned().unwrap_or(Value::Null);
                            let summary = if method == "tools/list" {
                                format!(
                                    "{} tools",
                                    result
                                        .get("tools")
                                        .and_then(|t| t.as_array())
                                        .map(|a| a.len())
                                        .unwrap_or(0)
                                )
                            } else if method == "initialize" {
                                format!(
                                    "{} {} ({})",
                                    result["serverInfo"]["name"].as_str().unwrap_or("?"),
                                    result["serverInfo"]["version"].as_str().unwrap_or(""),
                                    result["protocolVersion"].as_str().unwrap_or("?")
                                )
                            } else if r.truncated {
                                "ok (large result, truncated)".to_string()
                            } else {
                                format!("ok {}", short_json(&result, 60))
                            };
                            (summary, false)
                        }
                        None => ("ok".to_string(), false),
                    },
                };
                out.push(TimelineEntry {
                    ts: time_only(&record.ts),
                    seq: record.seq,
                    label,
                    detail: if args.is_empty() {
                        outcome
                    } else {
                        format!("{}  → {}", args, outcome)
                    },
                    denied,
                });
            }
            RecordKind::Decision => {
                let decision = record.method.clone().unwrap_or_default();
                let detail = record
                    .payload
                    .as_ref()
                    .map(|p| {
                        p.get("reason")
                            .and_then(|r| r.as_str())
                            .map(String::from)
                            .unwrap_or_else(|| short_json(p, 100))
                    })
                    .unwrap_or_default();
                out.push(TimelineEntry {
                    ts: time_only(&record.ts),
                    seq: record.seq,
                    label: match &record.tool {
                        Some(t) => format!("decision {} {}", decision, t),
                        None => format!("decision {}", decision),
                    },
                    detail,
                    denied: decision.contains("deny") || decision.contains("hidden"),
                });
            }
            _ => {}
        }
    }
    out
}

/// Render entries as text lines.
pub fn render_timeline(entries: &[TimelineEntry]) -> String {
    let width = entries
        .iter()
        .map(|e| e.label.len())
        .max()
        .unwrap_or(10)
        .min(40);
    entries
        .iter()
        .map(|e| {
            let mut label = e.label.clone();
            if label.len() > width {
                label.truncate(width);
            }
            format!(
                "{}  {:<width$}  {}{}",
                e.ts,
                label,
                if e.denied { "✗ " } else { "" },
                e.detail,
                width = width
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ----------------------------------------------------------------------------
// HTML report
// ----------------------------------------------------------------------------

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Render a session as a self-contained HTML page: metadata, chain status,
/// timeline with denials highlighted, and the raw records in a details block.
pub fn render_html(session: &Session, verification: VerifyResult) -> String {
    let info = session.info().unwrap_or_default();
    let entries = timeline(session);
    let denials = entries.iter().filter(|e| e.denied).count();
    let chain = if verification.ok {
        if verification.key_checked {
            ("ok", "Chain OK".to_string())
        } else {
            (
                "ok",
                "Chain continuity OK (signed; signatures not checked)".to_string(),
            )
        }
    } else {
        (
            "bad",
            format!(
                "Chain broken at record {}",
                verification
                    .first_bad_seq
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "?".into())
            ),
        )
    };

    let mut rows = String::new();
    for e in &entries {
        rows.push_str(&format!(
            "<tr class=\"{}\"><td class=\"ts\">{}</td><td class=\"seq\">{}</td><td class=\"label\">{}</td><td class=\"detail\">{}</td></tr>\n",
            if e.denied { "denied" } else { "" },
            html_escape(&e.ts),
            e.seq,
            html_escape(&e.label),
            html_escape(&e.detail)
        ));
    }

    let mut raw = String::new();
    for r in &session.records {
        raw.push_str(&html_escape(&serde_json::to_string(r).unwrap_or_default()));
        raw.push('\n');
    }

    format!(
        r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>mcp-guard session {sid}</title>
<style>
:root {{ --bg:#fff; --fg:#1b1b1b; --muted:#666; --line:#e4e4e4; --deny:#fff1f0; --deny-fg:#a40000; --ok:#1a7f37; --bad:#a40000; font-family: ui-sans-serif, system-ui, sans-serif; }}
@media (prefers-color-scheme: dark) {{ :root {{ --bg:#111; --fg:#eee; --muted:#9a9a9a; --line:#2a2a2a; --deny:#3a1414; --deny-fg:#ff8a80; --ok:#56d364; --bad:#ff8a80; }} }}
body {{ margin:0; padding:24px 16px; background:var(--bg); color:var(--fg); line-height:1.45; }}
main {{ max-width: 1100px; margin: 0 auto; }}
h1 {{ font-size: 1.4rem; margin: 0 0 4px; }}
.meta {{ color: var(--muted); font-size: .9rem; margin-bottom: 16px; }}
.meta code {{ font-size: .85rem; }}
.badge {{ display:inline-block; padding:2px 8px; border-radius:999px; font-weight:600; font-size:.85rem; }}
.badge.ok {{ background: color-mix(in srgb, var(--ok) 15%, transparent); color: var(--ok); }}
.badge.bad {{ background: color-mix(in srgb, var(--bad) 15%, transparent); color: var(--bad); }}
table {{ width:100%; border-collapse: collapse; font-size: .9rem; }}
th, td {{ text-align:left; padding: 6px 8px; border-bottom: 1px solid var(--line); vertical-align: top; }}
th {{ color: var(--muted); font-weight: 600; font-size:.8rem; text-transform: uppercase; letter-spacing:.03em; }}
td.ts, td.seq {{ white-space: nowrap; color: var(--muted); font-variant-numeric: tabular-nums; }}
td.label {{ white-space: nowrap; font-family: ui-monospace, monospace; }}
td.detail {{ font-family: ui-monospace, monospace; word-break: break-word; }}
tr.denied td {{ background: var(--deny); color: var(--deny-fg); }}
details {{ margin-top: 24px; }}
summary {{ cursor: pointer; color: var(--muted); }}
pre {{ font-size: .75rem; overflow-x: auto; padding: 12px; border: 1px solid var(--line); border-radius: 6px; }}
</style>
</head>
<body>
<main>
<h1>mcp-guard session {sid}</h1>
<div class="meta">
  <div>Upstream: <code>{upstream}</code></div>
  <div>Started: {started} &nbsp;·&nbsp; Policy mode: {mode} &nbsp;·&nbsp; {records} records &nbsp;·&nbsp; {denials} denials or decisions &nbsp;·&nbsp; {ended}</div>
  <div>Integrity: <span class="badge {chain_class}">{chain_text}</span>{signed}</div>
  <div>File: <code>{file}</code></div>
</div>
<table>
<thead><tr><th>Time (UTC)</th><th>#</th><th>Event</th><th>Detail</th></tr></thead>
<tbody>
{rows}</tbody>
</table>
<details>
<summary>Raw records ({records} lines, JSON)</summary>
<pre>{raw}</pre>
</details>
</main>
</body>
</html>
"##,
        sid = html_escape(&info.session_id),
        upstream = html_escape(info.upstream.as_deref().unwrap_or("(none)")),
        started = html_escape(&info.started_at),
        mode = html_escape(info.policy_mode.as_deref().unwrap_or("?")),
        records = session.records.len(),
        denials = denials,
        ended = if session.ended() {
            "session ended cleanly"
        } else {
            "no session_end record"
        },
        chain_class = chain.0,
        chain_text = html_escape(&chain.1),
        signed = if info.signed {
            " &nbsp;(HMAC-signed)"
        } else {
            ""
        },
        file = html_escape(&session.path.display().to_string()),
        rows = rows,
        raw = raw,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::Message;

    fn config() -> RecorderConfig {
        RecorderConfig {
            enabled: true,
            ..RecorderConfig::default()
        }
    }

    fn call(id: i64, tool: &str, args: Value) -> Message {
        Message::request(
            json!(id),
            "tools/call",
            Some(json!({"name": tool, "arguments": args})),
        )
    }

    #[test]
    fn records_chain_and_verify() {
        let dir = tempfile::tempdir().unwrap();
        let rec = Recorder::start(&config(), dir.path(), SessionInfo::default(), &[]).unwrap();
        rec.record_client_message(&call(1, "read_file", json!({"path": "/a"})))
            .unwrap();
        rec.record_outgoing_message(
            &Message::response(json!(1), json!({"content": []})),
            Some("tools/call"),
            Some("read_file"),
        )
        .unwrap();
        rec.record_decision("policy_deny", Some("rm"), json!({"reason": "nope"}))
            .unwrap();
        rec.finish("stdin closed").unwrap();

        let session = Session::load(rec.path()).unwrap();
        assert_eq!(session.records.len(), 5);
        assert_eq!(session.records[0].kind, RecordKind::SessionStart);
        assert_eq!(session.records[0].prev, GENESIS);
        assert_eq!(session.records[1].prev, session.records[0].hash);
        assert!(session.ended());
        let v = verify(&session, None);
        assert!(v.ok, "{:?}", v.problems);
        assert!(v.key_checked);

        let entries = timeline(&session);
        let text = render_timeline(&entries);
        assert!(text.contains("tools/call read_file"), "{}", text);
        assert!(text.contains("decision policy_deny rm"), "{}", text);
        assert!(text.contains("session end"), "{}", text);
    }

    #[test]
    fn verify_detects_edit_removal_and_reorder() {
        let dir = tempfile::tempdir().unwrap();
        let rec = Recorder::start(&config(), dir.path(), SessionInfo::default(), &[]).unwrap();
        for i in 1..=3 {
            rec.record_client_message(&call(i, "read_file", json!({"path": format!("/{}", i)})))
                .unwrap();
        }
        rec.finish("done").unwrap();
        let original = std::fs::read_to_string(rec.path()).unwrap();
        let lines: Vec<&str> = original.lines().collect();

        // Edit a payload.
        let edited = original.replace("\"/2\"", "\"/evil\"");
        std::fs::write(rec.path(), &edited).unwrap();
        let v = verify(&Session::load(rec.path()).unwrap(), None);
        assert!(!v.ok);
        assert_eq!(v.first_bad_seq, Some(2));

        // Remove a record.
        let mut removed = lines.clone();
        removed.remove(2);
        std::fs::write(rec.path(), removed.join("\n")).unwrap();
        let v = verify(&Session::load(rec.path()).unwrap(), None);
        assert!(!v.ok);
        assert_eq!(v.first_bad_seq, Some(3));

        // Reorder two records.
        let mut swapped = lines.clone();
        swapped.swap(1, 2);
        std::fs::write(rec.path(), swapped.join("\n")).unwrap();
        let v = verify(&Session::load(rec.path()).unwrap(), None);
        assert!(!v.ok);

        // Garbage line.
        std::fs::write(rec.path(), format!("{}\nnot json\n", original)).unwrap();
        let v = verify(&Session::load(rec.path()).unwrap(), None);
        assert!(!v.ok);
        assert!(v.problems.iter().any(|p| p.contains("not a valid record")));

        // Intact again.
        std::fs::write(rec.path(), &original).unwrap();
        assert!(verify(&Session::load(rec.path()).unwrap(), None).ok);
    }

    #[test]
    fn signed_chain_needs_the_key() {
        let dir = tempfile::tempdir().unwrap();
        std::env::set_var("MCP_GUARD_TEST_REC_KEY", "correct horse battery staple");
        let cfg = RecorderConfig {
            signing_key_env: Some("MCP_GUARD_TEST_REC_KEY".into()),
            ..config()
        };
        let rec = Recorder::start(&cfg, dir.path(), SessionInfo::default(), &[]).unwrap();
        rec.record_client_message(&call(1, "read_file", json!({})))
            .unwrap();
        rec.finish("done").unwrap();
        let session = Session::load(rec.path()).unwrap();
        assert!(session.records[0].hash.starts_with("hmac-sha256:"));
        assert!(session.info().unwrap().signed);

        let with_key = verify(&session, Some(b"correct horse battery staple"));
        assert!(with_key.ok && with_key.key_checked);
        let wrong_key = verify(&session, Some(b"wrong"));
        assert!(!wrong_key.ok);
        let no_key = verify(&session, None);
        assert!(no_key.ok, "continuity still checks out");
        assert!(!no_key.key_checked);

        // An attacker without the key cannot rechain after editing.
        let edited = std::fs::read_to_string(rec.path())
            .unwrap()
            .replace("read_file", "rm_rf");
        std::fs::write(rec.path(), edited).unwrap();
        assert!(
            !verify(
                &Session::load(rec.path()).unwrap(),
                Some(b"correct horse battery staple")
            )
            .ok
        );
        std::env::remove_var("MCP_GUARD_TEST_REC_KEY");
    }

    #[test]
    fn payloads_are_redacted_and_hash_of_original_kept() {
        let dir = tempfile::tempdir().unwrap();
        let user_rule = RedactionRule {
            name: "ticket".into(),
            pattern: r"TICKET-\d+".into(),
            replacement: "[TICKET]".into(),
        };
        let rec =
            Recorder::start(&config(), dir.path(), SessionInfo::default(), &[user_rule]).unwrap();
        let args = json!({
            "token": "ghp_abcdefghijklmnopqrstuvwxyz0123456789ABCD",
            "note": "see TICKET-42",
            "url": "https://user:pass@example.com/x",
            "pem": "-----BEGIN RSA PRIVATE KEY-----\nMIIE...\n-----END RSA PRIVATE KEY-----"
        });
        rec.record_client_message(&call(1, "post", args.clone()))
            .unwrap();
        rec.finish("done").unwrap();

        let text = std::fs::read_to_string(rec.path()).unwrap();
        assert!(!text.contains("ghp_abcdefghijklmnopqrstuvwxyz0123456789ABCD"));
        assert!(text.contains("[REDACTED:github_token]"));
        assert!(text.contains("[TICKET]"));
        assert!(!text.contains("user:pass@"));
        assert!(!text.contains("MIIE"));
        let session = Session::load(rec.path()).unwrap();
        let rec1 = &session.records[1];
        assert!(rec1.redacted);
        assert_eq!(
            rec1.payload_sha256.as_deref(),
            Some(payload_hash(&json!({"params": {"name": "post", "arguments": args}})).as_str())
        );
        assert!(
            verify(&session, None).ok,
            "hash covers the stored (redacted) record"
        );
    }

    #[test]
    fn oversized_payloads_are_truncated() {
        let dir = tempfile::tempdir().unwrap();
        let cfg = RecorderConfig {
            max_payload_bytes: 200,
            ..config()
        };
        let rec = Recorder::start(&cfg, dir.path(), SessionInfo::default(), &[]).unwrap();
        let big = "x".repeat(5000);
        rec.record_outgoing_message(
            &Message::response(
                json!(1),
                json!({"content": [{"type": "text", "text": big}]}),
            ),
            Some("tools/call"),
            Some("read_file"),
        )
        .unwrap();
        rec.finish("done").unwrap();
        let session = Session::load(rec.path()).unwrap();
        let r = &session.records[1];
        assert!(r.truncated);
        assert_eq!(r.payload.as_ref().unwrap()["_truncated"], true);
        assert!(r.payload.as_ref().unwrap().to_string().len() < 2000);
        assert!(verify(&session, None).ok);
    }

    #[test]
    fn include_results_false_keeps_errors() {
        let dir = tempfile::tempdir().unwrap();
        let cfg = RecorderConfig {
            include_results: false,
            ..config()
        };
        let rec = Recorder::start(&cfg, dir.path(), SessionInfo::default(), &[]).unwrap();
        rec.record_outgoing_message(
            &Message::response(json!(1), json!({"ok": true})),
            None,
            None,
        )
        .unwrap();
        rec.record_outgoing_message(
            &Message::error_response(Some(json!(2)), -32010, "denied"),
            None,
            None,
        )
        .unwrap();
        rec.finish("done").unwrap();
        let session = Session::load(rec.path()).unwrap();
        let kinds: Vec<RecordKind> = session.records.iter().map(|r| r.kind).collect();
        assert_eq!(
            kinds,
            vec![
                RecordKind::SessionStart,
                RecordKind::Response,
                RecordKind::SessionEnd
            ]
        );
        assert!(session.records[1]
            .payload
            .as_ref()
            .unwrap()
            .get("error")
            .is_some());
    }

    #[test]
    fn html_report_contains_timeline_and_escapes() {
        let dir = tempfile::tempdir().unwrap();
        let rec = Recorder::start(&config(), dir.path(), SessionInfo::default(), &[]).unwrap();
        rec.record_client_message(&call(
            1,
            "read_file",
            json!({"path": "<script>alert(1)</script>"}),
        ))
        .unwrap();
        rec.record_outgoing_message(
            &Message::error_response(Some(json!(1)), -32010, "denied"),
            Some("tools/call"),
            Some("read_file"),
        )
        .unwrap();
        rec.finish("done").unwrap();
        let session = Session::load(rec.path()).unwrap();
        let page = render_html(&session, verify(&session, None));
        assert!(page.starts_with("<!doctype html>"));
        assert!(page.contains("tools/call read_file"));
        assert!(page.contains("class=\"denied\""));
        assert!(page.contains("Chain OK"));
        assert!(!page.contains("<script>alert(1)</script>"), "escaped");
        assert!(page.contains("&lt;script&gt;"));
    }

    #[test]
    fn prune_by_age_and_count() {
        let dir = tempfile::tempdir().unwrap();
        let old = dir.path().join("20200101-000000-aaaaaaaaaaaa.jsonl");
        let mid = dir.path().join("20250601-120000-bbbbbbbbbbbb.jsonl");
        let new = dir
            .path()
            .join(session_file_name(&chrono::Utc::now(), "cccccccccccc"));
        let foreign = dir.path().join("notes.jsonl");
        for p in [&old, &mid, &new, &foreign] {
            std::fs::write(p, "{}\n").unwrap();
        }
        assert_eq!(prune_sessions(dir.path(), None, None).unwrap(), 0);
        assert_eq!(prune_sessions(dir.path(), Some(30), None).unwrap(), 2);
        assert!(!old.exists() && !mid.exists() && new.exists() && foreign.exists());

        std::fs::write(&old, "{}\n").unwrap();
        std::fs::write(&mid, "{}\n").unwrap();
        // max_sessions = 2 leaves room for the session about to start: keep 1.
        assert_eq!(prune_sessions(dir.path(), None, Some(2)).unwrap(), 2);
        assert!(new.exists() && !old.exists() && !mid.exists());
        assert!(session_started_at(&new).is_some());
        assert!(session_started_at(&foreign).is_none());
    }

    #[test]
    fn list_and_find_sessions() {
        let dir = tempfile::tempdir().unwrap();
        assert!(list_sessions(dir.path()).unwrap().is_empty());
        assert!(list_sessions(&dir.path().join("missing"))
            .unwrap()
            .is_empty());
        let a = Recorder::start(&config(), dir.path(), SessionInfo::default(), &[]).unwrap();
        a.finish("x").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1100));
        let b = Recorder::start(&config(), dir.path(), SessionInfo::default(), &[]).unwrap();
        b.finish("x").unwrap();
        let sessions = list_sessions(dir.path()).unwrap();
        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[0], b.path(), "newest first");
        assert_eq!(
            find_session(dir.path(), "latest").unwrap().unwrap(),
            b.path()
        );
        assert_eq!(
            find_session(dir.path(), a.session_id()).unwrap().unwrap(),
            a.path()
        );
        assert_eq!(
            find_session(dir.path(), a.path().to_str().unwrap())
                .unwrap()
                .unwrap(),
            a.path()
        );
        assert!(find_session(dir.path(), "nope").unwrap().is_none());
    }
}
