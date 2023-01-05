#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod mods;
mod map_change;
mod update;
mod util;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    println!("{:?}", mods::get_mods());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, mods::get_mods])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
