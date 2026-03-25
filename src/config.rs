use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub source: SourceConfig,
}

#[derive(Deserialize)]
pub struct SourceConfig {
    pub paths: Vec<String>,
}

pub fn load(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}
