use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use clap::Parser;
use crate::ui;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessagesConfig {
    pub ping: PingConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PingConfig {
    pub response_message: String,
}

impl Config {
    pub fn load(path: Option<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let default_path = PathBuf::from("config.yaml");
        let target_path = path.clone().unwrap_or_else(|| default_path.clone());

        if !target_path.exists() {
            if path.is_none() {
                print!("{}{} No config file found. Create one? (Y/n): ", ui::COLOR_WARN, ui::WARN);
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                print!("{}", ui::COLOR_RESET);
                io::stdout().flush()?;
                let input = input.trim().to_lowercase();
                if input.is_empty() || input == "y" {
                    Self::generate_default_at(&target_path)?;
                } else {
                    println!("{}{} Using internal defaults.", ui::COLOR_WARN, ui::WARN);
                    print!("{}", ui::COLOR_RESET);
                    return Ok(serde_yaml::from_str(DEFAULT_CONFIG)?);
                }
            } else {
                return Err(format!("Config file not found: {}", target_path.display()).into());
            }
        }

        let existing_str = fs::read_to_string(&target_path)?;
        let mut existing_val: serde_yaml::Value = serde_yaml::from_str(&existing_str)?;
        let default_val: serde_yaml::Value = serde_yaml::from_str(DEFAULT_CONFIG)?;

        let mut updated = false;
        merge_values(&mut existing_val, &default_val, &mut updated);

        if updated {
            let updated_str = serde_yaml::to_string(&existing_val)?;
            fs::write(&target_path, updated_str)?;
            println!("{}{} Your config was updated with new default values!", ui::COLOR_SUCCESS, ui::CHECK);
            print!("{}", ui::COLOR_RESET);
        }

        println!("Loading config from {}", target_path.display());

        let config: Config = serde_yaml::from_value(existing_val)?;

        Ok(config)
    }

    pub fn generate_default() -> Result<(), Box<dyn std::error::Error>> {
        Self::generate_default_at(&PathBuf::from("config.yaml"))
    }

    fn generate_default_at(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if path.exists() {
            return Err(format!("{} already exists. Please remove or rename it before generating a new one.", path.display()).into());
        }
        fs::write(path, DEFAULT_CONFIG)?;
        println!("{}{} Generated default {}", ui::COLOR_SUCCESS, ui::CHECK, path.display());
        Ok(())
    }
}

fn merge_values(existing: &mut serde_yaml::Value, default: &serde_yaml::Value, updated: &mut bool) {
    if let (Some(existing_map), Some(default_map)) = (existing.as_mapping_mut(), default.as_mapping()) {
        for (key, default_val) in default_map {
            if !existing_map.contains_key(key) {
                existing_map.insert(key.clone(), default_val.clone());
                *updated = true;
            } else {
                let existing_val = existing_map.get_mut(key).unwrap();
                merge_values(existing_val, default_val, updated);
            }
        }
    }
}
