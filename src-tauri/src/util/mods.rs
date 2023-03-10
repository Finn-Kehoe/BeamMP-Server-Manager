use std::io::{Result, prelude::*};
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

use crate::mods::{Mod, ModType};

use zip;
use serde_json;

pub fn get_list_of_mods() -> Result<Vec<String>> {
    let mut mods_list: Vec<String> = Vec::new();
    let cwd = std::env::current_dir()?;
    let mut mods_path = cwd.clone();
    mods_path.push("Resources");
    mods_path.push("Client");
    if mods_path.is_dir() {
        let raw_list = fs::read_dir(mods_path)?;

        for file in raw_list {
            let mod_name = file
                .unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            mods_list.push(mod_name)
        }

        println!("{:?}", mods_list);
        Ok(mods_list)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "path to mods folder does not exist"))
    }
}

pub fn examine_mod(mod_name: String) -> Result<Mod> {
    let mut internal_name = String::new();
    let mut external_name = String::new();
    let mut mod_type = ModType::None;

    let mut mod_path = std::env::current_dir()?
        .clone();
    mod_path.push("Resources");
    mod_path.push("Client");
    mod_path.push(&mod_name);
    
    let mut temp_path = mod_path.clone();
    temp_path.set_file_name("temp_mod_folder");
    println!("{}", mod_path.display());
    let mut mod_zip = zip::ZipArchive::new(fs::File::open(&mod_path)?)?;
    let mut mod_zip_two = zip::ZipArchive::new(fs::File::open(&mod_path)?)?;
    // mod_zip.extract(&temp_path)?;
    println!("here1");
    //let mut info_file = mod_zip.by_name("levels/c1/info.json")?;
    for file in mod_zip.file_names() {
        if file == "levels/" {
            mod_type = ModType::Map;
        }
        if file == "vehicles/" && mod_type != ModType::Map {
            mod_type = ModType::Vehicle;
        }
        if mod_type == ModType::Map {
            if file.starts_with("levels/") && file.ends_with("/") && internal_name == "" {
                let mut first_slash_found = false;
                let mut temp_map_name = String::new();
                for chr in file.chars() {
                    if chr == '/' && first_slash_found == false {
                        first_slash_found = true;
                        continue;
                    }
                    if first_slash_found == true && chr == '/' {
                        break;
                    }
                    if first_slash_found == true && chr != '/' {
                        temp_map_name.push(chr);
                    }
                }
                internal_name = temp_map_name;
            }
            if internal_name != "" {
                println!("{}", internal_name);
                let mut info_file_path = String::from("levels/");
                info_file_path.push_str(&internal_name);
                info_file_path.push_str("/info.json");

                let mut string_json = String::new();
                let mut info_file = mod_zip_two.by_name(&info_file_path)?;
                info_file.read_to_string(&mut string_json);
                let json: serde_json::Value = serde_json::from_str(&string_json).unwrap();

                external_name = json["title"].to_string();
                break;
            }
        } else if mod_type == ModType::Vehicle {
            let mut true_name_found = false;
            if file.starts_with("vehicles/") && file.ends_with("/") && internal_name == "" {
                let mut first_slash_found = false;
                let mut temp_map_name = String::new();
                for chr in file.chars() {
                    if chr == '/' && first_slash_found == false {
                        first_slash_found = true;
                        continue;
                    }
                    if first_slash_found == true && chr == '/' {
                        break;
                    }
                    if first_slash_found == true && chr != '/' {
                        temp_map_name.push(chr);
                    }
                }
                if temp_map_name == "common" {
                    temp_map_name = String::new();
                } else {
                    true_name_found = true;
                    println!("{}", temp_map_name);
                    internal_name = temp_map_name;
                }
            }
        } else {
            continue;
        }
    }
    println!("here2");

    let final_mod = Mod::new(mod_type, external_name, internal_name, mod_name);
    return Ok(final_mod);
    //let mut string_json = String::new();
    //info_file.read_to_string(&mut string_json)?;
    //let json: serde_json::Value = serde_json::from_str(&string_json).unwrap();
    //external_name = json["title"].to_string();
    //println!(":: {}", external_name);

    /*
    // map mods
    let lvl_one_dir = fs::read_dir(&temp_path)?;
    for file in lvl_one_dir {
        let file = file?;
        if file.file_name() == "levels" {
            temp_path.push("levels");
            let lvl_two_dir = fs::read_dir(&temp_path)?;
            for file in lvl_two_dir {
                let file = file?;
                internal_name = file.file_name().to_str().unwrap().to_string();
                temp_path.push(file.file_name());
                let lvl_three_dir = fs::read_dir(&temp_path)?;
                for file in lvl_three_dir {
                    let file = file?;
                    if file.file_name() == "info.json" {
                        let mut json_bytes: Vec<u8> = Vec::new();
                        temp_path.push("info.json");
                        let mut temp_json = fs::File::open(&temp_path)?;
                        temp_json.read(&mut json_bytes)?;
                        let json: serde_json::Value = serde_json::from_slice(&*json_bytes).unwrap();
                        external_name = json["title"].to_string();
                        break;
                    }
                }
                break;
            }
            let final_mod = Mod::new(ModType::Map, external_name, Some(internal_name), mod_name);
            return Ok(final_mod);
        }
    }
    // vehicle mods
    let lvl_one_dir = fs::read_dir(&temp_path)?;
    for file in lvl_one_dir {
        let file = file?;
        if file.file_name() == "vehicles" {
            temp_path.push("vehicles");
            let lvl_two_dir = fs::read_dir(&temp_path)?;
            for file in lvl_two_dir {
                let file = file?;
                if file.file_name() != "common" {
                    external_name = file.file_name().to_str().unwrap().to_string();
                    break;
                }
            }
            let final_mod = Mod::new(ModType::Vehicle, external_name, None, mod_name);
            return Ok(final_mod)
        }
    }
    */
   return Err(std::io::Error::from(std::io::ErrorKind::Other));
}