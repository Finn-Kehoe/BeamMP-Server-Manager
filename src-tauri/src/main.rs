#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod mods;
mod map_change;
mod update;
mod util;
mod server_control;

fn main() {
    // TODO: have beammp server shutdown with app
    tauri::Builder::default()
        .manage(server_control::Server::start())
        .manage(mods::ModList::init())
        .invoke_handler(tauri::generate_handler![
            mods::get_mod_vehicles,
            mods::get_mod_maps,
            map_change::change_map,
            server_control::close_server,
            server_control::restart_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
