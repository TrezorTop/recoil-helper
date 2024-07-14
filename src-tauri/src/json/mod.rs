use crate::app_state::Config;

/// Reads the configuration from the "../config.json" file and returns a `Config` struct.
///
/// # Errors
/// This function will return an error if there is a problem reading the configuration file.
pub fn read_config() -> Result<Config, std::io::Error> {
    let file = std::fs::File::open("../config.json")?;
    let config: Config = serde_json::from_reader(file)?;

    Ok(config)
}

pub fn write_config(config: &Config) -> Result<(), std::io::Error> {
    // TODO: Implement this function.

    Ok(())
}
