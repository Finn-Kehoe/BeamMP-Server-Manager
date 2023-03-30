use std::io::{Result, prelude::*};
use std::fs;

use zip;
use serde_json;

#[derive(PartialEq, Debug)]
pub enum ModType {
    Vehicle,
    Map,
    None
}

pub struct Mod {
    pub mod_type: ModType,
    pub external_name: String,
    pub internal_name: String,
    pub mod_file_name: String,
}

impl Mod {
    pub fn new(mod_type: ModType, external_name: String, internal_name: String, mod_file_name: String) -> Self {
        if mod_type == ModType::Map {
            Self {mod_type, external_name, internal_name, mod_file_name}
        } else {
            Self {mod_type, external_name: internal_name.clone(), internal_name, mod_file_name}
        }
    }
}

pub struct ModList {
    pub mods: Vec<Mod>,
}

impl ModList {
    pub fn init() -> Self {
        let mut mods: Vec<Mod> = Vec::new();
        let mod_names = get_list_of_mods().unwrap();
        for _mod in mod_names {
            mods.push(examine_mod(_mod).unwrap())
        }

        Self {mods}
    }

    pub fn refresh(&mut self) {
        self.mods.clear();
        let mod_names = get_list_of_mods().unwrap();
        for _mod in mod_names {
            self.mods.push(examine_mod(_mod).unwrap())
        }
    }
}

#[tauri::command]
pub fn get_mod_vehicles(state: tauri::State<'_, ModList>) -> Option<Vec<String>> {
    let mut modded_vehicles: Vec<String> = Vec::new();

    for i in state.mods.iter() {
        if i.mod_type == ModType::Vehicle {
            modded_vehicles.push(i.external_name.clone())
        }
    }

    if !modded_vehicles.is_empty() {
        return Some(modded_vehicles);
    } else {
        return None;
    }
}

#[tauri::command]
pub fn get_mod_maps(state: tauri::State<'_, ModList>) -> Option<Vec<String>> {
    let mut modded_maps: Vec<String> = Vec::new();

    for i in state.mods.iter() {
        if i.mod_type == ModType::Map {
            modded_maps.push(i.external_name.clone())
        }
    }

    if !modded_maps.is_empty() {
        return Some(modded_maps);
    } else {
        return None;
    }
}

fn get_list_of_mods() -> Result<Vec<String>> {
    let mut final_mods_list: Vec<String> = Vec::new();

    // getting path to base mods folder
    let mut mods_path = std::env::current_dir()?.clone();
    mods_path.push("Resources");
    mods_path.push("Client");

    if mods_path.is_dir() {
        let raw_mods_list = fs::read_dir(mods_path)?;
        for file in raw_mods_list {
            let mod_name = file
                .unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            final_mods_list.push(mod_name)
        }
        Ok(final_mods_list)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "path to mods folder does not exist"))
    }
}

fn get_internal_mod_name(file_path: &str) -> Result<String> {
    let mut first_slash_found = false;
    let mut mod_name = String::new();
    for chr in file_path.chars() {
        if chr == '/' && first_slash_found == false {
            first_slash_found = true;
            continue;
        }
        if first_slash_found == true && chr == '/' {
            break;
        }
        if first_slash_found == true && chr != '/' {
            mod_name.push(chr);
        }
    }
    if mod_name != String::new() {
        Ok(mod_name)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "unable to find mod's internal name"))
    }
}

fn examine_mod(mod_name: String) -> Result<Mod> {
    let mut internal_name = String::new();
    let mut external_name = String::new();
    let mut mod_type = ModType::None;

    // getting path to mod (the base directory of mod)
    let mut mod_path = std::env::current_dir()?.clone();
    mod_path.push("Resources");
    mod_path.push("Client");
    mod_path.push(&mod_name);
    
    let mut pathfinding_zip = zip::ZipArchive::new(fs::File::open(&mod_path)?)?;
    for file in pathfinding_zip.file_names() {
        // determining mod type based on internal folder names
        if mod_type == ModType::None {
            if file.contains("levels/") {
                mod_type = ModType::Map;
            } else if file.contains("vehicles/") && mod_type != ModType::Map {
                mod_type = ModType::Vehicle;
            }

        }
        if mod_type == ModType::Map {
            if file.starts_with("levels/") && file.ends_with("/") && internal_name == "" {
                internal_name = get_internal_mod_name(file)?;
            }
            if internal_name != "" {
                // creating path to internal "info.json" file
                let mut info_file_path = String::from("levels/");
                info_file_path.push_str(&internal_name);
                info_file_path.push_str("/info.json");
                
                // reading "info.json" and converting into json object
                let mut reading_zip = zip::ZipArchive::new(fs::File::open(&mod_path)?)?;
                let mut string_json = String::new();
                reading_zip.by_name(&info_file_path)?.read_to_string(&mut string_json)?;
                let json: serde_json::Value = serde_json::from_str(&string_json).unwrap();

                external_name = json["title"].to_string();
                break;
            }
        } else if mod_type == ModType::Vehicle {
            if file.starts_with("vehicles/") && file.ends_with("/") && internal_name == "" {
                let temp_map_name = get_internal_mod_name(file)?;
                // there is a "common" folder in every vehicle mod that we don't want
                if temp_map_name != "common" {
                    internal_name = temp_map_name;
                    break;
                }
            }
        } else {
            continue;
        }
    }

    Ok(Mod::new(mod_type, external_name, internal_name, mod_name))
}