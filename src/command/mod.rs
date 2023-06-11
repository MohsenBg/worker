use std::env;
use std::process::Command;

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
    let output = Command::new("git")
        .arg("push")
        .arg("-u")
        .arg(format!(
            "{}:{}git@github.com:MohsenBg/worker.git",
            username, password
        ))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let err = format!("Git push failed. Error: {}", stderr);
        Err(err)
    }
}
