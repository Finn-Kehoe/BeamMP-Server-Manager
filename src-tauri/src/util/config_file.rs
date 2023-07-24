use std::io::{self, prelude::*};
use std::fs;

use crate::util::error;

pub fn change_server_config_value<T: serde::Serialize>(key: String, value: T) -> io::Result<()> {
    let mut config_file_path = std::env::current_dir()?;
    config_file_path.push("ServerConfig.toml");

    return if config_file_path.is_file() {
        let mut config_file = fs::File::open(&config_file_path)?;
        let mut config_contents_string = String::new();

        config_file.read_to_string(&mut config_contents_string)?;

        let mut config_contents_toml = config_contents_string.parse::<toml::Table>().unwrap();
        config_contents_toml[&key] = toml::Value::try_from(value).unwrap();

        let mut new_config_file = fs::File::create(&config_file_path)?;
        new_config_file.write_all(config_contents_toml.to_string().as_bytes())?;

        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "server configuration file could not be found"))
    };
}

pub fn get_server_config_value(key: String) -> io::Result<toml::Value> {
    let mut config_file_path = std::env::current_dir()?;
    config_file_path.push("ServerConfig.toml");

    return if config_file_path.is_file() {
        let mut config_file = fs::File::open(&config_file_path)?;
        let mut config_contents_string = String::new();

        config_file.read_to_string(&mut config_contents_string)?;

        let config_contents_toml = config_contents_string.parse::<toml::Table>().unwrap();

        Ok(config_contents_toml.get(&key).unwrap().clone())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "server configuration file could not be found"))
    }
}

#[tauri::command]
pub fn has_authkey() -> Result<bool, error::Error> {
    let seralized_authkey = get_server_config_value(String::from("AuthKey")).unwrap();
    let authkey = seralized_authkey.as_str().unwrap();

    return if authkey == "" {
        Ok(false)
    } else {
        Ok(true)
    }

}

#[tauri::command]
pub fn add_authkey(key: String) -> Result<(), error::Error> {
    return match change_server_config_value(String::from("AuthKey"), key) {
        Ok(_) => Ok(()),
        Err(e) => Err(error::Error::from(e)),
    };
}