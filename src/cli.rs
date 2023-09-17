use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Rhub is a CLI for pushing local repositories to Github."
)]
pub struct CliArgs {
    #[arg(
        default_value = ".",
        value_name = "directory",
        help = "The directory to send to Github",
        value_parser = validate_path,
    )]
    pub directory: String,

    #[arg(
        short,
        long,
        default_value = "",
        help = "The description of the repository"
    )]
    pub description: String,

    #[arg(
        short,
        long,
        default_value = "",
        help = "The name that the repository will take on GitHub"
    )]
    pub name: String,

    #[arg(
        short,
        long,
        num_args = 0,
        help = "If present, makes the repository private"
    )]
    pub private: bool,
}

/// Validate a CLI path argument.
/// If the path does not exist, return an error.
fn validate_path(directory: &str) -> Result<String, String> {
    let path = std::path::Path::new(directory);
    if !path.exists() {
        return Err(format!("Path \"{}\" does not exist", directory));
    }
    Ok(directory.to_string())
}
