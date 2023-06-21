use dotenv::dotenv;
use std::{env, path::PathBuf, process};
mod command;
mod errors;
mod git_util;
mod sys;
use errors::ExitCode;
use git_util::{send_to_git_hub, GitOptions};

fn main() {
    //example

    if let Err(err) = dotenv() {
        println!("Error loading .env file: {}", err);
        panic!(
            "An error occurred. Error code: {}",
            ExitCode::EnvFailed as i32
        );
    }

    let ssh_password = match env::var("SSH_GIT_PASSWORD") {
        Ok(password) => password,
        Err(_) => {
            println!("Failed to retrieve SSH_GIT_PASSWORD");
            panic!(
                "An error occurred. Error code: {}",
                ExitCode::ReadShhPasswordFailed as i32
            );
        }
    };

    let mut git_option = GitOptions {
        repo_directory: PathBuf::from("/home/Bagheri/Projects/rust/devWorker/"),
        commit_msg: String::new(),
        remote_name: String::from("origin"),
        email: String::from("moh.1380.1393@gmail.com"),
        name: String::from("MohsenBg"),
        public_key_path: PathBuf::from("~/.ssh/worker_ssh.pub"),
        private_key_path: PathBuf::from("~/.ssh/worker_ssh"),
        passphrase: String::from(""),
    };

    let file = sys::generate_log_file();

    git_option.commit_msg += &format!("File '{:?}' has been created.", file.file_name().unwrap());
    match send_to_git_hub(git_option) {
        Ok(()) => process::exit(ExitCode::Success as i32),
        Err(err) => panic!("{} Error code: {}", err, 1),
    }

    // match command::check_git_installed() {
    //     Ok(msg) => println!("{}", msg),
    //     Err(err) => panic!("{} Error code: {}", err, ExitCode::GitNotFound as i32),
    // }

    // match command::git_add() {
    //     Ok(msg) => println!("{}", msg),
    //     Err(err) => panic!("{} Error code: {}", err, ExitCode::GitAddFailed as i32),
    // }

    // match command::git_commit(format!(
    //     "File '{:?}' has been created.",
    //     file.file_name().unwrap()
    // )) {
    //     Ok(msg) => println!("{}", msg),
    //     Err(err) => panic!("{} Error code: {}", err, ExitCode::GitAddFailed as i32),
    // }

    // match command::git_push("MohsenBg", &ssh_password) {
    //     Ok(msg) => println!("{}", msg),
    //     Err(err) => panic!("{} Error code: {}", err, ExitCode::GitAddFailed as i32),
    // }
}
