use std::fs;
use std::path::Path;
use toml::Value;

pub fn read_config(file_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let config: Value = toml::from_str(&content)?;
    Ok(config)
}

pub fn get_config_value(config: &Value, key: &str) -> Option<&Value> {
    config.get(key)
}