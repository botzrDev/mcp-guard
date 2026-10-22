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

//! Local policy evaluation: tool globs plus argument-level rules.
//!
//! Evaluation order for a `tools/call`:
//!
//! 1. `denied_tools`: a match denies.
//! 2. `allowed_tools`: no match denies.
//! 3. `rules`, in order: the first rule whose `tools` match and whose `when`
//!    conditions all hold decides. No matching rule means allow.
//!
//! The result is a [`Decision`]. Whether a `Deny` is enforced or only recorded
//! is decided by the caller from `PolicyConfig::mode`.

use crate::config::{ArgCondition, OneOrMany, PolicyConfig, PolicyRule, RuleAction};
use serde_json::Value;

/// Outcome of evaluating a call against the policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Deny {
        /// Human-readable reason, suitable for returning to the agent.
        reason: String,
        /// Name or index of the rule that matched, when a rule decided.
        rule: Option<String>,
    },
    /// Pause for human approval (`action = "hold"`).
    Hold {
        reason: String,
        rule: String,
    },
}

impl Decision {
    pub fn is_allow(&self) -> bool {
        matches!(self, Decision::Allow)
    }

    pub fn reason(&self) -> Option<&str> {
        match self {
            Decision::Allow => None,
            Decision::Deny { reason, .. } | Decision::Hold { reason, .. } => Some(reason),
        }
    }
}

/// Evaluate a `tools/call` against the policy.
pub fn evaluate(policy: &PolicyConfig, tool: &str, arguments: &Value) -> Decision {
    if policy.is_denied(tool) {
        return Decision::Deny {
            reason: format!("tool '{}' is in denied_tools", tool),
            rule: None,
        };
    }
    if !policy
        .allowed_tools
        .iter()
        .any(|p| crate::config::glob_match(p, tool))
    {
        return Decision::Deny {
            reason: format!("tool '{}' is not in allowed_tools", tool),
            rule: None,
        };
    }
    for (i, rule) in policy.rules.iter().enumerate() {
        if !rule
            .tools
            .iter()
            .any(|p| crate::config::glob_match(p, tool))
        {
            continue;
        }
        if !rule_conditions_hold(rule, arguments) {
            continue;
        }
        let label = rule
            .name
            .clone()
            .unwrap_or_else(|| format!("policy.rules[{}]", i));
        return match rule.action {
            RuleAction::Allow => Decision::Allow,
            RuleAction::Deny => Decision::Deny {
                reason: rule
                    .reason
                    .clone()
                    .unwrap_or_else(|| format!("denied by rule '{}'", label)),
                rule: Some(label),
            },
            RuleAction::Hold => Decision::Hold {
                reason: rule
                    .reason
                    .clone()
                    .unwrap_or_else(|| format!("held for approval by rule '{}'", label)),
                rule: label,
            },
        };
    }
    Decision::Allow
}

/// Whether every `when` condition of a rule holds for these arguments.
pub fn rule_conditions_hold(rule: &PolicyRule, arguments: &Value) -> bool {
    rule.when
        .iter()
        .all(|(path, cond)| condition_holds(cond, lookup(arguments, path)))
}

/// Resolve a dotted path (`options.recursive`, `files.0.name`) in a JSON value.
pub fn lookup<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for segment in path.split('.') {
        current = match current {
            Value::Object(map) => map.get(segment)?,
            Value::Array(items) => {
                let idx: usize = segment.parse().ok()?;
                items.get(idx)?
            }
            _ => return None,
        };
    }
    Some(current)
}

fn any_str(list: &OneOrMany<String>, f: impl Fn(&str) -> bool) -> bool {
    list.iter().any(|s| f(s))
}

fn glob_hit(pattern: &str, s: &str) -> bool {
    glob::Pattern::new(pattern)
        .map(|p| p.matches(s))
        .unwrap_or(false)
}

fn regex_hit(pattern: &str, s: &str) -> bool {
    regex::Regex::new(pattern)
        .map(|r| r.is_match(s))
        .unwrap_or(false)
}

/// Whether one condition holds for one (possibly absent) argument value.
pub fn condition_holds(cond: &ArgCondition, value: Option<&Value>) -> bool {
    let Some(value) = value else {
        // Absent argument: only `exists = false` can hold, and nothing else
        // may be required alongside it.
        return cond.exists == Some(false) && {
            let mut rest = cond.clone();
            rest.exists = None;
            rest == ArgCondition::default()
        };
    };
    if cond.exists == Some(false) {
        return false;
    }
    if let Some(expected) = &cond.equals {
        if value != expected {
            return false;
        }
    }
    if let Some(expected) = &cond.not_equals {
        if value == expected {
            return false;
        }
    }
    if let Some(set) = &cond.one_of {
        if !set.iter().any(|v| v == value) {
            return false;
        }
    }
    if let Some(set) = &cond.not_in {
        if set.iter().any(|v| v == value) {
            return false;
        }
    }

    let needs_string = cond.starts_with.is_some()
        || cond.not_starts_with.is_some()
        || cond.ends_with.is_some()
        || cond.not_ends_with.is_some()
        || cond.contains.is_some()
        || cond.not_contains.is_some()
        || cond.glob.is_some()
        || cond.not_glob.is_some()
        || cond.regex.is_some()
        || cond.not_regex.is_some();
    if needs_string {
        let Some(s) = value.as_str() else {
            return false;
        };
        if let Some(l) = &cond.starts_with {
            if !any_str(l, |p| s.starts_with(p)) {
                return false;
            }
        }
        if let Some(l) = &cond.not_starts_with {
            if any_str(l, |p| s.starts_with(p)) {
                return false;
            }
        }
        if let Some(l) = &cond.ends_with {
            if !any_str(l, |p| s.ends_with(p)) {
                return false;
            }
        }
        if let Some(l) = &cond.not_ends_with {
            if any_str(l, |p| s.ends_with(p)) {
                return false;
            }
        }
        if let Some(l) = &cond.contains {
            if !any_str(l, |p| s.contains(p)) {
                return false;
            }
        }
        if let Some(l) = &cond.not_contains {
            if any_str(l, |p| s.contains(p)) {
                return false;
            }
        }
        if let Some(l) = &cond.glob {
            if !any_str(l, |p| glob_hit(p, s)) {
                return false;
            }
        }
        if let Some(l) = &cond.not_glob {
            if any_str(l, |p| glob_hit(p, s)) {
                return false;
            }
        }
        if let Some(l) = &cond.regex {
            if !any_str(l, |p| regex_hit(p, s)) {
                return false;
            }
        }
        if let Some(l) = &cond.not_regex {
            if any_str(l, |p| regex_hit(p, s)) {
                return false;
            }
        }
    }

    if let Some(max) = cond.max_length {
        let len = match value {
            Value::String(s) => s.chars().count(),
            Value::Array(a) => a.len(),
            Value::Object(o) => o.len(),
            _ => return false,
        };
        if len > max {
            return false;
        }
    }

    let needs_number =
        cond.gt.is_some() || cond.gte.is_some() || cond.lt.is_some() || cond.lte.is_some();
    if needs_number {
        let Some(n) = value.as_f64() else {
            return false;
        };
        // NaN compares as incomparable and fails every bound.
        let ordered = |bound: f64, accept: fn(std::cmp::Ordering) -> bool| {
            n.partial_cmp(&bound).is_some_and(accept)
        };
        if cond.gt.is_some_and(|b| !ordered(b, |o| o.is_gt())) {
            return false;
        }
        if cond.gte.is_some_and(|b| !ordered(b, |o| o.is_ge())) {
            return false;
        }
        if cond.lt.is_some_and(|b| !ordered(b, |o| o.is_lt())) {
            return false;
        }
        if cond.lte.is_some_and(|b| !ordered(b, |o| o.is_le())) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn policy_from(toml_text: &str) -> PolicyConfig {
        toml::from_str(toml_text).expect("policy toml")
    }

    #[test]
    fn lookup_resolves_nested_paths() {
        let v = json!({"options": {"recursive": true}, "files": [{"name": "a"}, {"name": "b"}]});
        assert_eq!(lookup(&v, "options.recursive"), Some(&json!(true)));
        assert_eq!(lookup(&v, "files.1.name"), Some(&json!("b")));
        assert_eq!(lookup(&v, "files.9.name"), None);
        assert_eq!(lookup(&v, "missing"), None);
        assert_eq!(lookup(&v, "options.recursive.deeper"), None);
    }

    #[test]
    fn globs_decide_before_rules() {
        let p = policy_from(
            r#"
allowed_tools = ["read_*"]
denied_tools = ["read_secret"]
[[rules]]
tools = ["*"]
action = "allow"
"#,
        );
        assert!(evaluate(&p, "read_file", &json!({})).is_allow());
        assert!(matches!(
            evaluate(&p, "read_secret", &json!({})),
            Decision::Deny { rule: None, .. }
        ));
        assert!(matches!(
            evaluate(&p, "write_file", &json!({})),
            Decision::Deny { rule: None, .. }
        ));
    }

    #[test]
    fn first_matching_rule_wins() {
        let p = policy_from(
            r#"
[[rules]]
name = "workspace writes ok"
tools = ["write_file"]
action = "allow"
[rules.when]
path = { starts_with = "/work/" }

[[rules]]
name = "no other writes"
tools = ["write_*"]
action = "deny"
reason = "writes outside the workspace are blocked"
"#,
        );
        assert!(evaluate(&p, "write_file", &json!({"path": "/work/a.txt"})).is_allow());
        let d = evaluate(&p, "write_file", &json!({"path": "/etc/passwd"}));
        assert_eq!(
            d,
            Decision::Deny {
                reason: "writes outside the workspace are blocked".into(),
                rule: Some("no other writes".into())
            }
        );
        // Missing path: first rule's condition cannot hold, second rule has no
        // conditions and denies.
        assert!(!evaluate(&p, "write_file", &json!({})).is_allow());
        // Unrelated tool: no rule matches, allowed.
        assert!(evaluate(&p, "read_file", &json!({"path": "/etc/passwd"})).is_allow());
    }

    #[test]
    fn hold_rule_produces_hold_decision() {
        let p = policy_from(
            r#"
[[rules]]
name = "confirm destructive"
tools = ["delete_*", "run_shell"]
action = "hold"
reason = "destructive operation"
"#,
        );
        assert_eq!(
            evaluate(&p, "delete_file", &json!({})),
            Decision::Hold {
                reason: "destructive operation".into(),
                rule: "confirm destructive".into()
            }
        );
        assert!(evaluate(&p, "read_file", &json!({})).is_allow());
        assert_eq!(
            evaluate(&p, "run_shell", &json!({})).reason(),
            Some("destructive operation")
        );
    }

    #[test]
    fn unnamed_rule_gets_index_label() {
        let p = policy_from(
            r#"
[[rules]]
tools = ["rm"]
action = "deny"
"#,
        );
        let d = evaluate(&p, "rm", &json!({}));
        assert_eq!(
            d,
            Decision::Deny {
                reason: "denied by rule 'policy.rules[0]'".into(),
                rule: Some("policy.rules[0]".into())
            }
        );
    }

    fn cond(toml_text: &str) -> ArgCondition {
        toml::from_str(toml_text).expect("condition toml")
    }

    #[test]
    fn string_operators() {
        let v = json!("/home/me/project/src/main.rs");
        assert!(condition_holds(
            &cond(r#"starts_with = "/home/me/project/""#),
            Some(&v)
        ));
        assert!(condition_holds(
            &cond(r#"starts_with = ["/tmp/", "/home/me/project/"]"#),
            Some(&v)
        ));
        assert!(!condition_holds(
            &cond(r#"not_starts_with = "/home/me/""#),
            Some(&v)
        ));
        assert!(condition_holds(&cond(r#"ends_with = ".rs""#), Some(&v)));
        assert!(condition_holds(&cond(r#"contains = "/src/""#), Some(&v)));
        assert!(condition_holds(
            &cond(r#"not_contains = [".ssh", ".env"]"#),
            Some(&v)
        ));
        assert!(condition_holds(
            &cond(r#"glob = "/home/me/project/**/*.rs""#),
            Some(&v)
        ));
        assert!(condition_holds(&cond(r#"regex = "\\.rs$""#), Some(&v)));
        assert!(!condition_holds(&cond(r#"regex = "^/etc""#), Some(&v)));
        assert!(condition_holds(&cond(r#"max_length = 100"#), Some(&v)));
        assert!(!condition_holds(&cond(r#"max_length = 5"#), Some(&v)));
        // String operators never match a non-string.
        assert!(!condition_holds(
            &cond(r#"starts_with = "1""#),
            Some(&json!(123))
        ));
    }

    #[test]
    fn equality_and_set_operators() {
        assert!(condition_holds(
            &cond(r#"equals = true"#),
            Some(&json!(true))
        ));
        assert!(!condition_holds(
            &cond(r#"equals = true"#),
            Some(&json!(false))
        ));
        assert!(condition_holds(
            &cond(r#"not_equals = "rm -rf""#),
            Some(&json!("ls"))
        ));
        assert!(condition_holds(
            &cond(r#"in = ["GET", "HEAD"]"#),
            Some(&json!("GET"))
        ));
        assert!(!condition_holds(
            &cond(r#"in = ["GET", "HEAD"]"#),
            Some(&json!("DELETE"))
        ));
        assert!(condition_holds(
            &cond(r#"not_in = ["DELETE"]"#),
            Some(&json!("GET"))
        ));
    }

    #[test]
    fn numeric_operators() {
        assert!(condition_holds(&cond("lte = 100"), Some(&json!(100))));
        assert!(!condition_holds(&cond("lt = 100"), Some(&json!(100))));
        assert!(condition_holds(&cond("gt = 0\nlt = 10"), Some(&json!(5))));
        assert!(!condition_holds(&cond("gt = 0"), Some(&json!("5"))));
    }

    #[test]
    fn exists_operator_and_absent_values() {
        assert!(condition_holds(&cond("exists = false"), None));
        assert!(!condition_holds(&cond("exists = true"), None));
        assert!(condition_holds(&cond("exists = true"), Some(&json!(null))));
        assert!(!condition_holds(&cond("exists = false"), Some(&json!("x"))));
        // Absent argument fails every other operator, including negated ones.
        assert!(!condition_holds(&cond(r#"not_starts_with = "/etc""#), None));
        assert!(!condition_holds(
            &cond(
                r#"exists = false
starts_with = "/""#
            ),
            None
        ));
    }

    #[test]
    fn all_conditions_in_a_rule_must_hold() {
        let p = policy_from(
            r#"
[[rules]]
tools = ["http_request"]
action = "deny"
[rules.when]
method = { not_in = ["GET", "HEAD"] }
url = { not_starts_with = "https://api.internal/" }
"#,
        );
        assert!(evaluate(
            &p,
            "http_request",
            &json!({"method": "GET", "url": "https://x/"})
        )
        .is_allow());
        assert!(evaluate(
            &p,
            "http_request",
            &json!({"method": "POST", "url": "https://api.internal/v1"})
        )
        .is_allow());
        assert!(!evaluate(
            &p,
            "http_request",
            &json!({"method": "POST", "url": "https://evil/"})
        )
        .is_allow());
    }
}
