mod api;
mod cli;
mod git;
mod toml_config;
mod utils;

use api::{create_new_repository_on_github, repo_exists_on_github, ApiHandler};
use clap::Parser;
use cli::CliArgs;
use git::{
    call_git_init, commit_changes, create_readme, push_to_github, rename_branch_to_main,
    set_remote_origin, stage_files,
};
use reqwest::Error;
use toml_config::{get_toml_config, TomlConfig};
use utils::fs::{count_files_in_path, get_directory_name};

const GITHUB_USER_AGENT: &str = "Rhub-CLI/0.1.0";

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse CLI arguments
    let mut cli_args = CliArgs::parse();

    // Change the name argument if none is supplied
    if cli_args.name.is_empty() {
        if let Ok(dir_name) = get_directory_name(&cli_args.directory) {
            cli_args.name = dir_name;
        } else {
            eprintln!("Error getting directory name");
            std::process::exit(1);
        }
    }

    // Retrieve TOML config
    let TomlConfig {
        github_pat_token: api_key,
        github_username: username,
    } = get_toml_config("config.toml");

    // Api Client
    let api_handler =
        ApiHandler::new(GITHUB_USER_AGENT, &api_key).expect("Failed to create API handler");

    // Check if repository name exists on GitHub, if yes exit
    if repo_exists_on_github(&api_handler, &username, &cli_args.name).await? {
        eprintln!("Repository already exists on GitHub");
        std::process::exit(1);
    }

    // Git system call, check if .git exists else create it for the directory
    call_git_init(&cli_args.directory.as_str());

    create_new_repository_on_github(&api_handler, &cli_args).await?;

    if count_files_in_path(&cli_args.directory) == 0 {
        create_readme(&cli_args.directory, &cli_args.name);
    }
    stage_files(&cli_args.directory);
    commit_changes(&cli_args.directory, "init");
    rename_branch_to_main(&cli_args.directory);
    set_remote_origin(&cli_args.directory, &username, &cli_args.name);
    push_to_github(&cli_args.directory);

    Ok(())
}
