use std::io::Write;

use crate::app_state::{Config, Step};

/// Reads the configuration from the "../resources/config.json" file and returns a `Config` struct.
///
/// # Errors
/// This function will return an error if there is a problem reading the configuration file.
pub fn read_config() -> Result<Config, std::io::Error> {
    let file = std::fs::File::open("../resources/config.json")?;
    let mut config: Config = serde_json::from_reader(file)?;

    fix_config(&mut config);

    Ok(config)
}

/// Writes the provided `Config` struct to the "../resources/config.json" file.
///
/// # Errors
/// This function will return an error if there is a problem writing the configuration file.
pub fn write_config(config: &mut Config) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(config)?;

    fix_config(config);

    let mut file = std::fs::File::create("../resources/config.json")?;

    file.write_all(json.as_bytes())?;

    Ok(())
}

/// Fixes the configuration by ensuring that the `patterns` map has at least one entry.
/// If the `patterns` map is empty, it adds a default pattern with a single step that has a duration of 1000 milliseconds and no movement (dx=0, dy=0).
/// It then writes the updated configuration to the "../resources/config.json" file.
///
/// # Arguments
/// * `config` - A mutable reference to the `Config` struct to be fixed.
fn fix_config(config: &mut Config) {
    let first_pattern = config.patterns.values().next();

    if first_pattern.is_none() {
        let default_pattern = vec![Step {
            dx: 0,
            dy: 0,
            duration: 1000,
        }];

        config
            .patterns
            .insert(String::from("default"), default_pattern.clone());

        write_config(config).unwrap();
    }
}
