use serde::Deserialize;
use std::fs::read_to_string;
use toml;

#[derive(Debug, Deserialize)]
pub struct TomlConfig {
    pub github_pat_token: String,
    pub github_username: String,
}

fn read(filename: &str) -> TomlConfig {
    let toml_str = read_to_string(filename).expect("Failed to read Cargo.toml file");
    toml::from_str(&toml_str).expect("Failed to deserialize Cargo.toml")
}

pub fn get_toml_config(filename: &str) -> TomlConfig {
    let config_toml: TomlConfig = read(filename);
    config_toml
}
