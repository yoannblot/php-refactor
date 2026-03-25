use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub source: SourceConfig,
    #[serde(flatten)]
    pub rules: HashMap<String, HashMap<String, RuleConfig>>,
}

#[derive(Deserialize)]
pub struct SourceConfig {
    pub paths: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct RuleConfig {
    pub paths: Vec<String>,
}

pub fn load(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}
