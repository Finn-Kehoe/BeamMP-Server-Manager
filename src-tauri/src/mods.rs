use std::collections::HashMap;
use std::io::{Result, prelude::*};
use std::fs;

use zip;
use serde_json;
use regex::Regex;

#[derive(PartialEq, Debug, Clone, Copy, serde::Serialize)]
pub enum ModType {
    Vehicle,
    Map,
    None
}

#[derive(serde::Serialize, Clone)]
pub struct Mod {
    pub mod_type: ModType,
    pub internal_name: String,
    pub mod_file_name: String,
    pub details: HashMap<String, String>,
}

impl Mod {
    pub fn new(mod_type: ModType, internal_name: String, mod_file_name: String, details: HashMap<String, String>) -> Self {
        if mod_type == ModType::Map {
            Self {mod_type, internal_name, mod_file_name, details}
        } else {
            Self {mod_type, internal_name, mod_file_name, details}
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
pub fn get_mod_vehicles(state: tauri::State<'_, ModList>) -> Option<Vec<Mod>> {
    let mut modded_vehicles: Vec<Mod> = Vec::new();

    for i in state.mods.iter() {
        if i.mod_type == ModType::Vehicle {
            modded_vehicles.push(i.clone())
        }
    }

    return if !modded_vehicles.is_empty() {
        Some(modded_vehicles)
    } else {
        None
    }
}

#[tauri::command]
pub fn get_mod_maps(state: tauri::State<'_, ModList>) -> Option<Vec<Mod>> {
    let mut modded_maps: Vec<Mod> = Vec::new();

    for i in state.mods.iter() {
        if i.mod_type == ModType::Map {
            modded_maps.push(i.clone())
        }
    }

    return if !modded_maps.is_empty() {
        Some(modded_maps)
    } else {
        None
    }
}

// TODO: deactivate and reactivate mods

fn get_list_of_mods() -> Result<Vec<String>> {
    let mut mods_list: Vec<String> = Vec::new();

    // getting path to base mods folder
    let mut mods_folder_path = std::env::current_dir()?.clone();
    mods_folder_path.push("Resources");
    mods_folder_path.push("Client");

    if mods_folder_path.is_dir() {
        let raw_mods_list = fs::read_dir(mods_folder_path)?;
        for file in raw_mods_list {
            let mod_name = file
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .to_string();
            mods_list.push(mod_name)
        }
        Ok(mods_list)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "path to mods folder does not exist"))
    }
}

fn determine_mod_type<'a, T: Iterator<Item = &'a str>>(file_structure: &mut T) -> Option<ModType> {
    for file in file_structure {
        // only map mods contain a "levels" folder
        if file.contains("levels/") {
            return Some(ModType::Map);
        // both map and vehicle mods can contain a "vehicles" folder, so find this type by process of elimination
        } else if file.contains("vehicles/") {
            return Some(ModType::Vehicle);
        }
    }
    // if neither type is found, the mod is in an incorrect format
    return None;
}

fn find_internal_mod_name<'a, T: Iterator<Item = &'a str>>(file_structure: &mut T, mod_type: ModType) -> Option<String> {
    let mut full_internal_name = "";
    // "levels/NAME/" is what we are looking for here
    if mod_type == ModType::Map {
        match file_structure.find(|&x| x.starts_with("levels/") && x.matches("/").count() == 2) {
            Some(path) => full_internal_name = path,
            None => return None,
        }
    // "vehicles/NAME/" is what we are looking for here
    // all vehicle mods have a "common" folder that we don't care about, so we have to exclude that
    } else if mod_type == ModType::Vehicle {
        match file_structure.find(|&x| x.starts_with("vehicles/") && x.matches("/").count() == 2 && !x.contains("commmon")) {
            Some(path) => full_internal_name = path,
            None => return None,
        }
    }

    let re = Regex::new(r"/(?P<no_slash>.+)/").unwrap();
    let re_output = re.captures(full_internal_name).unwrap();
    return match re_output.name("no_slash") {
        Some(val) => Some(String::from(val.as_str())),
        None => None,
    };
}

fn read_info_file(zip_object: &mut zip::ZipArchive<std::fs::File>, path: String, mod_type: ModType) -> Result<HashMap<String, String>> {
    let mut hashmap: HashMap<String, String> = HashMap::new();
    let mut info_file = zip_object.by_name(&path)?;
    let mut string_contents = String::new();
    info_file.read_to_string(&mut string_contents)?;
    let json: serde_json::Value = serde_json::from_str(&string_contents).unwrap();

    if mod_type == ModType::Map {
        hashmap.insert(
            String::from("title"),
            json["title"].to_string(),
        );
        hashmap.insert(
            String::from("preview_image"),
            json["previews"][0].to_string(),
        );
        hashmap.insert(
            String::from("authors"),
            json["authors"].to_string(),
        );
    } else if mod_type == ModType::Vehicle {
        hashmap.insert(
            String::from("authors"),
            json["Author"].to_string(),
        );
        hashmap.insert(
            String::from("brand"),
            json["Brand"].to_string(),
        );
        hashmap.insert(
            String::from("name"),
            json["Name"].to_string(),
        );
    }

    Ok(hashmap)

}

fn examine_mod(mod_name: String) -> Result<Mod> {
    // getting path to mod (the base directory of mod)
    let mut mod_path = std::env::current_dir()?.clone();
    mod_path.push("Resources");
    mod_path.push("Client");
    mod_path.push(&mod_name);
    
    let mut zip_archive = zip::ZipArchive::new(fs::File::open(&mod_path)?)?;
    let mut file_structure = zip_archive.file_names();

    let mod_type = determine_mod_type(&mut file_structure).unwrap();

    let internal_name = find_internal_mod_name(&mut file_structure, mod_type).unwrap();
    drop(file_structure);

    let mut info_file_path = String::new();
    if mod_type == ModType::Map {
        info_file_path = format!("levels/{}/info.json", &internal_name);
    } else if mod_type == ModType::Vehicle {
        info_file_path = format!("vehicles/{}/info.json", &internal_name);
    }
    let mod_info: HashMap<String, String> = read_info_file(&mut zip_archive, info_file_path, mod_type)?;

    Ok(Mod::new(mod_type, internal_name, mod_name, mod_info))
}