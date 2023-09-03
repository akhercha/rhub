use super::utils::fs::path_exists;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const GIT_COMMAND: &str = "git";
const GIT_INIT_ARG: &str = "init";
const GIT_LS_REMOTE_ARG: &str = "ls-remote";
const GIT_FOLDER: &str = ".git";
const GIT_REMOTE_ADD_ARG: &str = "remote";
const GIT_BRANCH_ARG: &str = "branch";
const GIT_ADD_ARG: &str = "add";
const GIT_PUSH_ARG: &str = "push";
const GIT_COMMIT_ARG: &str = "commit";
const README_FILE: &str = "README.md";

/// Add the remote origin to the git repo.
pub fn set_remote_origin(directory: &str, user: &str, repo_name: &str) {
    let url = format!("git@github.com:{}/{}.git", user, repo_name);
    Command::new(GIT_COMMAND)
        .args([GIT_REMOTE_ADD_ARG, "add", "origin", &url])
        .current_dir(directory)
        .output()
        .expect("failed to add remote");
    println!("Set remote origin using {}!", &url);
}

/// Rename the current branch to main.
pub fn rename_branch_to_main(directory: &str) {
    Command::new(GIT_COMMAND)
        .args([GIT_BRANCH_ARG, "-M", "main"])
        .current_dir(directory)
        .output()
        .expect("failed to rename branch");
    println!("Switched to branch main!");
}

/// Stage all the files in the directory.
pub fn stage_files(directory: &str) {
    Command::new(GIT_COMMAND)
        .args([GIT_ADD_ARG, "-A"])
        .current_dir(directory)
        .output()
        .expect("failed to add files");
    println!("Staged (git add -A) files!");
}

/// Create a README.md file in the directory.
pub fn create_readme(directory: &str, repo_name: &str) {
    println!("No files in directory, creating README.md...");
    let mut file = fs::File::create(Path::new(directory).join(README_FILE))
        .expect("Failed to create README.md");

    if let Err(e) = write!(file, "# {}", repo_name) {
        eprintln!("Failed to write to README.md: {}", e);
        std::process::exit(1);
    }
    println!("README.md created!");
}

/// Commit the staged changes with the given message.
pub fn commit_changes(directory: &str, message: &str) {
    Command::new(GIT_COMMAND)
        .args([GIT_COMMIT_ARG, "-m", message])
        .current_dir(directory)
        .output()
        .expect("failed to commit changes");
    println!("Committed changes with message: '{}'", message);
}

/// Push the staged changes to GitHub.
/// If the push fails, exit with an error.
pub fn push_to_github(directory: &str) {
    Command::new(GIT_COMMAND)
        .args([GIT_PUSH_ARG, "-u", "origin", "main"])
        .current_dir(directory)
        .output()
        .expect("failed to push to github");
    println!("Pushed to GitHub!");
}

/// Check if the git repo has a remote configured.
/// If it does, return true.
fn has_remote_origin_set(git_path: &Path) -> bool {
    let command_output = Command::new(GIT_COMMAND)
        .args([GIT_LS_REMOTE_ARG])
        .current_dir(git_path)
        .output()
        .expect("failed to execute process");
    !String::from_utf8(command_output.stdout).unwrap().is_empty()
}

/// Call git init on the directory.
/// If the directory is already a git repo, check if it has a remote configured.
/// If it does, exit with an error.
/// If it does not, continue & execute the git init command.
pub fn call_git_init(directory: &str) {
    let git_path = Path::new(directory).join(GIT_FOLDER);
    if path_exists(&git_path) {
        if has_remote_origin_set(&git_path) {
            eprintln!("Error: already a git repo with a remote");
            std::process::exit(1);
        }
        return;
    }
    let command_output = Command::new(GIT_COMMAND)
        .args([GIT_INIT_ARG, directory])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(command_output.stdout).unwrap();
    println!("{}", stdout);
}
