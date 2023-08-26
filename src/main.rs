mod api;
mod cli;
mod git;
mod toml_config;
mod utils;

use api::{create_new_repository_on_github, repo_exists_on_github, ApiHandler};
use clap::Parser;
use cli::CliArgs;
use git::call_git_init;
use reqwest::Error;
use toml_config::{get_toml_config, TomlConfig};
use utils::io::get_directory_name;

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
    println!("{:?}", cli_args);

    // Retrieve TOML config
    let TomlConfig {
        github_pat_token: api_key,
        github_username: username,
    } = get_toml_config("config.toml");

    // Api Client
    let api_handler =
        ApiHandler::new(GITHUB_USER_AGENT, &api_key).expect("Failed to create API handler");

    // Git system call, check if .git exists else create it for the directory
    call_git_init(&cli_args.directory.as_str());

    // Check if repository name exists on GitHub, if yes exit
    if repo_exists_on_github(&api_handler, &username, &cli_args.name).await? {
        eprintln!("Repository already exists on GitHub");
        std::process::exit(1);
    }

    // Create new repository on GitHub
    create_new_repository_on_github(&api_handler, &cli_args).await?;

    Ok(())
}
