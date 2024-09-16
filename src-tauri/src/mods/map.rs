use std::sync::Mutex;
use std::io::{self, prelude::*};

use crate::mods::generic::*;
use crate::util::json;

pub struct ProtoMap {
    pub internal_name: String,
    pub external_name: String,
    pub authors: String,
}

#[derive(serde::Serialize, Clone)]
pub struct Map {
    pub is_active: bool,
    pub file_name: String,
    pub internal_name: String,
    pub external_name: String,
    pub authors: String,
}

pub struct MapList {
    pub maps: Mutex<Vec<Map>>,
}

impl Map {
    pub fn new(is_active: bool, file_name: String, internal_name: String, external_name: String, authors: String) -> Self {
        Map { is_active, file_name, internal_name, external_name, authors }
    }
}

impl GenericMod for Map {
    fn is_active(&self) -> bool {
        self.is_active
    }
    fn change_activation(&mut self) {
        self.is_active = !self.is_active;
    }
    fn file_name(&self) -> String {
        self.file_name.clone()
    }
}

impl MapList {
    pub fn empty_init() -> Self {
        MapList { maps: Mutex::new(Vec::new()) }
    }
}

#[tauri::command]
pub fn get_mod_maps(map_list: tauri::State<MapList>) -> Vec<Map> {
    map_list.maps.lock().unwrap().to_vec()
}

fn find_internal_mod_name<'a, T: Iterator<Item = &'a str>>(file_structure: &mut T) -> String {
    let full_internal_name: &str;
    // "levels/NAME/" is what we are looking for here
    match file_structure.find(|&x| x.starts_with("levels/") && x.matches("/").count() == 2) {
        Some(path) => full_internal_name = path,
        None => return String::new(),
    }

    extract_name_from_info_path(full_internal_name.to_string())
}

fn read_info_file(zip_object: &mut zip::ZipArchive<std::fs::File>, path: String) -> io::Result<ProtoMap> {
    let mut info_file = zip_object.by_name(&path)?;
    let mut raw_json = String::new();
    info_file.read_to_string(&mut raw_json)?;

    Ok(ProtoMap {   
        internal_name: String::new(), // internal name is determined later
        external_name: json::get_value_from_json(&raw_json, String::from("title")),
        authors: json::get_value_from_json(&raw_json, String::from("authors")), 
    })
}

pub fn examine_map_mod(zip_object: &mut zip::ZipArchive<std::fs::File>) -> ProtoMap {
    let mut file_structure = zip_object.file_names();
    let internal_name = find_internal_mod_name(&mut file_structure);
    drop(file_structure);

    let info_file_path = format!("levels/{}/info.json", &internal_name);
    let mut this_protomap = read_info_file(zip_object, info_file_path).unwrap();
    this_protomap.internal_name = internal_name;

    this_protomap
}