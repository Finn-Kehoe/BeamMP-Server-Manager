use crate::util::mods;


#[derive(PartialEq)]
pub enum ModType {
    Vehicle,
    Map,
    None
}

pub struct Mod {
    pub mod_type: ModType,
    pub name: String,
    pub internal_name: String,
    pub mod_file_name: String,
}

impl Mod {
    pub fn new(mod_type: ModType, name: String, internal_name: String, mod_file_name: String) -> Self {
        if mod_type == ModType::Map {
            Self {mod_type, name, internal_name, mod_file_name}
        } else {
            Self {mod_type, name, internal_name, mod_file_name}
        }
    }
}

struct ModList {
    pub mods: Vec<Mod>,
}

impl ModList {
    pub fn get_mods() -> Self {
        let mut mods: Vec<Mod> = Vec::new();
        let mod_names = mods::get_list_of_mods().unwrap();
        for _mod in mod_names {
            mods.push(mods::examine_mod(_mod).unwrap())
        }

        Self {mods}
    }
}

#[tauri::command]
pub fn get_mods() -> Vec<String> {
    let mut mod_names: Vec<String> = Vec::new();
    let mods = ModList::get_mods();
    for _mod in mods.mods {
        mod_names.push(_mod.name)
    }
    mod_names
}