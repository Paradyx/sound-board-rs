use std::path::Path;
use config::{ConfigError, Config, File};
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub fps: u8,
    pub tracks: HashMap<String, TrackConfig>,
}

#[derive(Debug, Deserialize)]
pub struct TrackConfig {
    pub path: String,
    pub mode: String,
}

impl Settings {
    pub fn new(config: &Path) -> Result <Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::from(config));

        //println!("settings: {:?}", s);

        s.try_into()
    }
}