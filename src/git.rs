use std::process::Command;

pub fn call_git_status() {
    let command_output = Command::new("git")
        .args(["status"])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(command_output.stdout).unwrap();
    println!("{}", stdout);
}
