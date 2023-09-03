use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Debug, Deserialize)]
pub struct TomlConfig {
    pub github_pat_token: String,
    pub github_username: String,
}

fn read(filename: &str) -> TomlConfig {
    let toml_str = read_to_string(filename).expect("Failed to read configuration file");
    let config: TomlConfig =
        toml::from_str(&toml_str).expect("Failed to parse the toml configuration");
    if config.github_pat_token.is_empty() {
        panic!("Invalid configuration: github_pat_token is empty");
    }
    if config.github_username.is_empty() {
        panic!("Invalid configuration: github_username is empty");
    }
    config
}

pub fn get_toml_config(filename: &str) -> TomlConfig {
    let config_toml: TomlConfig = read(filename);
    config_toml
}
