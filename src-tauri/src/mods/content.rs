use std::sync::Mutex;
use std::io::{self, prelude::*};

use crate::mods::generic::*;
use crate::util::json;

struct InfoJSON {
    authors: String,
    brand: String,
    name: String,
}

#[derive(serde::Serialize, Clone, PartialEq)]
pub struct InnerContent {
    pub internal_name: String,
    pub external_name: String,
    pub authors: String,
    pub brand: String,
    pub name: String,
}

#[derive(serde::Serialize, Clone, PartialEq)]
pub struct Content {
    pub is_active: bool,
    pub file_name: String,
    pub inner_content: Vec<InnerContent>,
}

pub struct ContentList {
    pub content_mods: Mutex<Vec<Content>>,
}

impl InnerContent {
    pub fn new(internal_name: String, external_name: String, authors: String, brand: String, name: String) -> Self {
        InnerContent { internal_name, external_name, authors, brand, name }
    }
}

impl Content {
    pub fn new(is_active: bool, file_name: String, inner_content: Vec<InnerContent>) -> Self {
        Content { is_active, file_name, inner_content }
    }
}

impl GenericMod for Content {
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

impl ContentList {
    pub fn empty_init() -> Self {
        ContentList { content_mods: Mutex::new(Vec::new()) }
    }
}

#[tauri::command]
pub fn get_mod_content(content_list: tauri::State<ContentList>) -> Vec<Content> {
    content_list.content_mods.lock().unwrap().to_vec()
}

fn get_info_file_paths<'a, T: Iterator<Item = &'a str>>(file_structure: &mut T) -> Vec<String> {
    let mut proto_paths: Vec<String> = Vec::new();
    for file in file_structure {
        if file.contains("info.json") {
            proto_paths.push(String::from(file));
        }
    }

    let mut final_paths: Vec<String> = Vec::new();
    for path in proto_paths.into_iter() {
        // weeding out potential info.jsons in folders not related to actual content
        if path.starts_with("vehicles") {
            final_paths.push(path);
        }
    }

    final_paths
}

fn read_info_file(zip_object: &mut zip::ZipArchive<std::fs::File>, path: String) -> io::Result<InfoJSON> {
    let mut info_file = zip_object.by_name(&path)?;
    let mut raw_json = String::new();
    info_file.read_to_string(&mut raw_json)?;

    Ok(InfoJSON {   
        authors: json::get_value_from_json(&raw_json, String::from("Author")), 
        brand: json::get_value_from_json(&raw_json, String::from("Brand")), 
        name: json::get_value_from_json(&raw_json, String::from("Name"))
    })
}

pub fn examine_content_mod(zip_object: &mut zip::ZipArchive<std::fs::File>) -> Vec<InnerContent> {
    let mut contents_vec: Vec<InnerContent> = Vec::new();
    let mut file_structure = zip_object.file_names();
    let info_paths = get_info_file_paths(&mut file_structure);
    drop(file_structure);

    for path in info_paths.iter() {
        let raw_info = read_info_file(zip_object, path.to_string()).unwrap();
        let internal_name = extract_name_from_info_path(path.to_string());

        contents_vec.push(InnerContent::new(internal_name, String::new(), raw_info.authors, raw_info.brand, raw_info.name))
    }

    contents_vec
}