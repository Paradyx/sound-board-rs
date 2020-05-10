use std::path::{Path};
use config::{ConfigError, Config, File};
use std::collections::HashMap;
use serde::Deserialize;
use std::{process, env};
use dirs;


#[derive(Debug, Deserialize)]
pub struct Settings {
    pub fps: u8,
    pub bindings: HashMap<String, BindingConfig>,
    pub tracks: HashMap<String, TrackConfig>,

}

#[derive(Debug, Deserialize)]
pub struct BindingConfig {
    pub binding: String,
}

#[derive(Debug, Deserialize)]
pub struct TrackConfig {
    pub path: String,
    pub mode: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::new();

        // Default settings
        settings.set("fps", "60")?;

        // Load user config
        if let Some(mut config_dir) = dirs::config_dir() {
            config_dir.push("sound_board");
            config_dir.push("config.toml");
            if config_dir.exists() {
                settings.merge(File::from(config_dir.clone())).expect(format!("Failed to read config file {}!", config_dir.display()).as_str());
            } else { eprintln!("No user config file found!"); }
        } else { eprintln!("No config directory found!"); }


        // Load project config
        let mut args: Vec<String> = env::args().collect();
        assert_eq!(args.len(), 2, "sound_board expects exactly one argument!\nUsage: soundboard path/to/project/config.toml");
        let arg = &args.pop().expect("Failed to get config path from args");
        let project_config_path = Path::new(arg);

        if project_config_path.exists() {
            settings.merge(File::from(project_config_path)).expect(format!("Failed to read config file {}!", project_config_path.display()).as_str());
        } else {
            eprintln!("Config file {} not found!", project_config_path.display());
            process::exit(1);
        }
        //println!("settings: {:?}", s);
        settings.try_into()
    }
}