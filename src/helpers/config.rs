use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub version: String, //For the /About endpoint
    pub openai_api_url: String,
    pub openai_api_key: String,
    pub model: String,
    pub max_iterations: usize
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config/config.json")?;

    log::warn!("Attempting to load config.");

    let config: Config = serde_json::from_str(&config_str)?;

    Ok(config)
}