mod api;
mod cli;
mod git;
mod read_toml;

use clap::Parser;
use cli::CliArgs;
use git::call_git_status;
use read_toml::retrieve_api_key;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse CLI arguments
    let cli_args = CliArgs::parse();
    println!("{:?}", cli_args);

    // Retrieve TOML config
    println!("Your API key is: \"{}\"", retrieve_api_key("config.toml"));

    // API request GET
    api::get("https://jsonplaceholder.typicode.com/posts/1").await?;

    // API request POST
    api::post("https://jsonplaceholder.typicode.com/posts").await?;

    // Git system call
    call_git_status();

    Ok(())
}
