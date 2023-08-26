use std::path::Path;
use std::process::Command;

const GIT_COMMAND: &str = "git";

const GIT_INIT_ARG: &str = "init";
const GIT_LS_REMOTE_ARG: &str = "ls-remote";

const GIT_FOLDER: &str = ".git";

/// Check if the path exists.
fn path_exists(path: &Path) -> bool {
    path.exists()
}

/// Check if the git repo has a remote configured.
pub fn has_remote_configured(git_path: &Path) -> bool {
    let command_output = Command::new(GIT_COMMAND)
        .args(&[GIT_LS_REMOTE_ARG])
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
        if has_remote_configured(&git_path) {
            eprintln!("Error: already a git repo with a remote");
            std::process::exit(1);
        }
        return;
    }
    let command_output = Command::new(GIT_COMMAND)
        .args(&[GIT_INIT_ARG, directory])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(command_output.stdout).unwrap();
    println!("{}", stdout);
}
