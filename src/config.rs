use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub discord: DiscordConfig,
    pub sensor: SensorConfig,
    pub camera: CameraConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiscordConfig {
    pub webhook_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SensorConfig {
    pub gpio_pin: u8,
    pub cooldown_seconds: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CameraConfig {
    pub script_path: String,
    pub output_dir: String,
    pub filename_format: String,
    pub resolution: Resolution,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<String>,
}

impl Config {
    /// Load configuration from a YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path.as_ref())
            .context("Failed to read config file")?;

        let config: Config = serde_yaml::from_str(&contents)
            .context("Failed to parse config file")?;

        config.validate()?;

        Ok(config)
    }

    /// Validate configuration values
    fn validate(&self) -> Result<()> {
        if self.discord.webhook_url.is_empty()
            || self.discord.webhook_url.contains("YOUR_WEBHOOK") {
            anyhow::bail!("Discord webhook URL must be configured");
        }

        if self.sensor.gpio_pin > 27 {
            anyhow::bail!("GPIO pin must be between 0 and 27");
        }

        if self.sensor.cooldown_seconds == 0 {
            anyhow::bail!("Cooldown seconds must be greater than 0");
        }

        Ok(())
    }
}
