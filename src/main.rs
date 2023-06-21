use std::{path::PathBuf, process};
mod command;
mod errors;
mod git_util;
mod sys;
use errors::ExitCode;
use git_util::{send_to_git_hub, GitOptions};

fn main() {
    let mut git_option = GitOptions {
        repo_directory: PathBuf::from("/home/Bagheri/Projects/rust/devWorker/"),
        commit_msg: String::new(),
        remote_name: String::from("origin"),
        email: String::from("moh.1380.1393@gmail.com"),
        name: String::from("MohsenBg"),
        public_key_path: PathBuf::from("/home/Bagheri/.ssh/worker_ssh.pub"),
        private_key_path: PathBuf::from("/home/Bagheri/.ssh/worker_ssh"),
        passphrase: String::from(""),
    };

    let file = sys::generate_log_file();

    git_option.commit_msg += &format!("File '{:?}' has been created.", file.file_name().unwrap());
    match send_to_git_hub(git_option) {
        Ok(()) => process::exit(ExitCode::Success as i32),
        Err(err) => panic!("{} Error code: {}", err, 1),
    }
}
