#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod mods;
mod map_change;
mod update;
mod util;

fn main() {
    tauri::Builder::default()
        .manage(mods::ModList::init())
        .invoke_handler(tauri::generate_handler![mods::get_mod_vehicles, mods::get_mod_maps, map_change::change_map])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
