use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use clap::Parser;

pub const DEFAULT_CONFIG: &str = r#"messages:
  ping:
    response_message: "Pong! Latency: [latency]"
"#;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to config file (default config.yaml)
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Generate config file to config.yaml
    #[arg(short = 'g', long)]
    pub config_gen: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub messages: MessagesConfig,
}

// messages
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessagesConfig {
    pub ping: PingConfig,
}

// messages.ping
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PingConfig {
    pub response_message: String,
}

impl Config {
    pub fn load(path: Option<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = if let Some(path) = path {
            if !path.exists() {
                return Err(format!("Config file not found: {}", path.display()).into());
            }
            println!("Loading config from {}", path.display());
            fs::read_to_string(path)?
        } else {
            let default_path = PathBuf::from("config.yaml");
            if default_path.exists() {
                println!("Loading config from config.yaml");
                fs::read_to_string(default_path)?
            } else {
                println!("No config file found. Using internal defaults.");
                DEFAULT_CONFIG.to_string()
            }
        };

        let config: Config = serde_yaml::from_str(&config_str)?;
        Ok(config)
    }

    pub fn generate_default() -> Result<(), Box<dyn std::error::Error>> {
        let path = PathBuf::from("config.yaml");
        if path.exists() {
            return Err("config.yaml already exists. Please remove or rename it before generating a new one.".into());
        }
        fs::write(path, DEFAULT_CONFIG)?;
        println!("Generated default config.yaml");
        Ok(())
    }
}
