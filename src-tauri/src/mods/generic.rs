use std::io;
use std::collections::HashMap;
use std::fs;

use crate::mods::{content::*, map::*};
use crate::util::error;

#[derive(PartialEq, Debug, Clone, Copy, serde::Serialize)]
pub enum ModType {
    Content,
    Map,
}

pub trait GenericMod {
    fn is_active(&self) -> bool;
    fn change_activation(&mut self);
    fn file_name(&self) -> String;
}

// top level function will only ever be used by content mods
#[tauri::command]
pub fn change_mod_activation(file_name: String, content_list: tauri::State<ContentList>) {
    let this_mod: &mut Content;
    let mut this_list = content_list.content_mods.lock().unwrap();
    match this_list.iter_mut().find(|x| x.file_name == file_name) {
        Some(found_mod) => this_mod = found_mod,
        None => return,
    }

    _change_mod_activation(this_mod);
}

pub fn _change_mod_activation<T: GenericMod>(this_mod: &mut T) {

    let current_dir = std::env::current_dir().unwrap().clone();

    let mut active_path = current_dir.clone();
    active_path.push(format!("Resources/Client/{}", &this_mod.file_name()));

    let mut inactive_path = current_dir.clone();
    inactive_path.push(format!("Resources/Inactive/{}", &this_mod.file_name()));

    if this_mod.is_active() {
        // moving mod from active directory to inactive directory
        fs::rename(active_path, inactive_path).unwrap();
    } else {
        // moving mod from inactive directory to active directory
        fs::rename(inactive_path, active_path).unwrap();
    }
    this_mod.change_activation();
}

#[tauri::command]
pub fn delete_mod(file_name: String, content_list: tauri::State<ContentList>, map_list: tauri::State<MapList>) {
    let mut _content_list = content_list.content_mods.lock().unwrap();
    let mut _map_list = map_list.maps.lock().unwrap();
    let this_mod: &dyn GenericMod;
    let mod_type: ModType;
    match _content_list.iter().find(|x| x.file_name == file_name) {
        Some(found_mod) => {this_mod = found_mod; mod_type = ModType::Content;},
        None => match _map_list.iter().find(|x| x.file_name == file_name) {
            Some(found_mod) => {this_mod = found_mod; mod_type = ModType::Map;},
            None => return,
        },
    }

    let current_dir = std::env::current_dir().unwrap().clone();

    let mut active_path = current_dir.clone();
    active_path.push(format!("Resources/Client/{}", &this_mod.file_name()));

    let mut inactive_path = current_dir.clone();
    inactive_path.push(format!("Resources/Inactive/{}", &this_mod.file_name()));

    if this_mod.is_active() {
        fs::remove_file(active_path).unwrap();
    } else {
        fs::remove_file(inactive_path).unwrap();
    }

    let cloned_file_name = this_mod.file_name().clone();

    if mod_type == ModType::Content {
        _content_list.retain(|x| *x.file_name != cloned_file_name);
    } else if mod_type == ModType::Map {
        _map_list.retain(|x| *x.file_name != cloned_file_name);
    }
}

#[tauri::command]
pub fn add_mod(path: String, content_list: tauri::State<ContentList>, map_list: tauri::State<MapList>) -> Result<ModType, error::Error> {
    let init_path = std::path::Path::new(&path);
    let file_name = init_path.file_name().unwrap().to_str().unwrap();

    let mut destination_path = std::env::current_dir()?;
    destination_path.push(format!("Resources/Client/{}", file_name));

    if init_path.extension().unwrap() == "zip" {
        fs::rename(init_path, &destination_path)?;
    } else {
        return Err(error::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidInput, "incorrect file type")));
    }

    let mod_type = examine_mod(String::from(file_name), true, &content_list, &map_list)?;

    // maps are always inactive unless currently loaded, and a just added mod will never be currently loaded
    // so we deactivate any new mod maps
    if mod_type == ModType::Map {
        let mut inactive_path = std::env::current_dir()?;
        inactive_path.push(format!("Resources/Inactive/{}", file_name));

        fs::rename(destination_path, inactive_path).unwrap();
    }


    Ok(mod_type)
}

fn get_list_of_mods() -> io::Result<HashMap<String, bool>> {
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
            return Some(ModType::Content);
        }
    }
    // if neither type is found, the mod is in an incorrect format
    return None;
}

pub fn extract_name_from_info_path(path: String) -> String {
    // info paths look like: vehicles/NAME/... OR levels/NAME/...
    
    // get index of each slash (/)
    let slash_indices: Vec<_> = path.match_indices("/").map(|(i, _)| i ).collect();
    // the name is between the first two slashes
    let name = &path[slash_indices[0] + 1..slash_indices[1]];

    String::from(name)
}

fn examine_mod(mod_name: String, is_active: bool, content_list: &tauri::State<ContentList>, map_list: &tauri::State<MapList>) -> io::Result<ModType> {
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
    drop(file_structure);

    if mod_type == ModType::Content {
        let inner_contents = examine_content_mod(&mut zip_archive);
        let content_mod = Content::new(is_active, mod_name, inner_contents);
        content_list.content_mods.lock().unwrap().push(content_mod);

    } else if mod_type == ModType::Map {
        let protomap = examine_map_mod(&mut zip_archive);
        let map_mod = Map::new(is_active, mod_name, protomap.internal_name, protomap.external_name, protomap.authors);
        map_list.maps.lock().unwrap().push(map_mod);
    }

    Ok(mod_type)
}

pub fn examine_mods(content_list: tauri::State<ContentList>, map_list: tauri::State<MapList>) -> () {
    let mod_names = get_list_of_mods().unwrap();
    for _mod in mod_names {
        examine_mod(_mod.0, _mod.1, &content_list, &map_list).unwrap();
    }
}