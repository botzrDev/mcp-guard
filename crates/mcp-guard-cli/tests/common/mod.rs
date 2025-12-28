use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::net::TcpListener;

pub struct TestContext {
    pub config_path: PathBuf,
    pub work_dir: tempfile::TempDir,
}

pub async fn setup_test_context() -> TestContext {
    let work_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let config_path = work_dir.path().join("config.toml");

    TestContext {
        config_path,
        work_dir,
    }
}

pub async fn get_free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    listener.local_addr().unwrap().port()
}

pub async fn wait_for_server(port: u16) -> bool {
    let url = format!("http://127.0.0.1:{}/health", port);
    for _ in 0..30 {
        if reqwest::get(&url).await.is_ok() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    false
}

pub fn cargo_bin(name: &str) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_mcp-guard"));
    cmd.env("RUST_LOG", "debug");
    cmd
}
