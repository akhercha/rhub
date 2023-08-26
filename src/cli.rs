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
        help = "The directory to send to Github"
    )]
    pub directory: String,

    #[arg(
        short,
        long,
        default_value = "config.toml",
        help = "The path to the configuration file containing the Github API key"
    )]
    pub config: String,

    #[arg(
        short,
        long,
        default_value = "",
        help = "The description of the repository"
    )]
    pub description: String,

    #[arg(short, long, default_value = "", help = "The name of the repository")]
    pub name: String,

    #[arg(
        short,
        long,
        num_args = 0,
        help = "If present, makes the repository private"
    )]
    pub private: bool,
}
