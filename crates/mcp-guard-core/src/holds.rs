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

//! Human-in-the-loop holds: `sudo` for agent tool calls.
//!
//! A rule with `action = "hold"` pauses the call. The proxy writes a request
//! file into the holds directory, notifies the user, and waits for a
//! matching `.approved` or `.denied` marker, which `mcp-guard approve` and
//! `mcp-guard deny` create. No marker before the timeout means deny.
//!
//! Files are the coordination mechanism on purpose: the proxy runs as a child
//! of the MCP client with no terminal, and a directory the user owns is the
//! simplest channel that works on every platform and needs no daemon.

use crate::config::HoldsConfig;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// How often the proxy checks for a decision marker.
const POLL_INTERVAL: Duration = Duration::from_millis(200);

/// A call waiting for approval.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HoldRequest {
    pub id: String,
    pub created_at: String,
    pub expires_at: String,
    pub tool: String,
    /// Redacted arguments, for display.
    pub arguments: Value,
    pub reason: String,
    pub rule: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream: Option<String>,
}

/// How a hold was resolved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldOutcome {
    Approved,
    Denied,
    TimedOut,
}

impl HoldOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            HoldOutcome::Approved => "approved",
            HoldOutcome::Denied => "denied",
            HoldOutcome::TimedOut => "timed_out",
        }
    }
}

/// Contents of a decision marker written by `approve` or `deny`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HoldDecision {
    #[serde(default)]
    pub decided_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

fn request_path(dir: &Path, id: &str) -> PathBuf {
    dir.join(format!("{}.json", id))
}

fn approved_path(dir: &Path, id: &str) -> PathBuf {
    dir.join(format!("{}.approved", id))
}

fn denied_path(dir: &Path, id: &str) -> PathBuf {
    dir.join(format!("{}.denied", id))
}

fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

/// Create a new hold request file. Returns the request.
#[allow(clippy::too_many_arguments)]
pub fn create(
    dir: &Path,
    timeout: Duration,
    tool: &str,
    arguments: Value,
    reason: &str,
    rule: &str,
    session_id: Option<&str>,
    config_path: Option<&str>,
    upstream: Option<&str>,
) -> std::io::Result<HoldRequest> {
    std::fs::create_dir_all(dir)?;
    let id = uuid::Uuid::new_v4().simple().to_string()[..8].to_string();
    let now = chrono::Utc::now();
    let request = HoldRequest {
        id: id.clone(),
        created_at: now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        expires_at: (now + chrono::Duration::from_std(timeout).unwrap_or_default())
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        tool: tool.to_string(),
        arguments,
        reason: reason.to_string(),
        rule: rule.to_string(),
        session_id: session_id.map(String::from),
        config_path: config_path.map(String::from),
        upstream: upstream.map(String::from),
    };
    // Remove stale markers that could pre-approve a brand new id (vanishingly
    // unlikely, but cheap to rule out).
    let _ = std::fs::remove_file(approved_path(dir, &id));
    let _ = std::fs::remove_file(denied_path(dir, &id));
    let json = serde_json::to_string_pretty(&request)?;
    std::fs::write(request_path(dir, &id), format!("{}\n", json))?;
    Ok(request)
}

/// Check once whether a decision marker exists.
pub fn check(dir: &Path, id: &str) -> Option<(HoldOutcome, HoldDecision)> {
    let read = |p: PathBuf| -> HoldDecision {
        std::fs::read_to_string(p)
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
            .unwrap_or_default()
    };
    let approved = approved_path(dir, id);
    if approved.exists() {
        return Some((HoldOutcome::Approved, read(approved)));
    }
    let denied = denied_path(dir, id);
    if denied.exists() {
        return Some((HoldOutcome::Denied, read(denied)));
    }
    None
}

/// Wait for a decision, polling the directory, until `timeout` elapses.
/// Cleans up the request file and marker afterwards.
pub async fn wait(dir: &Path, id: &str, timeout: Duration) -> (HoldOutcome, HoldDecision) {
    let deadline = tokio::time::Instant::now() + timeout;
    let outcome = loop {
        if let Some(found) = check(dir, id) {
            break found;
        }
        if tokio::time::Instant::now() >= deadline {
            break (HoldOutcome::TimedOut, HoldDecision::default());
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    };
    cleanup(dir, id);
    outcome
}

/// Remove the request file and any markers for a hold.
pub fn cleanup(dir: &Path, id: &str) {
    let _ = std::fs::remove_file(request_path(dir, id));
    let _ = std::fs::remove_file(approved_path(dir, id));
    let _ = std::fs::remove_file(denied_path(dir, id));
}

/// Pending requests in a directory, oldest first. Expired ones are included
/// and flagged by the caller through `is_expired`.
pub fn list(dir: &Path) -> std::io::Result<Vec<HoldRequest>> {
    let mut out = Vec::new();
    let rd = match std::fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(out),
        Err(e) => return Err(e),
    };
    for entry in rd.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().map(|x| x == "json").unwrap_or(false) {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(req) = serde_json::from_str::<HoldRequest>(&content) {
                    // Skip requests that already have a decision waiting to be consumed.
                    if check(dir, &req.id).is_none() {
                        out.push(req);
                    }
                }
            }
        }
    }
    out.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    Ok(out)
}

impl HoldRequest {
    pub fn is_expired(&self) -> bool {
        chrono::DateTime::parse_from_rfc3339(&self.expires_at)
            .map(|t| t < chrono::Utc::now())
            .unwrap_or(false)
    }
}

/// Resolve a selector (`latest`, `all`, or an id prefix) to pending requests.
pub fn select(dir: &Path, selector: &str) -> std::io::Result<Vec<HoldRequest>> {
    let pending = list(dir)?;
    Ok(match selector {
        "all" => pending,
        "latest" => pending.into_iter().last().into_iter().collect(),
        prefix => pending
            .into_iter()
            .filter(|r| r.id.starts_with(prefix))
            .collect(),
    })
}

/// Write a decision marker. `approve` or `deny` from the CLI.
pub fn decide(
    dir: &Path,
    id: &str,
    outcome: HoldOutcome,
    note: Option<&str>,
) -> std::io::Result<()> {
    if !request_path(dir, id).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("no pending hold with id {}", id),
        ));
    }
    let decision = HoldDecision {
        decided_at: now_rfc3339(),
        note: note.map(String::from),
    };
    let path = match outcome {
        HoldOutcome::Approved => approved_path(dir, id),
        HoldOutcome::Denied => denied_path(dir, id),
        HoldOutcome::TimedOut => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "cannot record a timeout by hand",
            ))
        }
    };
    std::fs::write(path, serde_json::to_string(&decision)?)
}

/// The command a person runs to approve this hold.
pub fn approve_command(request: &HoldRequest) -> String {
    match &request.config_path {
        Some(c) => format!("mcp-guard approve {} --config {}", request.id, c),
        None => format!("mcp-guard approve {}", request.id),
    }
}

/// Fill `{id}`, `{tool}`, `{reason}`, `{arguments}`, `{approve}` placeholders.
pub fn expand_placeholders(template: &str, request: &HoldRequest) -> String {
    template
        .replace("{id}", &request.id)
        .replace("{tool}", &request.tool)
        .replace("{reason}", &request.reason)
        .replace("{arguments}", &request.arguments.to_string())
        .replace("{approve}", &approve_command(request))
}

/// Notify the user that a call is waiting. Best effort: failures are logged.
pub fn notify(config: &HoldsConfig, request: &HoldRequest) {
    let argv: Vec<String> = match &config.notify_command {
        Some(cmd) if !cmd.is_empty() => cmd
            .iter()
            .map(|part| expand_placeholders(part, request))
            .collect(),
        Some(_) => return,
        None => {
            if !config.notify {
                return;
            }
            match default_notify_command(request) {
                Some(argv) => argv,
                None => return,
            }
        }
    };
    let (program, args) = match argv.split_first() {
        Some(x) => x,
        None => return,
    };
    match std::process::Command::new(program)
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(_) => tracing::debug!(program = %program, "Hold notification sent"),
        Err(e) => tracing::debug!(program = %program, error = %e, "Hold notification failed"),
    }
}

fn default_notify_command(request: &HoldRequest) -> Option<Vec<String>> {
    let title = "mcp-guard: approval needed";
    let body = format!(
        "{} wants to run {}. {}",
        request
            .upstream
            .as_deref()
            .and_then(|u| u.split_whitespace().last())
            .unwrap_or("An agent"),
        request.tool,
        approve_command(request)
    );
    match std::env::consts::OS {
        "macos" => Some(vec![
            "osascript".into(),
            "-e".into(),
            format!(
                "display notification \"{}\" with title \"{}\"",
                body.replace('"', "'"),
                title
            ),
        ]),
        "linux" => Some(vec!["notify-send".into(), title.into(), body]),
        "windows" => Some(vec![
            "powershell".into(),
            "-NoProfile".into(),
            "-Command".into(),
            format!(
                "[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms') | Out-Null; [System.Windows.Forms.MessageBox]::Show('{}', '{}') | Out-Null",
                body.replace('\'', "''"),
                title
            ),
        ]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make(dir: &Path, tool: &str) -> HoldRequest {
        create(
            dir,
            Duration::from_secs(60),
            tool,
            json!({"path": "/x"}),
            "needs a human",
            "confirm destructive",
            Some("sess1"),
            Some("/cfg/guard.toml"),
            Some("npx server"),
        )
        .unwrap()
    }

    #[test]
    fn create_list_approve_cleanup() {
        let dir = tempfile::tempdir().unwrap();
        let a = make(dir.path(), "delete_file");
        std::thread::sleep(Duration::from_millis(1100));
        let b = make(dir.path(), "run_shell");
        let pending = list(dir.path()).unwrap();
        assert_eq!(pending.len(), 2);
        assert_eq!(pending[0].id, a.id, "oldest first");
        assert!(!a.is_expired());
        assert!(check(dir.path(), &a.id).is_none());

        assert_eq!(select(dir.path(), "latest").unwrap()[0].id, b.id);
        assert_eq!(select(dir.path(), &a.id[..4]).unwrap()[0].id, a.id);
        assert_eq!(select(dir.path(), "all").unwrap().len(), 2);
        assert!(select(dir.path(), "zzzz").unwrap().is_empty());

        decide(dir.path(), &a.id, HoldOutcome::Approved, Some("looks fine")).unwrap();
        let (outcome, decision) = check(dir.path(), &a.id).unwrap();
        assert_eq!(outcome, HoldOutcome::Approved);
        assert_eq!(decision.note.as_deref(), Some("looks fine"));
        // A decided request no longer shows as pending.
        assert_eq!(list(dir.path()).unwrap().len(), 1);

        cleanup(dir.path(), &a.id);
        assert!(check(dir.path(), &a.id).is_none());
        assert!(
            decide(dir.path(), &a.id, HoldOutcome::Denied, None).is_err(),
            "gone"
        );
        assert!(decide(dir.path(), &b.id, HoldOutcome::TimedOut, None).is_err());
    }

    #[tokio::test]
    async fn wait_returns_decision_or_times_out() {
        let dir = tempfile::tempdir().unwrap();
        let req = make(dir.path(), "delete_file");
        let (outcome, _) = wait(dir.path(), &req.id, Duration::from_millis(500)).await;
        assert_eq!(outcome, HoldOutcome::TimedOut);
        assert!(list(dir.path()).unwrap().is_empty(), "request cleaned up");

        let req = make(dir.path(), "delete_file");
        let d = dir.path().to_path_buf();
        let id = req.id.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(300)).await;
            decide(&d, &id, HoldOutcome::Denied, Some("no")).unwrap();
        });
        let (outcome, decision) = wait(dir.path(), &req.id, Duration::from_secs(5)).await;
        assert_eq!(outcome, HoldOutcome::Denied);
        assert_eq!(decision.note.as_deref(), Some("no"));
        assert!(list(dir.path()).unwrap().is_empty());
    }

    #[test]
    fn placeholders_and_default_notification() {
        let dir = tempfile::tempdir().unwrap();
        let req = make(dir.path(), "delete_file");
        let s = expand_placeholders("{tool}|{reason}|{arguments}|{approve}|{id}", &req);
        assert!(s.starts_with("delete_file|needs a human|{\"path\":\"/x\"}|mcp-guard approve "));
        assert!(s.contains("--config /cfg/guard.toml"));
        assert!(s.ends_with(&req.id));
        let cmd = default_notify_command(&req);
        if cfg!(any(
            target_os = "macos",
            target_os = "linux",
            target_os = "windows"
        )) {
            let cmd = cmd.unwrap();
            assert!(cmd.iter().any(|p| p.contains("delete_file")));
            assert!(cmd.iter().any(|p| p.contains("server wants to run")));
        }
        // notify with an explicit command that cannot run must not panic.
        let cfg = HoldsConfig {
            notify_command: Some(vec!["/definitely/not/a/program".into(), "{id}".into()]),
            ..HoldsConfig::default()
        };
        notify(&cfg, &req);
        let silent = HoldsConfig {
            notify: false,
            ..HoldsConfig::default()
        };
        notify(&silent, &req);
    }
}
