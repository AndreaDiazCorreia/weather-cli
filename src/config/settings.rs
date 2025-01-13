use serde::Deserialize;
use config::{Config, ConfigError, File};
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub api_key: String,
    pub default_country: String,
    pub temperature_unit: String,
    pub display_format: DisplayFormat,
    pub save_history: bool,
    pub max_history_items: usize,
}

#[derive(Debug, Deserialize)]
pub enum DisplayFormat {
    #[serde(rename = "simple")]
    Simple,
    #[serde(rename = "detailed")]
    Detailed,
    #[serde(rename = "compact")]
    Compact,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            default_country: "US".to_string(),
            temperature_unit: "celsius".to_string(),
            display_format: DisplayFormat::Simple,
            save_history: true,
            max_history_items: 10,
        }
    }
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.api_key.is_empty() {
            return Err(ConfigError::NotFound("API key is required".into()));
        }
        if self.default_country.len() != 2 {
            return Err(ConfigError::Message(
                "Default country must be a 2-letter code".into()
            ));
        }
        Ok(())
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let config_string = toml::to_string(self)
            .map_err(|e| ConfigError::Message(e.to_string()))?;
        
        std::fs::write("config.toml", config_string)
            .map_err(|e| ConfigError::Message(e.to_string()))?;
        
        Ok(())
    }
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let config_path = Path::new("config.toml");
    let template_path = Path::new("config.tpl.toml");

    if !config_path.exists() && template_path.exists() {
        println!("Configuration file not found.");
        println!("Please copy config.tpl.toml to config.toml and add your API key.");
        return Err(ConfigError::NotFound("Configuration file not found".into()));
    }

    let config = Config::builder()
        .add_source(File::from(template_path).required(false))
        .add_source(File::from(config_path).required(false))
        .build()?;

    let app_config: AppConfig = config.try_deserialize()?;
    app_config.validate()?;

    Ok(app_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let valid_config = AppConfig {
            api_key: "test_key".to_string(),
            default_country: "US".to_string(),
            ..AppConfig::default()
        };
        assert!(valid_config.validate().is_ok());

        let invalid_config = AppConfig {
            api_key: "".to_string(),
            ..AppConfig::default()
        };
        assert!(invalid_config.validate().is_err());
    }
}