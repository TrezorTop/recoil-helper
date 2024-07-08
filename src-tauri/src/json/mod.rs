use std::collections::HashMap;

use serde::Deserialize;

use crate::app_state::PatternPart;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub patterns: HashMap<String, Vec<PatternPart>>,
}

pub fn read_config(file_path: &str) -> Result<Config, std::io::Error> {
    let file = std::fs::File::open(file_path)?;
    let config = serde_json::from_reader(file)?;

    Ok(config)
}
