use std::io::{self, prelude::*};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfigTable {
    General,
    Misc,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ConfigInput {
    Str(String),
    Bool(bool),
    Num(u16),
}

/*
pub fn change_toml_value<T: serde::Serialize, P: AsRef<std::path::Path>>(file_path: P, key: Vec<String>, value: T) -> io::Result<()> {
    let mut raw_file = fs::File::open(&file_path)?;
    let mut raw_file_string = String::new();
    raw_file.read_to_string(&mut raw_file_string)?;

    let mut toml_contents = raw_file_string.parse::<toml::Table>().unwrap();
    let mut current_value: &mut toml::Value;
    for i in 0..key.len() {
        if i == 0 {
            current_value = toml_contents.get_mut(&key[i]).unwrap();
        } else {
            current_value = current_value.get_mut(&key[i]).unwrap();
        }
    }
    current_value = toml::Value::try_from(value).unwrap();

    Ok(())
}
*/

pub fn change_server_config_value<T: serde::Serialize>(key: String, value: T, table: ConfigTable) -> io::Result<()> {
    let mut config_file_path = std::env::current_dir()?;
    config_file_path.push("ServerConfig.toml");

    return if config_file_path.is_file() {
        let mut config_file = fs::File::open(&config_file_path)?;
        let mut config_contents_string = String::new();

        config_file.read_to_string(&mut config_contents_string)?;

        let mut config_contents_toml = config_contents_string.parse::<toml::Table>().unwrap();
        if table == ConfigTable::General {
            config_contents_toml["General"][&key] = toml::Value::try_from(value).unwrap();
        } else if table == ConfigTable::Misc {
            config_contents_toml["Misc"][&key] = toml::Value::try_from(value).unwrap();
        }

        let mut new_config_file = fs::File::create(&config_file_path)?;
        new_config_file.write_all(config_contents_toml.to_string().as_bytes())?;

        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "server configuration file could not be found"))
    };
}

pub fn get_server_config_value(key: String, table: ConfigTable) -> io::Result<toml::Value> {
    let mut config_file_path = std::env::current_dir()?;
    config_file_path.push("ServerConfig.toml");

    return if config_file_path.is_file() {
        let mut config_file = fs::File::open(&config_file_path)?;
        let mut config_contents_string = String::new();

        config_file.read_to_string(&mut config_contents_string)?;

        let config_contents_toml = config_contents_string.parse::<toml::Table>().unwrap();

        if table == ConfigTable::General {
            Ok(config_contents_toml["General"][&key].clone())
        } else if table == ConfigTable::Misc {
            Ok(config_contents_toml["Misc"][&key].clone())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "table type input invalid"))
        }
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "server configuration file could not be found"))
    }
}