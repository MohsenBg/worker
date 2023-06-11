use dotenv::dotenv;
use std::{env, process};
mod command;
mod errors;
mod sys;
use errors::ExitCode;

fn main() {
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

    let file = sys::generate_log_file();

    match command::check_git_installed() {
        Ok(msg) => println!("{}", msg),
        Err(err) => panic!("{} Error code: {}", err, ExitCode::GitNotFound as i32),
    }

    match command::git_add() {
        Ok(msg) => println!("{}", msg),
        Err(err) => panic!("{} Error code: {}", err, ExitCode::GitAddFailed as i32),
    }

    match command::git_commit(format!("File '{:?}' has been created.", file.file_name())) {
        Ok(msg) => println!("{}", msg),
        Err(err) => panic!("{} Error code: {}", err, ExitCode::GitAddFailed as i32),
    }

    match command::git_push("MohsenBg", &ssh_password) {
        Ok(msg) => println!("{}", msg),
        Err(err) => panic!("{} Error code: {}", err, ExitCode::GitAddFailed as i32),
    }

    process::exit(ExitCode::Success as i32);
}
