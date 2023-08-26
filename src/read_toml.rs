use serde::Deserialize;
use std::fs::read_to_string;
use toml;

#[derive(Debug, Deserialize)]
pub struct TomlConfig {
    github_api_key: String,
}

fn read(filename: &str) -> TomlConfig {
    let toml_str = read_to_string(filename).expect("Failed to read Cargo.toml file");
    toml::from_str(&toml_str).expect("Failed to deserialize Cargo.toml")
}

pub fn get_api_key(filename: &str) -> String {
    let config_toml: TomlConfig = read(filename);
    config_toml.github_api_key
}
