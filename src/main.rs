mod api;
mod cli;
mod git;
mod read_toml;

use clap::Parser;
use cli::CliArgs;
use git::call_git_init;
use read_toml::get_api_key;
use reqwest::Error;

async fn tests() -> Result<(), Error> {
    // Retrieve TOML config
    println!("Your API key is: \"{}\"", get_api_key("config.toml"));
    // API request GET
    api::get("https://jsonplaceholder.typicode.com/posts/1").await?;
    // API request POST
    api::post("https://jsonplaceholder.typicode.com/posts").await?;
    Ok(())
}

fn get_directory_name(path: &str) -> Result<String, std::io::Error> {
    let p = if path == "." {
        std::env::current_dir()?
    } else {
        std::path::PathBuf::from(path)
    };
    Ok(p.file_name()
        .map_or_else(|| ".".to_string(), |s| s.to_string_lossy().into_owned()))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tests().await?;
    println!("===============================");

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

    // Check the args
    println!("{:?}", cli_args);

    // TODO: Check if the name does not already exists on GitHub

    // Git system call, check if .git exists else create it for the directory
    call_git_init(&cli_args.directory.as_str());

    Ok(())
}
