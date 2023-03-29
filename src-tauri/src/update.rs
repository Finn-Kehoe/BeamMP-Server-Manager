use std::process::Command;
use std::collections::HashMap;

use reqwest;

fn get_current_server_version() -> String {
    let raw_version = if cfg!(target_os = "windows") {
        Command::new(r".\BeamMP-Server.exe --version")
            .output()
            .expect("could not get server version")
            .stdout
    } else {
        let mut server_name = String::new();
        let this_dir = std::fs::read_dir(std::env::current_dir().unwrap()).unwrap();
        for file in this_dir {
            let unwrapped_file = file.unwrap();
            if unwrapped_file.file_name().to_str().unwrap().contains("BeamMP-Server-") {
                server_name = unwrapped_file.file_name().to_str().unwrap().to_string();
                break;
            }
        }

        Command::new(format!("./{} --version", server_name))
            .output()
            .expect("could not get server version")
            .stdout
    };

    let mut raw_string_version = String::from_utf8(raw_version).unwrap();
    raw_string_version.replace("BeamMP-Server v", "");

    raw_string_version
}

pub async fn get_latest_server_version() {
    let response = reqwest::get("https://api.github.com/repos/BeamMP/BeamMP-Server/releases/latest")
        .await
        .unwrap()
        .json::<HashMap<String, String>>()
        .await
        .unwrap();

    println!("version: {}", response.get("name").unwrap())
}
/*
fn needs_update(local_version: String, latest_version: String) -> bool {

}
*/