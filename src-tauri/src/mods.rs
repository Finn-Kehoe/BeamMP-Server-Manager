use std::io::{Result, prelude::*};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::fs;

use crate::util::json;

use zip;
use serde_json;
use regex::Regex;

#[derive(PartialEq, Debug, Clone, Copy, serde::Serialize)]
pub enum ModType {
    Vehicle,
    Map,
}

#[derive(serde::Serialize, Clone, PartialEq)]
pub struct Mod {
    pub mod_type: ModType,
    pub is_active: bool,
    pub internal_name: String,
    pub file_name: String,
    pub details: HashMap<String, String>,
}

impl Mod {
    pub fn new(mod_type: ModType, is_active: bool, internal_name: String, mod_file_name: String, details: HashMap<String, String>) -> Self {
        Self {mod_type, is_active, internal_name, file_name: mod_file_name, details}
    }

    pub fn change_activation(&mut self) {
        self.is_active = !self.is_active;
    }
}

pub struct ModList {
    pub mods: Mutex<Vec<Mod>>,
}

impl ModList {
    pub fn init() -> Self {
        let mut mods: Vec<Mod> = Vec::new();
        let mod_names = get_list_of_mods().unwrap();
        for _mod in mod_names {
            mods.push(examine_mod(_mod.0, _mod.1).unwrap());
        }

        Self {mods: Mutex::new(mods)}
    }

    pub fn refresh(&mut self) {
        let stored_mods_vec: Vec<String> = self.mods.lock().unwrap()
            .iter()
            .map(|Mod { ref file_name, .. }| file_name)
            .cloned()
            .collect();
        let current_mods_vec = get_list_of_mods().unwrap().keys().cloned().collect::<Vec<String>>();

        let stored_mods_hash: HashSet<String> = HashSet::from_iter(stored_mods_vec.into_iter());
        let current_mods_hash: HashSet<String> = HashSet::from_iter(current_mods_vec.into_iter());

        let removed_mods = stored_mods_hash.difference(&current_mods_hash).collect::<Vec<&String>>();
        let added_mods = current_mods_hash.difference(&stored_mods_hash).collect::<Vec<&String>>();

        // TODO: finish implementing

        let mut mods_vec = self.mods.lock().unwrap();
        mods_vec.clear();
        let mod_names = get_list_of_mods().unwrap();
        for _mod in mod_names {
            mods_vec.push(examine_mod(_mod.0, _mod.1).unwrap())
        }
    }
}

#[tauri::command]
pub fn get_mod_vehicles(state: tauri::State<'_, ModList>) -> Option<Vec<Mod>> {
    let mut modded_vehicles: Vec<Mod> = Vec::new();

    for i in state.mods.lock().unwrap().iter() {
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

    for i in state.mods.lock().unwrap().iter() {
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

#[tauri::command]
pub fn change_mod_activation(internal_name: String, state: tauri::State<ModList>) {
    let mut mod_list = state.mods.lock().unwrap();
    let this_mod: &mut Mod;
    match mod_list.iter_mut().find(|x| x.internal_name == internal_name) {
        Some(found_mod) => this_mod = found_mod,
        None => return,
    }

    _change_mod_activation(this_mod);
}

pub fn _change_mod_activation(this_mod: &mut Mod) {

    let current_dir = std::env::current_dir().unwrap().clone();

    let mut active_path = current_dir.clone();
    active_path.push(format!("Resources/Client/{}", &this_mod.file_name));

    let mut inactive_path = current_dir.clone();
    inactive_path.push(format!("Resources/Inactive/{}", &this_mod.file_name));

    if this_mod.is_active {
        // moving mod from active directory to inactive directory
        fs::rename(active_path, inactive_path).unwrap();
    } else {
        // moving mod from inactive directory to active directory
        fs::rename(inactive_path, active_path).unwrap();
    }
    this_mod.change_activation();
}

#[tauri::command]
pub fn delete_mod(internal_name: String, state: tauri::State<ModList>) {
    let mut mod_list = state.mods.lock().unwrap();
    let this_mod: &Mod;
    match mod_list.iter().find(|x| x.internal_name == internal_name) {
        Some(found_mod) => this_mod = found_mod,
        None => return,
    }

    let current_dir = std::env::current_dir().unwrap().clone();

    let mut active_path = current_dir.clone();
    active_path.push(format!("Resources/Client/{}", &this_mod.file_name));

    let mut inactive_path = current_dir.clone();
    inactive_path.push(format!("Resources/Inactive/{}", &this_mod.file_name));

    if this_mod.is_active {
        fs::remove_file(active_path).unwrap();
    } else {
        fs::remove_file(inactive_path).unwrap();
    }

    let cloned_mod = this_mod.clone();
    mod_list.retain(|x| *x != cloned_mod);
}

fn get_list_of_mods() -> Result<HashMap<String, bool>> {
    let mut mods_list: HashMap<String, bool> = HashMap::new(); // mod name, active status

    // getting path to active mods folder (./Resources/Client)
    let mut active_mods_path = std::env::current_dir()?.clone();
    active_mods_path.push("Resources");
    active_mods_path.push("Client");

    if active_mods_path.is_dir() {
        let raw_mods_list = fs::read_dir(active_mods_path)?;
        for file in raw_mods_list {
            let mod_name = file
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .to_string();
            mods_list.insert(mod_name, true);
        }
    } else {
       return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "path to mods folder does not exist"));
    }

    // getting path to inactive mods folder (./Resources/Inactive)
    let mut inactive_mods_path = std::env::current_dir()?.clone();
    inactive_mods_path.push("Resources");
    inactive_mods_path.push("Inactive");

    if inactive_mods_path.is_dir() {
        let raw_mods_list = fs::read_dir(inactive_mods_path)?;
        for file in raw_mods_list {
            let mod_name = file
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .to_string();
            mods_list.insert(mod_name, false);
        }
    } else {
        // this folder does not exist in the regular configuration of the BeamMP Server, so if it does not already exist, we create it
        fs::create_dir(inactive_mods_path)?;
    }

    Ok(mods_list)
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

    // capturing group is used so only characters in-between slashes are captured
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
    let mut raw_json = String::new();
    info_file.read_to_string(&mut raw_json)?;
    // try to deserialize info json file (json can potentially be incorrectly formatted)
    let json: serde_json::Value = match serde_json::from_str(&raw_json) {
        Ok(json) => json,
        // if deserialization fails, preprocess it and reattempt to deseralize
        Err(_) => {
            serde_json::from_str(&json::preprocess_json(raw_json)).unwrap()
        }
    };

    if mod_type == ModType::Map {
        hashmap.insert(
            String::from("title"),
            json["title"].as_str().unwrap().to_string(),
        );
        hashmap.insert(
            String::from("preview_image"),
            json["previews"].as_array().unwrap()[0].to_string(),
        );
        hashmap.insert(
            String::from("authors"),
            json["authors"].as_str().unwrap().to_string(),
        );
    } else if mod_type == ModType::Vehicle {
        hashmap.insert(
            String::from("authors"),
            json["Author"].as_str().unwrap().to_string(),
        );
        hashmap.insert(
            String::from("brand"),
            json["Brand"].as_str().unwrap().to_string(),
        );
        hashmap.insert(
            String::from("name"),
            json["Name"].as_str().unwrap().to_string(),
        );
    }

    Ok(hashmap)

}

fn examine_mod(mod_name: String, is_active: bool) -> Result<Mod> {
    let mut mod_path = std::env::current_dir()?.clone();
    mod_path.push("Resources");
    if is_active {
        mod_path.push("Client");
    } else {
        mod_path.push("Inactive");
    }
    mod_path.push(&mod_name);
    
    let mut zip_archive = zip::ZipArchive::new(fs::File::open(&mod_path)?)?;
    let mut file_structure = zip_archive.file_names();

    let mod_type = determine_mod_type(&mut file_structure).unwrap();

    let internal_name = find_internal_mod_name(&mut file_structure, mod_type).unwrap();
    // file_structure is dropped so we can reference the ZipArchive object elsewhere
    drop(file_structure);

    let mut info_file_path = String::new();
    if mod_type == ModType::Map {
        info_file_path = format!("levels/{}/info.json", &internal_name);
    } else if mod_type == ModType::Vehicle {
        info_file_path = format!("vehicles/{}/info.json", &internal_name);
    }
    let mod_info: HashMap<String, String> = read_info_file(&mut zip_archive, info_file_path, mod_type)?;

    Ok(Mod::new(mod_type, is_active, internal_name, mod_name, mod_info))
}