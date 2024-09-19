#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod map_change;
mod update;
mod util;
mod server_control;
mod mods;

fn main() {
    // update::auto_update_server();
    tauri::Builder::default()
        .manage(server_control::Server::start())
        .manage(mods::content::ContentList::empty_init())
        .manage(mods::map::MapList::empty_init())
        .setup(|app| {
            let handle = app.handle();
            mods::generic::examine_mods(handle.state::<mods::content::ContentList>(), handle.state::<mods::map::MapList>());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            mods::map::get_mod_maps,
            mods::content::get_mod_content,
            mods::generic::change_mod_activation,
            mods::generic::delete_mod,
            mods::generic::add_mod,
            map_change::change_map,
            map_change::get_current_map,
            server_control::start_server,
            server_control::close_server,
            server_control::restart_server,
            server_control::check_server_status,
            util::config_file::has_authkey,
            util::config_file::add_authkey,
        ])
        .build(tauri::generate_context!())
        .expect("error building tauri app")
        .run(|handle,event| match event {
            tauri::RunEvent::Exit => {
                match server_control::close_server(handle.state::<server_control::Server>()) {
                    Ok(_) => (),
                    Err(e) => {
                        // access denied error means that the server was already closed, so we don't care about it
                        if !e.to_string().contains("Access is denied") {
                            panic!("Error closing BeamMP server: {e}");
                        }
                    }
                };
            }
            _ => ()
        });
}
