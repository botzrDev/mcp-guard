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

//! MCP client configuration files: discovery, inspection, wrapping.
//!
//! Agent clients (Claude Desktop, Claude Code, Cursor, Windsurf, VS Code) each
//! keep a JSON file listing the MCP servers they launch. This module reads
//! those files, reports what is and is not protected (`doctor`), and rewrites
//! stdio entries so the client launches `mcp-guard serve` in front of the
//! original server (`wrap`), or restores them (`unwrap`).
//!
//! Nothing here touches the network. All functions take explicit paths so
//! they can be tested against temporary directories.

use crate::config::Config;
use serde::Serialize;
use serde_json::{json, Map, Value};
use std::path::{Path, PathBuf};

// ============================================================================
// Client kinds and discovery
// ============================================================================

/// A supported MCP client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClientKind {
    ClaudeDesktop,
    ClaudeCode,
    Cursor,
    Windsurf,
    VsCode,
    /// A JSON file given by path. The servers key is detected.
    Custom,
}

impl ClientKind {
    /// Stable identifier used on the command line and in paths.
    pub fn slug(&self) -> &'static str {
        match self {
            ClientKind::ClaudeDesktop => "claude-desktop",
            ClientKind::ClaudeCode => "claude-code",
            ClientKind::Cursor => "cursor",
            ClientKind::Windsurf => "windsurf",
            ClientKind::VsCode => "vscode",
            ClientKind::Custom => "custom",
        }
    }

    /// Human-readable name.
    pub fn display_name(&self) -> &'static str {
        match self {
            ClientKind::ClaudeDesktop => "Claude Desktop",
            ClientKind::ClaudeCode => "Claude Code",
            ClientKind::Cursor => "Cursor",
            ClientKind::Windsurf => "Windsurf",
            ClientKind::VsCode => "VS Code",
            ClientKind::Custom => "custom file",
        }
    }

    /// Parse a command-line client name.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().replace('_', "-").as_str() {
            "claude-desktop" | "claude" | "desktop" => Some(ClientKind::ClaudeDesktop),
            "claude-code" | "code" => Some(ClientKind::ClaudeCode),
            "cursor" => Some(ClientKind::Cursor),
            "windsurf" | "codeium" => Some(ClientKind::Windsurf),
            "vscode" | "vs-code" => Some(ClientKind::VsCode),
            _ => None,
        }
    }

    /// All kinds that have well-known file locations.
    pub fn all() -> &'static [ClientKind] {
        &[
            ClientKind::ClaudeDesktop,
            ClientKind::ClaudeCode,
            ClientKind::Cursor,
            ClientKind::Windsurf,
            ClientKind::VsCode,
        ]
    }
}

/// Where to look for client configuration files.
#[derive(Debug, Clone)]
pub struct DiscoveryEnv {
    pub home: PathBuf,
    pub cwd: PathBuf,
    /// `%APPDATA%` on Windows.
    pub appdata: Option<PathBuf>,
    /// `std::env::consts::OS` ("macos", "linux", "windows").
    pub os: String,
}

impl DiscoveryEnv {
    /// Read the environment of the current process.
    pub fn from_system() -> Self {
        let home = std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let appdata = std::env::var_os("APPDATA").map(PathBuf::from);
        Self {
            home,
            cwd,
            appdata,
            os: std::env::consts::OS.to_string(),
        }
    }

    /// Candidate configuration files for a client, whether or not they exist.
    pub fn candidate_paths(&self, kind: ClientKind) -> Vec<PathBuf> {
        match kind {
            ClientKind::ClaudeDesktop => {
                let file = "claude_desktop_config.json";
                match self.os.as_str() {
                    "macos" => vec![self
                        .home
                        .join("Library/Application Support/Claude")
                        .join(file)],
                    "windows" => self
                        .appdata
                        .as_ref()
                        .map(|a| vec![a.join("Claude").join(file)])
                        .unwrap_or_default(),
                    _ => vec![self.home.join(".config/Claude").join(file)],
                }
            }
            ClientKind::ClaudeCode => {
                vec![self.cwd.join(".mcp.json"), self.home.join(".claude.json")]
            }
            ClientKind::Cursor => vec![
                self.cwd.join(".cursor/mcp.json"),
                self.home.join(".cursor/mcp.json"),
            ],
            ClientKind::Windsurf => vec![self.home.join(".codeium/windsurf/mcp_config.json")],
            ClientKind::VsCode => vec![self.cwd.join(".vscode/mcp.json")],
            ClientKind::Custom => vec![],
        }
    }

    /// Existing configuration files for the given clients.
    pub fn discover(&self, kinds: &[ClientKind]) -> Vec<ClientConfigFile> {
        let mut out = Vec::new();
        for kind in kinds {
            for path in self.candidate_paths(*kind) {
                if path.is_file() {
                    out.push(ClientConfigFile { kind: *kind, path });
                }
            }
        }
        out
    }

    /// Resolve a `--client` argument: a known client name or a file path.
    pub fn resolve(&self, spec: &str) -> Result<Vec<ClientConfigFile>, ClientError> {
        if let Some(kind) = ClientKind::parse(spec) {
            let found = self.discover(&[kind]);
            if found.is_empty() {
                let looked = self
                    .candidate_paths(kind)
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(ClientError::NotFound(format!(
                    "no {} configuration found (looked in: {})",
                    kind.display_name(),
                    looked
                )));
            }
            return Ok(found);
        }
        let path = PathBuf::from(spec);
        if path.is_file() {
            return Ok(vec![ClientConfigFile {
                kind: ClientKind::Custom,
                path,
            }]);
        }
        Err(ClientError::NotFound(format!(
            "'{}' is not a known client (claude-desktop, claude-code, cursor, windsurf, vscode) or an existing file",
            spec
        )))
    }
}

/// A client configuration file on disk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClientConfigFile {
    pub kind: ClientKind,
    pub path: PathBuf,
}

/// Errors from reading or rewriting client configuration.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("{0}")]
    NotFound(String),
    #[error("failed to read {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("{path} is not valid JSON: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path} has no MCP server list (expected a \"mcpServers\" or \"servers\" object)")]
    NoServers { path: PathBuf },
    #[error("failed to write {path}: {source}")]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("{0}")]
    Invalid(String),
}

// ============================================================================
// Reading server entries
// ============================================================================

/// The key under which a client file lists its servers.
pub fn servers_key(kind: ClientKind, root: &Value) -> Option<&'static str> {
    let has = |k: &str| root.get(k).map(|v| v.is_object()).unwrap_or(false);
    match kind {
        ClientKind::VsCode => {
            if has("servers") {
                Some("servers")
            } else if has("mcpServers") {
                Some("mcpServers")
            } else {
                None
            }
        }
        _ => {
            if has("mcpServers") {
                Some("mcpServers")
            } else if has("servers") {
                Some("servers")
            } else {
                None
            }
        }
    }
}

/// One server entry as the client sees it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ServerEntry {
    pub name: String,
    pub raw: Value,
}

impl ServerEntry {
    pub fn command(&self) -> Option<&str> {
        self.raw.get("command").and_then(|c| c.as_str())
    }

    pub fn args(&self) -> Vec<String> {
        self.raw
            .get("args")
            .and_then(|a| a.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn url(&self) -> Option<&str> {
        self.raw.get("url").and_then(|u| u.as_str())
    }

    pub fn env(&self) -> Option<&Map<String, Value>> {
        self.raw.get("env").and_then(|e| e.as_object())
    }

    pub fn headers(&self) -> Option<&Map<String, Value>> {
        self.raw.get("headers").and_then(|e| e.as_object())
    }

    /// Declared transport type, if the client records one.
    pub fn declared_type(&self) -> Option<&str> {
        self.raw.get("type").and_then(|t| t.as_str())
    }

    /// Remote (HTTP or SSE) servers are reached by URL, not launched.
    pub fn is_remote(&self) -> bool {
        self.url().is_some()
            || matches!(
                self.declared_type(),
                Some("http") | Some("sse") | Some("streamable-http") | Some("streamable_http")
            )
    }

    /// Stdio servers are launched as subprocesses.
    pub fn is_stdio(&self) -> bool {
        !self.is_remote() && self.command().is_some()
    }

    /// Whether the entry already launches mcp-guard in serve mode.
    pub fn is_wrapped(&self) -> bool {
        let Some(command) = self.command() else {
            return false;
        };
        let base = Path::new(command)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or(command);
        let looks_like_guard = base.starts_with("mcp-guard");
        looks_like_guard && self.args().iter().any(|a| a == "serve")
    }

    /// The mcp-guard config path of a wrapped entry.
    pub fn wrapped_config_path(&self) -> Option<PathBuf> {
        if !self.is_wrapped() {
            return None;
        }
        let args = self.args();
        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            if arg == "--config" || arg == "-c" {
                return iter.next().map(PathBuf::from);
            }
            if let Some(rest) = arg.strip_prefix("--config=") {
                return Some(PathBuf::from(rest));
            }
        }
        None
    }

    /// Short description of what the entry launches or connects to.
    pub fn summary(&self) -> String {
        if let Some(url) = self.url() {
            return url.to_string();
        }
        match self.command() {
            Some(cmd) => {
                let mut parts = vec![cmd.to_string()];
                parts.extend(self.args());
                parts.join(" ")
            }
            None => "(no command or url)".to_string(),
        }
    }
}

/// Parsed client file: the whole JSON document plus its server entries.
#[derive(Debug, Clone)]
pub struct LoadedClientConfig {
    pub file: ClientConfigFile,
    pub root: Value,
    pub servers_key: &'static str,
}

impl LoadedClientConfig {
    /// Read and parse a client file.
    pub fn load(file: &ClientConfigFile) -> Result<Self, ClientError> {
        let content = std::fs::read_to_string(&file.path).map_err(|source| ClientError::Read {
            path: file.path.clone(),
            source,
        })?;
        let root: Value = serde_json::from_str(&content).map_err(|source| ClientError::Parse {
            path: file.path.clone(),
            source,
        })?;
        let key = servers_key(file.kind, &root).ok_or_else(|| ClientError::NoServers {
            path: file.path.clone(),
        })?;
        Ok(Self {
            file: file.clone(),
            root,
            servers_key: key,
        })
    }

    /// Server entries in file order (or sorted, depending on the JSON parser).
    pub fn servers(&self) -> Vec<ServerEntry> {
        self.root
            .get(self.servers_key)
            .and_then(|s| s.as_object())
            .map(|map| {
                map.iter()
                    .map(|(name, raw)| ServerEntry {
                        name: name.clone(),
                        raw: raw.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn servers_mut(&mut self) -> &mut Map<String, Value> {
        self.root
            .get_mut(self.servers_key)
            .and_then(|s| s.as_object_mut())
            .expect("servers key validated at load")
    }

    /// Write the document back, after saving a timestamped backup of the original.
    pub fn save_with_backup(&self) -> Result<PathBuf, ClientError> {
        let stamp = chrono::Utc::now().format("%Y%m%d-%H%M%S");
        let backup = PathBuf::from(format!("{}.bak-{}", self.file.path.display(), stamp));
        std::fs::copy(&self.file.path, &backup).map_err(|source| ClientError::Write {
            path: backup.clone(),
            source,
        })?;
        let json = serde_json::to_string_pretty(&self.root)
            .map_err(|e| ClientError::Invalid(e.to_string()))?;
        std::fs::write(&self.file.path, format!("{}\n", json)).map_err(|source| {
            ClientError::Write {
                path: self.file.path.clone(),
                source,
            }
        })?;
        Ok(backup)
    }
}

// ============================================================================
// Wrapping
// ============================================================================

/// Options for `wrap`.
#[derive(Debug, Clone)]
pub struct WrapOptions {
    /// Path of the mcp-guard binary the client should launch.
    pub guard_bin: PathBuf,
    /// Directory where per-server guard configs and audit logs are written.
    pub config_dir: PathBuf,
    /// Only wrap these server names (all stdio servers when empty).
    pub only: Vec<String>,
    pub allowed_tools: Vec<String>,
    pub denied_tools: Vec<String>,
    pub pin_tools: bool,
}

/// What `wrap` will do to one server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum WrapAction {
    Wrap {
        name: String,
        guard_config_path: PathBuf,
        guard_config_toml: String,
        audit_log_path: PathBuf,
        new_entry: Value,
    },
    Skip {
        name: String,
        reason: String,
    },
}

impl WrapAction {
    pub fn name(&self) -> &str {
        match self {
            WrapAction::Wrap { name, .. } | WrapAction::Skip { name, .. } => name,
        }
    }
}

/// Sanitize a server name for use in a file name.
fn file_safe(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if cleaned.is_empty() {
        "server".to_string()
    } else {
        cleaned
    }
}

/// Render the per-server guard configuration.
pub fn render_guard_config(
    client: ClientKind,
    server_name: &str,
    command: &str,
    args: &[String],
    audit_log: &Path,
    opts: &WrapOptions,
) -> String {
    let mut upstream = toml::Table::new();
    upstream.insert("transport".into(), toml::Value::String("stdio".into()));
    upstream.insert("command".into(), toml::Value::String(command.to_string()));
    upstream.insert(
        "args".into(),
        toml::Value::Array(
            args.iter()
                .map(|a| toml::Value::String(a.clone()))
                .collect(),
        ),
    );

    let mut audit = toml::Table::new();
    audit.insert("enabled".into(), toml::Value::Boolean(true));
    audit.insert(
        "file".into(),
        toml::Value::String(audit_log.to_string_lossy().into_owned()),
    );
    audit.insert("stdout".into(), toml::Value::Boolean(false));

    let mut policy = toml::Table::new();
    policy.insert(
        "allowed_tools".into(),
        toml::Value::Array(
            opts.allowed_tools
                .iter()
                .map(|a| toml::Value::String(a.clone()))
                .collect(),
        ),
    );
    policy.insert(
        "denied_tools".into(),
        toml::Value::Array(
            opts.denied_tools
                .iter()
                .map(|a| toml::Value::String(a.clone()))
                .collect(),
        ),
    );
    policy.insert("pin_tools".into(), toml::Value::Boolean(opts.pin_tools));
    policy.insert("on_pin_mismatch".into(), toml::Value::String("deny".into()));

    let mut root = toml::Table::new();
    root.insert("upstream".into(), toml::Value::Table(upstream));
    root.insert("audit".into(), toml::Value::Table(audit));
    root.insert("policy".into(), toml::Value::Table(policy));

    let body = toml::to_string_pretty(&root).unwrap_or_default();
    format!(
        "# Generated by `mcp-guard wrap` for {} server \"{}\".\n\
         # mcp-guard runs this server as a subprocess and sits between it and the agent.\n\
         # Edit [policy] to restrict tools; run `mcp-guard pin --config <this file>` after\n\
         # reviewing a tool change. See docs/local-mode.md.\n\n{}",
        client.display_name(),
        server_name,
        body
    )
}

/// Decide what to do with each server. Pure: writes nothing.
pub fn plan_wrap(loaded: &LoadedClientConfig, opts: &WrapOptions) -> Vec<WrapAction> {
    let client_dir = opts.config_dir.join(loaded.file.kind.slug());
    let logs_dir = opts.config_dir.join("logs");
    let mut actions = Vec::new();

    for entry in loaded.servers() {
        if !opts.only.is_empty() && !opts.only.iter().any(|n| n == &entry.name) {
            continue;
        }
        if entry.is_wrapped() {
            actions.push(WrapAction::Skip {
                name: entry.name,
                reason: "already wrapped by mcp-guard".into(),
            });
            continue;
        }
        if entry.is_remote() {
            actions.push(WrapAction::Skip {
                name: entry.name,
                reason: "remote server (url); wrap applies to stdio servers. Put `mcp-guard run` in front of it instead".into(),
            });
            continue;
        }
        let Some(command) = entry.command() else {
            actions.push(WrapAction::Skip {
                name: entry.name,
                reason: "entry has neither command nor url".into(),
            });
            continue;
        };
        let args = entry.args();
        if let Err(e) = crate::transport::validate_command_for_injection(command) {
            actions.push(WrapAction::Skip {
                name: entry.name,
                reason: format!("command rejected by mcp-guard's injection check: {}", e),
            });
            continue;
        }
        if let Err(e) = crate::transport::validate_args_for_injection(&args) {
            actions.push(WrapAction::Skip {
                name: entry.name,
                reason: format!("arguments rejected by mcp-guard's injection check: {}", e),
            });
            continue;
        }

        let safe = file_safe(&entry.name);
        let guard_config_path = client_dir.join(format!("{}.toml", safe));
        let audit_log_path =
            logs_dir.join(format!("{}-{}.audit.jsonl", loaded.file.kind.slug(), safe));
        let toml_text = render_guard_config(
            loaded.file.kind,
            &entry.name,
            command,
            &args,
            &audit_log_path,
            opts,
        );

        let mut new_entry = Map::new();
        if let Some(t) = entry.declared_type() {
            new_entry.insert("type".into(), json!(t));
        }
        new_entry.insert(
            "command".into(),
            json!(opts.guard_bin.to_string_lossy().into_owned()),
        );
        new_entry.insert(
            "args".into(),
            json!([
                "--config",
                guard_config_path.to_string_lossy().into_owned(),
                "serve"
            ]),
        );
        // Environment stays on the client entry: mcp-guard inherits it and
        // passes it to the upstream process, so secrets never land in the
        // generated guard config.
        if let Some(env) = entry.env() {
            new_entry.insert("env".into(), Value::Object(env.clone()));
        }
        for passthrough in ["cwd", "disabled", "timeout", "alwaysAllow", "autoApprove"] {
            if let Some(v) = entry.raw.get(passthrough) {
                new_entry.insert(passthrough.into(), v.clone());
            }
        }

        actions.push(WrapAction::Wrap {
            name: entry.name,
            guard_config_path,
            guard_config_toml: toml_text,
            audit_log_path,
            new_entry: Value::Object(new_entry),
        });
    }
    actions
}

/// Outcome of `apply_wrap`.
#[derive(Debug, Clone, Serialize)]
pub struct WrapOutcome {
    pub backup_path: PathBuf,
    pub wrapped: Vec<String>,
}

/// Write guard configs and rewrite the client file. Returns the backup path.
pub fn apply_wrap(
    loaded: &mut LoadedClientConfig,
    actions: &[WrapAction],
) -> Result<WrapOutcome, ClientError> {
    let mut wrapped = Vec::new();
    for action in actions {
        if let WrapAction::Wrap {
            name,
            guard_config_path,
            guard_config_toml,
            audit_log_path,
            new_entry,
        } = action
        {
            if let Some(dir) = guard_config_path.parent() {
                std::fs::create_dir_all(dir).map_err(|source| ClientError::Write {
                    path: dir.to_path_buf(),
                    source,
                })?;
            }
            if let Some(dir) = audit_log_path.parent() {
                std::fs::create_dir_all(dir).map_err(|source| ClientError::Write {
                    path: dir.to_path_buf(),
                    source,
                })?;
            }
            std::fs::write(guard_config_path, guard_config_toml).map_err(|source| {
                ClientError::Write {
                    path: guard_config_path.clone(),
                    source,
                }
            })?;
            loaded.servers_mut().insert(name.clone(), new_entry.clone());
            wrapped.push(name.clone());
        }
    }
    if wrapped.is_empty() {
        return Err(ClientError::Invalid("nothing to wrap".into()));
    }
    let backup_path = loaded.save_with_backup()?;
    Ok(WrapOutcome {
        backup_path,
        wrapped,
    })
}

// ============================================================================
// Unwrapping
// ============================================================================

/// What `unwrap` will do to one server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum UnwrapAction {
    Unwrap {
        name: String,
        guard_config_path: PathBuf,
        restored_entry: Value,
    },
    Skip {
        name: String,
        reason: String,
    },
}

/// Decide how to restore each wrapped server from its guard config.
pub fn plan_unwrap(loaded: &LoadedClientConfig, only: &[String]) -> Vec<UnwrapAction> {
    let mut actions = Vec::new();
    for entry in loaded.servers() {
        if !only.is_empty() && !only.iter().any(|n| n == &entry.name) {
            continue;
        }
        if !entry.is_wrapped() {
            actions.push(UnwrapAction::Skip {
                name: entry.name,
                reason: "not wrapped by mcp-guard".into(),
            });
            continue;
        }
        let Some(config_path) = entry.wrapped_config_path() else {
            actions.push(UnwrapAction::Skip {
                name: entry.name,
                reason: "wrapped entry has no --config argument".into(),
            });
            continue;
        };
        let config = match Config::from_file(&config_path) {
            Ok(c) => c,
            Err(e) => {
                actions.push(UnwrapAction::Skip {
                    name: entry.name,
                    reason: format!("cannot read {}: {}", config_path.display(), e),
                });
                continue;
            }
        };
        let Some(command) = config.upstream.command.clone() else {
            actions.push(UnwrapAction::Skip {
                name: entry.name,
                reason: format!("{} has no upstream command", config_path.display()),
            });
            continue;
        };

        let mut restored = Map::new();
        if let Some(t) = entry.declared_type() {
            restored.insert("type".into(), json!(t));
        }
        restored.insert("command".into(), json!(command));
        restored.insert("args".into(), json!(config.upstream.args));
        if let Some(env) = entry.env() {
            restored.insert("env".into(), Value::Object(env.clone()));
        }
        for passthrough in ["cwd", "disabled", "timeout", "alwaysAllow", "autoApprove"] {
            if let Some(v) = entry.raw.get(passthrough) {
                restored.insert(passthrough.into(), v.clone());
            }
        }
        actions.push(UnwrapAction::Unwrap {
            name: entry.name,
            guard_config_path: config_path,
            restored_entry: Value::Object(restored),
        });
    }
    actions
}

/// Rewrite the client file with the restored entries. Guard configs are kept.
pub fn apply_unwrap(
    loaded: &mut LoadedClientConfig,
    actions: &[UnwrapAction],
) -> Result<WrapOutcome, ClientError> {
    let mut restored_names = Vec::new();
    for action in actions {
        if let UnwrapAction::Unwrap {
            name,
            restored_entry,
            ..
        } = action
        {
            loaded
                .servers_mut()
                .insert(name.clone(), restored_entry.clone());
            restored_names.push(name.clone());
        }
    }
    if restored_names.is_empty() {
        return Err(ClientError::Invalid("nothing to unwrap".into()));
    }
    let backup_path = loaded.save_with_backup()?;
    Ok(WrapOutcome {
        backup_path,
        wrapped: restored_names,
    })
}

// ============================================================================
// Doctor
// ============================================================================

/// How much a finding matters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warn,
    High,
}

impl Severity {
    pub fn label(&self) -> &'static str {
        match self {
            Severity::Info => "INFO",
            Severity::Warn => "WARN",
            Severity::High => "HIGH",
        }
    }
}

/// One thing `doctor` noticed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
    pub severity: Severity,
    pub client: ClientKind,
    pub file: PathBuf,
    pub server: Option<String>,
    /// Stable machine-readable code.
    pub code: &'static str,
    pub message: String,
    pub fix: Option<String>,
}

/// Result of a `doctor` scan.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DoctorReport {
    pub files_scanned: Vec<ClientConfigFile>,
    pub servers_seen: usize,
    pub guarded: usize,
    pub findings: Vec<Finding>,
}

impl DoctorReport {
    pub fn count(&self, severity: Severity) -> usize {
        self.findings
            .iter()
            .filter(|f| f.severity == severity)
            .count()
    }
}

fn looks_like_secret(key: &str) -> bool {
    let k = key.to_ascii_uppercase();
    [
        "TOKEN",
        "SECRET",
        "PASSWORD",
        "PASSWD",
        "API_KEY",
        "APIKEY",
        "PRIVATE_KEY",
    ]
    .iter()
    .any(|needle| k.contains(needle))
}

fn is_loopback_url(url: &str) -> bool {
    url.contains("://localhost") || url.contains("://127.0.0.1") || url.contains("://[::1]")
}

/// Inspect client files and report unprotected or misconfigured servers.
pub fn doctor(files: &[ClientConfigFile]) -> DoctorReport {
    let mut report = DoctorReport {
        files_scanned: files.to_vec(),
        ..Default::default()
    };

    for file in files {
        let loaded = match LoadedClientConfig::load(file) {
            Ok(l) => l,
            Err(ClientError::NoServers { .. }) => continue,
            Err(e) => {
                report.findings.push(Finding {
                    severity: Severity::High,
                    client: file.kind,
                    file: file.path.clone(),
                    server: None,
                    code: "CLIENT_CONFIG_UNREADABLE",
                    message: e.to_string(),
                    fix: None,
                });
                continue;
            }
        };

        for entry in loaded.servers() {
            report.servers_seen += 1;
            let finding = |severity, code, message: String, fix: Option<String>| Finding {
                severity,
                client: file.kind,
                file: file.path.clone(),
                server: Some(entry.name.clone()),
                code,
                message,
                fix,
            };

            if let Some(env) = entry.env() {
                let secret_keys: Vec<&String> = env
                    .iter()
                    .filter(|(k, v)| {
                        looks_like_secret(k) && v.as_str().map(|s| !s.is_empty()).unwrap_or(false)
                    })
                    .map(|(k, _)| k)
                    .collect();
                if !secret_keys.is_empty() {
                    report.findings.push(finding(
                        Severity::Info,
                        "SECRET_IN_CLIENT_CONFIG",
                        format!(
                            "environment holds credentials in plaintext ({}); any process reading this file can use them",
                            secret_keys.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
                        ),
                        None,
                    ));
                }
            }

            if entry.is_remote() {
                let url = entry.url().unwrap_or("");
                if url.starts_with("http://") && !is_loopback_url(url) {
                    report.findings.push(finding(
                        Severity::High,
                        "REMOTE_PLAINTEXT",
                        format!("remote server {} is reached over plain HTTP; tokens and tool data cross the network unencrypted", url),
                        Some("use an https:// URL".into()),
                    ));
                }
                let has_auth_header = entry
                    .headers()
                    .map(|h| h.keys().any(|k| k.eq_ignore_ascii_case("authorization")))
                    .unwrap_or(false);
                if !has_auth_header {
                    report.findings.push(finding(
                        Severity::Warn,
                        "REMOTE_NO_STATIC_AUTH",
                        format!("remote server {} has no Authorization header in the client config; make sure the server enforces OAuth, or it is open to anyone who finds the URL", entry.summary()),
                        Some("put `mcp-guard run` in front of the server, or confirm the server requires OAuth".into()),
                    ));
                }
                continue;
            }

            if !entry.is_stdio() {
                report.findings.push(finding(
                    Severity::Warn,
                    "ENTRY_MALFORMED",
                    "entry has neither a command nor a url".into(),
                    None,
                ));
                continue;
            }

            if !entry.is_wrapped() {
                report.findings.push(finding(
                    Severity::Warn,
                    "UNGUARDED_STDIO",
                    format!(
                        "`{}` runs with full access to every tool it offers, no audit log, and no pinning of its tool definitions",
                        entry.summary()
                    ),
                    Some(format!(
                        "mcp-guard wrap --client {} --server \"{}\"",
                        match file.kind {
                            ClientKind::Custom => file.path.display().to_string(),
                            k => k.slug().to_string(),
                        },
                        entry.name
                    )),
                ));
                continue;
            }

            report.guarded += 1;
            let Some(config_path) = entry.wrapped_config_path() else {
                report.findings.push(finding(
                    Severity::High,
                    "GUARD_CONFIG_MISSING",
                    "wrapped entry has no --config argument, so mcp-guard will fail to start"
                        .into(),
                    None,
                ));
                continue;
            };
            let config = match Config::from_file(&config_path) {
                Ok(c) => c,
                Err(e) => {
                    report.findings.push(finding(
                        Severity::High,
                        "GUARD_CONFIG_MISSING",
                        format!(
                            "guard config {} cannot be loaded: {}",
                            config_path.display(),
                            e
                        ),
                        Some(format!(
                            "mcp-guard unwrap --client {} --server \"{}\" and wrap again",
                            file.kind.slug(),
                            entry.name
                        )),
                    ));
                    continue;
                }
            };
            if config.policy.denied_tools.is_empty()
                && config.policy.allowed_tools.iter().any(|t| t == "*")
            {
                report.findings.push(finding(
                    Severity::Info,
                    "POLICY_ALLOW_ALL",
                    "policy allows every tool; the guard audits and pins but does not restrict"
                        .into(),
                    Some(format!("edit [policy] in {}", config_path.display())),
                ));
            }
            if !config.policy.pin_tools {
                report.findings.push(finding(
                    Severity::Warn,
                    "PINNING_DISABLED",
                    "tool pinning is off; a server can rewrite its tool descriptions without notice".into(),
                    Some(format!("set pin_tools = true in {}", config_path.display())),
                ));
            } else {
                let lock = config.policy.lock_path_for(&config_path);
                if !lock.exists() {
                    report.findings.push(finding(
                        Severity::Info,
                        "NOT_YET_PINNED",
                        "tool definitions will be pinned on first use (trust on first use)".into(),
                        Some(format!(
                            "mcp-guard pin --config {} to pin now and review",
                            config_path.display()
                        )),
                    ));
                }
            }
            if !config.audit.enabled {
                report.findings.push(finding(
                    Severity::Warn,
                    "AUDIT_DISABLED",
                    "audit logging is disabled for this server".into(),
                    Some(format!(
                        "set [audit] enabled = true in {}",
                        config_path.display()
                    )),
                ));
            }
        }
    }

    report
        .findings
        .sort_by_key(|f| std::cmp::Reverse(f.severity));
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(dir: &Path) -> DiscoveryEnv {
        DiscoveryEnv {
            home: dir.join("home"),
            cwd: dir.join("project"),
            appdata: Some(dir.join("appdata")),
            os: "linux".into(),
        }
    }

    fn write(path: &Path, content: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    const SAMPLE: &str = r#"{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"],
      "env": {"GITHUB_TOKEN": "ghp_secret"}
    },
    "remote": {"url": "http://example.com/mcp"},
    "shelly": {"command": "bash", "args": ["-c", "evil"]}
  },
  "theme": "dark"
}"#;

    #[test]
    fn parses_client_names() {
        assert_eq!(
            ClientKind::parse("Claude-Desktop"),
            Some(ClientKind::ClaudeDesktop)
        );
        assert_eq!(
            ClientKind::parse("claude_code"),
            Some(ClientKind::ClaudeCode)
        );
        assert_eq!(ClientKind::parse("vscode"), Some(ClientKind::VsCode));
        assert_eq!(ClientKind::parse("emacs"), None);
    }

    #[test]
    fn discovery_uses_os_specific_paths() {
        let dir = tempfile::tempdir().unwrap();
        let mut e = env(dir.path());
        let linux = e.candidate_paths(ClientKind::ClaudeDesktop);
        assert!(linux[0].ends_with(".config/Claude/claude_desktop_config.json"));
        e.os = "macos".into();
        let mac = e.candidate_paths(ClientKind::ClaudeDesktop);
        assert!(mac[0]
            .to_string_lossy()
            .contains("Library/Application Support/Claude"));
        e.os = "windows".into();
        let win = e.candidate_paths(ClientKind::ClaudeDesktop);
        assert!(win[0].starts_with(dir.path().join("appdata")));
    }

    #[test]
    fn discover_only_returns_existing_files() {
        let dir = tempfile::tempdir().unwrap();
        let e = env(dir.path());
        assert!(e.discover(ClientKind::all()).is_empty());
        write(&e.cwd.join(".cursor/mcp.json"), SAMPLE);
        write(&e.home.join(".claude.json"), SAMPLE);
        let found = e.discover(ClientKind::all());
        assert_eq!(found.len(), 2);
        assert!(found.iter().any(|f| f.kind == ClientKind::Cursor));
        assert!(found.iter().any(|f| f.kind == ClientKind::ClaudeCode));
        assert!(e.resolve("cursor").is_ok());
        assert!(e.resolve("windsurf").is_err());
        assert!(e
            .resolve(e.home.join(".claude.json").to_str().unwrap())
            .is_ok());
        assert!(e.resolve("nonsense").is_err());
    }

    #[test]
    fn server_entry_classification() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("cfg.json");
        write(&path, SAMPLE);
        let loaded = LoadedClientConfig::load(&ClientConfigFile {
            kind: ClientKind::Custom,
            path,
        })
        .unwrap();
        let servers = loaded.servers();
        let fs = servers.iter().find(|s| s.name == "filesystem").unwrap();
        assert!(fs.is_stdio() && !fs.is_remote() && !fs.is_wrapped());
        assert_eq!(fs.args().len(), 3);
        let remote = servers.iter().find(|s| s.name == "remote").unwrap();
        assert!(remote.is_remote());
        assert_eq!(remote.summary(), "http://example.com/mcp");
    }

    #[test]
    fn vscode_uses_servers_key() {
        let root: Value = serde_json::from_str(r#"{"servers": {"a": {"command": "x"}}}"#).unwrap();
        assert_eq!(servers_key(ClientKind::VsCode, &root), Some("servers"));
        assert_eq!(servers_key(ClientKind::Cursor, &root), Some("servers"));
        let none: Value = serde_json::from_str(r#"{"other": 1}"#).unwrap();
        assert_eq!(servers_key(ClientKind::Cursor, &none), None);
    }

    fn wrap_opts(dir: &Path) -> WrapOptions {
        WrapOptions {
            guard_bin: PathBuf::from("/usr/local/bin/mcp-guard"),
            config_dir: dir.join("guard"),
            only: vec![],
            allowed_tools: vec!["*".into()],
            denied_tools: vec!["delete_*".into()],
            pin_tools: true,
        }
    }

    #[test]
    fn wrap_plan_wraps_stdio_and_skips_the_rest() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("claude_desktop_config.json");
        write(&path, SAMPLE);
        let loaded = LoadedClientConfig::load(&ClientConfigFile {
            kind: ClientKind::ClaudeDesktop,
            path,
        })
        .unwrap();
        let actions = plan_wrap(&loaded, &wrap_opts(dir.path()));
        assert_eq!(actions.len(), 3);
        let fs = actions.iter().find(|a| a.name() == "filesystem").unwrap();
        match fs {
            WrapAction::Wrap {
                guard_config_path,
                guard_config_toml,
                new_entry,
                ..
            } => {
                assert!(guard_config_path.ends_with("claude-desktop/filesystem.toml"));
                assert!(guard_config_toml.contains("command = \"npx\""));
                assert!(guard_config_toml.contains("denied_tools = [\"delete_*\"]"));
                assert!(
                    !guard_config_toml.contains("ghp_secret"),
                    "secrets must stay out of guard config"
                );
                assert_eq!(new_entry["command"], "/usr/local/bin/mcp-guard");
                assert_eq!(new_entry["args"][0], "--config");
                assert_eq!(new_entry["args"][2], "serve");
                assert_eq!(new_entry["env"]["GITHUB_TOKEN"], "ghp_secret");
            }
            other => panic!("expected wrap, got {:?}", other),
        }
        assert!(matches!(
            actions.iter().find(|a| a.name() == "remote").unwrap(),
            WrapAction::Skip { .. }
        ));
        assert!(matches!(
            actions.iter().find(|a| a.name() == "shelly").unwrap(),
            WrapAction::Skip { reason, .. } if reason.contains("injection")
        ));
    }

    #[test]
    fn wrap_then_unwrap_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("mcp.json");
        write(&path, SAMPLE);
        let file = ClientConfigFile {
            kind: ClientKind::Cursor,
            path: path.clone(),
        };
        let mut loaded = LoadedClientConfig::load(&file).unwrap();
        let mut opts = wrap_opts(dir.path());
        opts.only = vec!["filesystem".into()];
        let actions = plan_wrap(&loaded, &opts);
        assert_eq!(actions.len(), 1);
        let outcome = apply_wrap(&mut loaded, &actions).unwrap();
        assert!(outcome.backup_path.exists());
        assert_eq!(outcome.wrapped, vec!["filesystem"]);

        // Re-read from disk: wrapped, config exists, other keys preserved.
        let reloaded = LoadedClientConfig::load(&file).unwrap();
        assert_eq!(reloaded.root["theme"], "dark");
        let fs = reloaded
            .servers()
            .into_iter()
            .find(|s| s.name == "filesystem")
            .unwrap();
        assert!(fs.is_wrapped());
        let cfg_path = fs.wrapped_config_path().unwrap();
        assert!(cfg_path.exists());
        let cfg = Config::from_file(&cfg_path).unwrap();
        assert_eq!(cfg.upstream.command.as_deref(), Some("npx"));
        assert_eq!(cfg.policy.denied_tools, vec!["delete_*"]);
        assert!(cfg.policy.pin_tools);

        // Doctor sees one guarded server and no UNGUARDED finding for it.
        let report = doctor(std::slice::from_ref(&file));
        assert_eq!(report.guarded, 1);
        assert!(!report
            .findings
            .iter()
            .any(|f| f.code == "UNGUARDED_STDIO" && f.server.as_deref() == Some("filesystem")));
        assert!(report
            .findings
            .iter()
            .any(|f| f.code == "NOT_YET_PINNED" && f.server.as_deref() == Some("filesystem")));

        // Unwrap restores the original launch.
        let mut reloaded = reloaded;
        let unwrap_actions = plan_unwrap(&reloaded, &[]);
        assert!(unwrap_actions
            .iter()
            .any(|a| matches!(a, UnwrapAction::Unwrap { name, .. } if name == "filesystem")));
        apply_unwrap(&mut reloaded, &unwrap_actions).unwrap();
        let restored = LoadedClientConfig::load(&file).unwrap();
        let fs = restored
            .servers()
            .into_iter()
            .find(|s| s.name == "filesystem")
            .unwrap();
        assert!(!fs.is_wrapped());
        assert_eq!(fs.command(), Some("npx"));
        assert_eq!(fs.args()[1], "@modelcontextprotocol/server-filesystem");
        assert_eq!(fs.env().unwrap()["GITHUB_TOKEN"], "ghp_secret");
    }

    #[test]
    fn doctor_flags_unguarded_remote_and_secrets() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("mcp.json");
        write(&path, SAMPLE);
        let file = ClientConfigFile {
            kind: ClientKind::ClaudeCode,
            path: path.clone(),
        };
        let report = doctor(&[file]);
        assert_eq!(report.servers_seen, 3);
        assert_eq!(report.guarded, 0);
        let codes: Vec<&str> = report.findings.iter().map(|f| f.code).collect();
        assert!(codes.contains(&"UNGUARDED_STDIO"));
        assert!(codes.contains(&"REMOTE_PLAINTEXT"));
        assert!(codes.contains(&"REMOTE_NO_STATIC_AUTH"));
        assert!(codes.contains(&"SECRET_IN_CLIENT_CONFIG"));
        // Sorted most severe first.
        assert_eq!(report.findings[0].severity, Severity::High);
        let fix = report
            .findings
            .iter()
            .find(|f| f.code == "UNGUARDED_STDIO" && f.server.as_deref() == Some("filesystem"))
            .and_then(|f| f.fix.clone())
            .unwrap();
        assert!(fix.contains("--client claude-code"));
    }

    #[test]
    fn doctor_reports_unreadable_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("broken.json");
        write(&path, "{ not json");
        let report = doctor(&[ClientConfigFile {
            kind: ClientKind::Custom,
            path,
        }]);
        assert_eq!(report.findings.len(), 1);
        assert_eq!(report.findings[0].code, "CLIENT_CONFIG_UNREADABLE");
    }
}
