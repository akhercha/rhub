use std::path::Path;
use std::process::Command;

const GIT_COMMAND: &str = "git";
const GIT_INIT_ARG: &str = "init";

const GIT_FOLDER: &str = ".git";

pub fn path_exists(path: &Path) -> bool {
    path.exists()
}

pub fn call_git_init(directory: &str) {
    let git_path = Path::new(directory).join(GIT_FOLDER);
    if path_exists(&git_path) {
        println!("DEBUG: already a git repo");
        return;
    }
    let command_output = Command::new(GIT_COMMAND)
        .args([GIT_INIT_ARG, git_path.to_str().unwrap()])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(command_output.stdout).unwrap();
    println!("{}", stdout);
}
