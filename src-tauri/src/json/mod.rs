use std::fs::File;
use std::io::Write;

use crate::app_state::{Config, Step};

pub fn read_config() -> Result<Config, std::io::Error> {
    let file = File::open("resources/config.json")
        .map_err(|e| std::io::Error::new(e.kind(), format!("Failed to open config file: {}", e)))?;

    let mut config: Config = serde_json::from_reader(file).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to parse config file: {}", e),
        )
    })?;

    fix_config(&mut config)?;

    Ok(config)
}

pub fn write_config(config: &mut Config) -> Result<(), std::io::Error> {
    fix_config(config)?;

    let json = serde_json::to_string_pretty(config).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to serialize config: {}", e),
        )
    })?;

    let mut file = File::create("resources/config.json").map_err(|e| {
        std::io::Error::new(e.kind(), format!("Failed to create config file: {}", e))
    })?;

    file.write_all(json.as_bytes()).map_err(|e| {
        std::io::Error::new(e.kind(), format!("Failed to write to config file: {}", e))
    })?;

    Ok(())
}

/// Fixes the configuration by ensuring that the `patterns` field is not empty.
/// If the `patterns` field is empty, it adds a default pattern and writes the updated configuration to the file.
///
/// # Errors
/// This function will return an error if there is a problem writing the configuration file.
fn fix_config(config: &mut Config) -> Result<(), std::io::Error> {
    if config.patterns.is_empty() {
        let default_pattern = vec![Step {
            dx: 0,
            dy: 0,
            duration: 1000,
        }];

        config
            .patterns
            .insert(String::from("default"), default_pattern);

        // Handle potential error from write_config
        if let Err(e) = write_config(config) {
            eprintln!("Failed to write config: {}", e);
            return Err(e);
        }
    }
    Ok(())
}
