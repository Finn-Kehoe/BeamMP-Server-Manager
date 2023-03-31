use std::process::Command;
use std::io;

use reqwest;
use serde_json;

fn get_current_server_version() -> io::Result<String> {
    let raw_version = if cfg!(target_os = "windows") {
        Command::new(r".\BeamMP-Server.exe --version")
            .output()
            .expect("could not get server version")
            .stdout
    } else {
        let mut server_name = String::new();
        let this_dir = std::fs::read_dir(std::env::current_dir()?)?;
        for file in this_dir {
            let unwrapped_file = file?;
            if unwrapped_file.file_name().to_str().unwrap().contains("BeamMP-Server-") {
                server_name = unwrapped_file.file_name()
                    .to_str()
                    .unwrap()
                    .to_string();
                break;
            }
        }

        Command::new(format!("./{} --version", server_name))
            .output()
            .expect("could not get server version")
            .stdout
    };

    let raw_string_version = String::from_utf8(raw_version).expect("unable to convert server version to string");
    let string_version = raw_string_version.replace("BeamMP-Server v", "");

    Ok(string_version)
}

pub fn get_latest_server_version() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("BeamMP-Server-Manager")
        .build()?;
    let response = client.get("https://api.github.com/repos/BeamMP/BeamMP-Server/releases/latest")
        .send()?
        .text()?;
    let json_response: serde_json::Value = serde_json::from_str(&response)?;

    let raw_name = json_response["name"].to_string();
    let stripped_name = raw_name[1..].to_string();

    Ok(stripped_name)
}

fn get_numbers_from_version(version: String) -> Vec<u16> {
    let mut numbers: Vec<u16> = Vec::new();
    let mut current_number = String::new();
    for chr in version.chars() {
        if chr != '.' {
            current_number.push(chr);
        } else {
            numbers.push(current_number.parse::<u16>().unwrap());
            current_number.clear();
            continue;
        }
    }

    numbers
}

fn needs_update(local_version: String, latest_version: String) -> bool {
    let mut needs_update = false;
    let local_numbers = get_numbers_from_version(local_version);
    let latest_numbers = get_numbers_from_version(latest_version);

    for (i, num) in local_numbers.iter().enumerate() {
        if latest_numbers[i] > *num {
            needs_update = true;
            break;
        }
    }

    needs_update
}
