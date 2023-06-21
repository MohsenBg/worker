use git2::{Cred, RemoteCallbacks, Repository, Signature};
use std::path::PathBuf;

pub struct GitOptions {
    pub repo_directory: PathBuf,
    pub commit_msg: String,
    pub remote_name: String,
    pub email: String,
    pub name: String,
    pub public_key_path: PathBuf,
    pub private_key_path: PathBuf,
    pub passphrase: String,
}

pub fn send_to_git_hub(git_option: GitOptions) -> Result<(), git2::Error> {
    let repo = Repository::open(git_option.repo_directory)?;
    let mut index = repo.index()?;

    // Add
    index.add_all(["."], git2::IndexAddOption::DEFAULT, None)?;
    let oid = index.write_tree()?;

    // Commit
    let tree = repo.find_tree(oid)?;
    let head = repo.head()?;
    let parent_commit = repo.find_commit(head.target().unwrap())?;
    let signature = Signature::now(&git_option.name, &git_option.email)?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &git_option.commit_msg,
        &tree,
        &[&parent_commit],
    )?;

    // Push
    let mut remote = repo.find_remote(&git_option.remote_name)?;
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap_or(&git_option.name),
            Some(&git_option.public_key_path),
            &git_option.private_key_path,
            Some(&git_option.passphrase),
        )
    });

    let mut options = git2::PushOptions::new();
    options.remote_callbacks(callbacks);
    remote.push(&["refs/heads/master"], Some(&mut options))?;

    Ok(())
}
