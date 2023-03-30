use std::collections::HashMap;
use std::io::prelude::*;
use std::fs;

use crate::mods::{ModList, ModType};

fn get_internal_map_name(external_name: String, state: tauri::State<ModList>) -> String {
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

    return internal_name;
}

#[tauri::command]
pub fn change_map(map_name: String, state: tauri::State<ModList>) -> bool {
    let internal_map_name = get_internal_map_name(map_name, state);

    let full_map_path = format!("/levels/{}/info.json", internal_map_name);

    let mut server_config_file_path = std::env::current_dir().unwrap();
    server_config_file_path.push("ServerConfig.toml");

    return if server_config_file_path.is_file() {
        let mut config_contents = String::new();
        let mut new_config_contents = String::new();
        let mut file = fs::File::open(&server_config_file_path).unwrap();

        file.read_to_string(&mut config_contents).expect("unable to read server config file");

        for line in config_contents.lines() {
            if line.starts_with("Map = ") {
                new_config_contents += &format!("Map = \"{}\"\n", full_map_path);
            } else {
                new_config_contents += &format!("{}\n", line);
            }
        }

        let mut new_file = fs::File::create(server_config_file_path).unwrap();
        new_file.write_all(new_config_contents.as_bytes()).expect("unable to write to server config file");

        true
    } else {
        false
    }
}