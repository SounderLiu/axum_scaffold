use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    // additional settings...
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        let env = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)))?;
        s.try_deserialize()
    }
}