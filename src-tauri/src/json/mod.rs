use std::collections::HashMap;
use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::app_state::PatternPart;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub patterns: HashMap<String, Vec<PatternPart>>,
}

pub fn read_config() -> Result<Config, std::io::Error> {
    let file = std::fs::File::open("../config.json")?;
    let config = serde_json::from_reader(file)?;

    Ok(config)
}

pub fn write_config(config: Config) -> Result<(), std::io::Error> {
    let file = std::fs::File::create("../config.json")?;
    let json = serde_json::to_string_pretty(&config)?;

    // Use a buffered writer for efficient writing
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(json.as_bytes())?;
    writer.flush()?;

    Ok(())
}
