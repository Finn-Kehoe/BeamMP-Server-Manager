use std::io::{self, prelude::*};
use std::fs;

use crate::{util::error, settings::file_interaction};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ManagerSettings {
    pub auto_update: bool,
}

#[tauri::command]
pub fn read_manager_settings() -> Result<ManagerSettings, error::Error> {
    _read_manager_settings()
}

pub fn _read_manager_settings() -> Result<ManagerSettings, error::Error> {
    let mut manager_config_path = std::env::current_dir()?;
    manager_config_path.push("ManagerConfig.toml");

    if manager_config_path.exists() {
        let mut config_file = fs::File::open(&manager_config_path)?;
        let mut config_contents_string = String::new();

        config_file.read_to_string(&mut config_contents_string)?;

        let parsed_settings: ManagerSettings = toml::from_str(&config_contents_string).unwrap();
        Ok(parsed_settings)
    } else {
        // "ManagerConfig.toml" does not exist in normal running of server, so it must be created by us
        // its nonexistance implies this is a first-time run, so auto_update must be true to download the server for the first time
        let default_settings: ManagerSettings = ManagerSettings { auto_update: true };
        fs::write(manager_config_path, toml::to_string_pretty(&default_settings).unwrap())?;
        Ok(default_settings)
    }
}

#[tauri::command]
pub fn update_manager_config(key: String, value: file_interaction::ConfigInput) -> Result<(), error::Error> {
    let mut manager_config_path = std::env::current_dir()?;
    manager_config_path.push("ManagerConfig.toml");

    if manager_config_path.exists() {
        let mut config_file = fs::File::open(&manager_config_path)?;
        let mut config_contents_string = String::new();

        config_file.read_to_string(&mut config_contents_string)?;

        let mut parsed_settings = config_contents_string.parse::<toml::Table>().unwrap();
        match value {
            file_interaction::ConfigInput::Bool(b) => parsed_settings[&key] = toml::Value::try_from(b).unwrap(),
            _ => (),
        }

        let mut new_config_file = fs::File::create(&manager_config_path)?;
        new_config_file.write_all(parsed_settings.to_string().as_bytes())?;

        Ok(())
    } else {
        Err(error::Error::from(io::Error::new(io::ErrorKind::NotFound, "server configuration file could not be found")))
    }
}