#![allow(non_snake_case, dead_code)]
// to deserialize toml into a struct, all fields must be present and named exactly as they are in the TOML file
// this causes annoying warnings that this attribute removes

use std::io::{self, prelude::*};
use std::process::Command;
use std::fs;

use crate::{util::error, settings::file_interaction};

#[derive(serde::Deserialize, Debug)]
struct FullServerSettings {
    General: General,
    Misc: Misc,
}

#[derive(serde::Deserialize, Debug)]
struct General {
    Name: String,
    Port: u16,
    AuthKey: String,
    LogChat: bool,
    Tags: String,
    Debug: bool,
    Private: bool,
    MaxCars: u16,
    MaxPlayers: u16,
    Map: String,
    ResourceFolder: String,
    Description: String,
}

#[derive(serde::Deserialize, Debug)]
struct Misc {
    ImScaredOfUpdates: bool,
    SendErrorsShowMessage: bool,
    SendErrors: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ServerSettings {
    server_name: String,
    auth_key: String,
    is_private: bool,
    max_players: u16,
    max_cars: u16,
}

fn read_server_config_raw() -> io::Result<FullServerSettings> {
    let mut config_file_path = std::env::current_dir()?;
    config_file_path.push("ServerConfig.toml");

    return if config_file_path.is_file() {
        let mut config_file = fs::File::open(&config_file_path)?;
        let mut config_contents_string = String::new();

        config_file.read_to_string(&mut config_contents_string)?;

        let config_contents_toml: FullServerSettings = toml::from_str(&config_contents_string).unwrap();

        Ok(config_contents_toml)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "server configuration file could not be found"))
    }
}

#[tauri::command]
pub fn read_server_settings() -> Result<ServerSettings, error::Error> {
    let raw_settings = read_server_config_raw()?;
    
    Ok(ServerSettings { 
        server_name: raw_settings.General.Name, 
        auth_key: raw_settings.General.AuthKey, 
        is_private: raw_settings.General.Private,
        max_players: raw_settings.General.MaxPlayers,
        max_cars: raw_settings.General.MaxCars
    })
}

#[tauri::command]
pub fn update_server_config(key: String, value: file_interaction::ConfigInput) -> Result<(), error::Error> {
    let mut server_config_path = std::env::current_dir()?;
    server_config_path.push("ServerConfig.toml");

    if server_config_path.exists() {
        let mut config_file = fs::File::open(&server_config_path)?;
        let mut config_contents_string = String::new();

        config_file.read_to_string(&mut config_contents_string)?;

        let mut parsed_settings = config_contents_string.parse::<toml::Table>().unwrap();
        match value {
            file_interaction::ConfigInput::Str(s) => parsed_settings["General"][&key] = toml::Value::try_from(s).unwrap(),
            file_interaction::ConfigInput::Bool(b) => parsed_settings["General"][&key] = toml::Value::try_from(b).unwrap(),
            file_interaction::ConfigInput::Num(n) => parsed_settings["General"][&key] = toml::Value::try_from(n).unwrap(),
        }

        let mut new_config_file = fs::File::create(&server_config_path)?;
        new_config_file.write_all(parsed_settings.to_string().as_bytes())?;

        Ok(())
    } else {
        Err(error::Error::from(io::Error::new(io::ErrorKind::NotFound, "server configuration file could not be found")))
    }
}

#[tauri::command]
pub fn user_open_config_file() {
    let mut config_path = std::env::current_dir().unwrap();
    config_path.push(std::path::PathBuf::from("ServerConfig.toml"));
    if cfg!(target_os = "windows") {
        // "start" command only works in cmd environment
        Command::new("cmd").args(["/C", "start", "", config_path.to_str().unwrap()]).spawn().expect("");
    } else {
        Command::new("xdg-open").args([config_path.to_str().unwrap()]).spawn().expect("");
    }
}