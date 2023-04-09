use std::collections::HashMap;

use crate::mods::{ModList, ModType};
use crate::util::{error::Error, config_file};

fn get_internal_map_name(external_name: String, state: tauri::State<ModList>) -> Option<String> {
    let STANDARD_MAPS: HashMap<String, String> = HashMap::from([
        ("Gridmap v2".to_string(), "gridmap_v2".to_string()),
        ("Johnson Valley".to_string(), "johnson_valley".to_string()),
        ("Automation Test Track".to_string(), "automation_test_track".to_string()),
        ("East Coast, USA".to_string(), "east_coast_usa".to_string()),
        ("Hirochi Raceway".to_string(), "hirochi_raceway".to_string()),
        ("Italy".to_string(), "italy".to_string()),
        ("Jungle Rock Island".to_string(), "jungle_rock_island".to_string()),
        ("Industrial Site".to_string(), "industrial".to_string()),
        ("Small Island, USA".to_string(), "small_island".to_string()),
        ("Grid, Small, Pure".to_string(), "smallgrid".to_string()),
        ("Utah, USA".to_string(), "utah".to_string()),
        ("West Coast, USA".to_string(), "west_coast_usa".to_string()),
        ("ETK Driver Experience Center".to_string(), "driver_training".to_string()),
        ("Derby Arenas".to_string(), "derby".to_string()),
    ]);

    let mut internal_name = String::new();

    match STANDARD_MAPS.get(&external_name) {
        Some(_name) => internal_name = _name.to_string(),
        None => {
            for i in state.mods.iter() {
                if external_name == i.external_name && i.mod_type == ModType::Map {
                    internal_name = i.internal_name.clone();
                    break;
                }
            }
        }
    }
    return if internal_name != String::new() {
        Some(internal_name)
    } else {
        None
    }
}

#[tauri::command]
pub fn change_map(map_name: String, state: tauri::State<ModList>) -> Result<(), Error> {
    let internal_map_name = match get_internal_map_name(map_name, state) {
        Some(name) => name,
        None => return Err(Error::from(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid map name"))),
    };

    let full_map_path = format!("/levels/{}/info.json", internal_map_name);

    config_file::change_server_config_value(String::from("Map"), full_map_path)?;

    Ok(())
}