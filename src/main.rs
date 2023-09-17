mod api;
mod cli;
mod git;
mod utils;

use api::GithubApi;
use clap::Parser;
use cli::CliArgs;
use git::{
    call_git_init, commit_changes, create_readme, push_to_github, rename_branch_to_main,
    set_remote_origin, stage_files,
};
use reqwest::Error;
use std::env;
use utils::fs::{count_files_in_path, get_directory_name};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut cli_args = CliArgs::parse();
    if cli_args.name.is_empty() {
        if let Ok(dir_name) = get_directory_name(&cli_args.directory) {
            cli_args.name = dir_name;
        } else {
            panic!("Error getting directory name");
        }
    }

    // TODO: abstract this out + allow for CLI argument or config file
    let api_key = env::var("GITHUB_PERSONAL_TOKEN").expect("GITHUB_PERSONAL_TOKEN not set");
    let api = GithubApi::new(&api_key).expect("Failed to create API handler");

    let username: String = api.get_username().await?;
    if api.repo_exists(&username, &cli_args.name).await? {
        panic!("Repository already exists on GitHub");
    }

    // TODO: currently yikes, refactor this
    call_git_init(cli_args.directory.as_str());
    api.create_new_repository(&cli_args).await?;
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
