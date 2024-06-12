use regex::Regex;

use crate::util::{error::Error, config_file};

#[tauri::command]
pub fn change_map(map_name: String) -> Result<(), Error> {
    let full_map_path = format!("/levels/{}/info.json", map_name);

    config_file::change_server_config_value(String::from("Map"), full_map_path, config_file::ConfigTable::General)?;

    Ok(())
}

#[tauri::command]
pub fn get_current_map() -> Result<String, Error> {
    let full_string: String;
    match config_file::get_server_config_value(String::from("Map"), config_file::ConfigTable::General) {
        Ok(val) => full_string = String::from(val.as_str().unwrap()),
        Err(e) => return Err(Error::from(e)),     
    };

    let re = Regex::new(r"/levels/(?P<map>[^/]+)/info\.json").unwrap();
    let re_output = re.captures(&full_string).unwrap();
    return match re_output.name("map") {
        Some(val) => Ok(String::from(val.as_str())),
        None => Err(Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "map name not found"))),
    };
}