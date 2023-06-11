use std::env;
use std::process::{Command, Stdio};

pub fn check_git_installed() -> Result<String, String> {
    let output = Command::new("git")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let err = format!("Git is not installed. Error: {}", stderr);
        Err(err)
    }
}

pub fn git_add() -> Result<String, String> {
    let current_dir = env::current_dir().expect("Failed to retrieve current directory");

    let output = Command::new("git")
        .arg("add")
        .arg(current_dir)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let err = format!("Git add failed. Error: {}", stderr);
        Err(err)
    }
}

pub fn git_commit(msg: String) -> Result<String, String> {
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(msg)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let err = format!("Git commit failed. Error: {}", stderr);
        Err(err)
    }
}

pub fn git_push(username: &str, password: &str) -> Result<String, String> {
    let mut git_cmd = Command::new("git");
    git_cmd
        .arg("push")
        .arg("-u")
        .env(
            "GIT_SSH_COMMAND",
            format!("ssh -o BatchMode=yes -o Passphrase={}", password),
        )
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = git_cmd.output().expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let err = format!("Git push failed. Error: {}", stderr);
        Err(err)
    }
}
