use std::thread;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::settings::file_interaction;

// waiting for server to finish creating envorinment during initial setup
pub fn wait_on_first_startup() {
    // first check if it is first launch by checking for the Resources folder
    let mut resources_path = std::env::current_dir().unwrap();
    resources_path.push(PathBuf::from("Resources/"));
    // if it's there, it isn't the initial setup so function can exit
    if Path::new(&resources_path).exists() {
        return;
    }

    // if AuthKey field is empty (which implies that the ServerConfig.toml file exists), exit function so that the GUI can open
    match file_interaction::get_server_config_value(String::from("AuthKey"), file_interaction::ConfigTable::General) {
        Ok(v) => { if v.as_str().unwrap() == "" { return; } },
        Err(e) => match e.kind() {
            // NotFound error shows that ServerConfig.toml has not been created yet, so server needs more time (continue function)
            std::io::ErrorKind::NotFound => {},
            _ => return,
        },
    }

    let mut config_path = std::env::current_dir().unwrap();
    config_path.push("ServerConfig.toml");
    // check every 5 seconds for setup to finish (should take <5 seconds)
    loop {
        // if the Resources folder exists than the server is finished setting up
        if Path::new(&resources_path).exists() {
            return;
        // if the config file path doesn't exist than we have to let function end so the server can create the config file
        } if !Path::new(&config_path).exists() {
            return;
        } 
        // otherwise wait 5 seconds
        println!("waiting for server to complete setup...");
        thread::sleep(Duration::from_secs(5));
    }
}