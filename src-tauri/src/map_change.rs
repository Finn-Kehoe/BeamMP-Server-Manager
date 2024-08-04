use regex::Regex;

use crate::mods;
use crate::util::{error::Error, config_file};

#[tauri::command]
pub fn change_map(map_name: String, modlist: tauri::State<mods::ModList>) -> Result<(), Error> {

    for _mod in modlist.mods.lock().unwrap().iter_mut() {
        if _mod.mod_type == mods::ModType::Map {
            // if the map we are changing to isn't active, make it active
            if _mod.internal_name == map_name {
                if _mod.is_active == false {
                    mods::_change_mod_activation(_mod);
                }
            // if a map that we aren't changing to is active, make it inactive
            } else {
                if _mod.is_active == true {
                    mods::_change_mod_activation(_mod);
                }
            }
        }
    }

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