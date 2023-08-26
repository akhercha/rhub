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
        default_value = "config.toml",
        help = "The path to the configuration file containing the Github API key",
        value_parser = validate_toml_file,
    )]
    pub config: String,

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

/// Validate a CLI toml file argument.
/// Returns an error if:
///     - The path does not exist,
///     - The path is a directory,
///     - The path does not have a .toml extension.
fn validate_toml_file(path: &str) -> Result<String, String> {
    let path = std::path::Path::new(path);
    if !path.exists() {
        return Err(format!("Path \"{}\" does not exist", path.display()));
    }
    if path.is_dir() {
        return Err(format!("Path \"{}\" is a directory", path.display()));
    }
    if path.extension().unwrap_or_default() != "toml" {
        return Err(format!(
            "Path \"{}\" does not have a .toml extension",
            path.display()
        ));
    }
    Ok(path.display().to_string())
}
