mod api;
mod cli;
mod read_toml;
use clap::Parser;

use cli::CliArgs;
use read_toml::retrieve_api_key;

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse CLI arguments
    let cli_args = CliArgs::parse();
    for _ in 0..cli_args.count {
        println!("Hello {}!", cli_args.name)
    }

    // Retrieve TOML config
    println!("Your API key is: {}", retrieve_api_key("config.toml"));

    // API request GET
    api::get("https://jsonplaceholder.typicode.com/posts/1").await?;

    // API request POST
    api::post("https://jsonplaceholder.typicode.com/posts").await?;
    Ok(())
}
