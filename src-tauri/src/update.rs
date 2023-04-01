use std::process::Command;
use std::io::Write;
use std::io;
use std::fs;

use reqwest;
use serde_json;

// TODO: handle first time setup (no beammp server file exists)
fn get_current_server_version() -> io::Result<String> {
    let raw_version = if cfg!(target_os = "windows") {
        Command::new(r".\BeamMP-Server.exe --version")
            .output()
            .expect("could not get server version")
            .stdout
    } else {
        let mut server_name = String::new();
        let this_dir = fs::read_dir(std::env::current_dir()?)?;
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

pub fn download_latest_server() -> Result<(), Box<dyn std::error::Error>> {
    // creating http client
    let client = reqwest::blocking::Client::builder()
        .user_agent("BeamMP-Server-Manager")
        .build()?;
    // getting latest server release data
    let full_release = client.get("https://api.github.com/repos/BeamMP/BeamMP-Server/releases/latest")
        .send()?
        .text()?;
    // converting response data to json
    let json_full_release: serde_json::Value = serde_json::from_str(&full_release)?;
    // getting the assets array from the main json
    let release_assets = &json_full_release["assets"];

    let mut download_link = String::new();
    // looping through each asset until the correct one for the users OS is found
    for asset in release_assets.as_array().unwrap() {
        if cfg!(target_os = "windows") {
            if asset["name"].to_string().contains("BeamMP-Server.exe") {
                download_link = asset["browser_download_url"].to_string();
            }
        } else {
            if asset["name"].to_string().contains("BeamMP-Server-linux") {
                download_link = asset["browser_download_url"].to_string();
            }
        }
    }

    // download_link has quotation marks ("") that reqwest does not accept
    // so they have to be stripped off
    let trimmed_download_link = &download_link[1..download_link.len()-1];
    // downloading file from previously found link
    let downloaded_file = client.get(trimmed_download_link)
        .send()?
        .bytes()?;

    // creating the path for the server file
    let mut server_path = std::env::current_dir()?;
    if cfg!(target_os = "windows") {
        server_path.push("BeamMP-Server.exe");
    } else {
        server_path.push("BeamMP-Server-linux");
    }
    // writing downloaded data to server file
    let mut new_server = fs::File::create(server_path)?;
    new_server.write_all(&downloaded_file)?;

    // making the BeamMP-Server file executable on Linux
    if cfg!(target_os = "linux") {
        Command::new("chmod")
            .args(["777", "BeamMP-Server-linux"])
            .spawn()?;
    }

    Ok(())
}