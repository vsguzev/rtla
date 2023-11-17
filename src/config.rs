use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::io::{self, Read};

// Define configuration structure
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub log_config: LogConfig,
    pub alert_config: AlertConfig,
}

#[derive(Serialize, Deserialize)]
pub struct LogConfig {
    // TODO
    // Add log configuration fields here
    // For example, log file paths, types, etc.
}

#[derive(Serialize, Deserialize)]
pub struct AlertConfig {
    // TODO
    // Add alert configuration fields here
    // For example, rules for triggering alerts
}

pub fn load_config<P: AsRef<Path>>(path: P) -> io::Result<Config> {
    // Read the configuration file
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the TOML configuration
    let config: Config = toml::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(config)
}
