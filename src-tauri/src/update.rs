use std::process::Command;
use std::io::Write;
use std::io;
use std::fs;

use reqwest;
use serde_json;

fn get_current_server_version() -> io::Result<String> {
    // executing command to get server version
    let version_command_result = if cfg!(target_os = "windows") {
        Command::new(r".\BeamMP-Server.exe")
            .arg("--version")
            .output()
    } else {
        Command::new("./BeamMP-Server-linux")
            .arg("--version")
            .output()
    };

    // checking for errors with version command execution
    let version_command_output = match version_command_result {
        Ok(output) => output.stdout,
        Err(e) => {
            match e.kind() {
                // if the error is "NotFound", we return the lowest possible version so that the server can be downloaded
                io::ErrorKind::NotFound => "0.0.0".as_bytes().to_vec(),
                _ => return Err(e)
            }
        }
    };

    let raw_string_version = String::from_utf8(version_command_output).expect("server version should be string");
    // string left after replacement is: x.x.x
    let string_version = raw_string_version.replace("BeamMP-Server v", "");

    Ok(string_version)
}

fn get_latest_server_version() -> Result<String, Box<dyn std::error::Error>> {
    // creating http client
    let client = reqwest::blocking::Client::builder()
        .user_agent("BeamMP-Server-Manager")
        .build()?;
    // getting latest server release data
    let response = client.get("https://api.github.com/repos/BeamMP/BeamMP-Server/releases/latest")
        .send()?
        .text()?;
    // converting response data to json
    let json_response: serde_json::Value = serde_json::from_str(&response)?;

    let raw_name = json_response["name"].to_string();
    // stripping "v" off of the front of the name (name looks like: vx.x.x)
    let stripped_name = raw_name[1..].to_string();

    Ok(stripped_name)
}

fn get_numbers_from_version(version: String) -> Vec<u16> {
    // versions look like: x.x.x
    let mut numbers: Vec<u16> = Vec::new();
    let mut current_number = String::new();
    for chr in version.chars() {
        // numbers in the version string are separated by '.'
        // so if the character is not '.', it has to be a number
        if chr != '.' {
            current_number.push(chr);
        } else {
            // if the character is '.', push the current number to the vector
            // and reset the current number
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

    // checking if any number in the latest version is greater than the current version
    for (i, num) in local_numbers.iter().enumerate() {
        if latest_numbers[i] > *num {
            needs_update = true;
            break;
        }
    }

    needs_update
}

fn download_latest_server() -> Result<(), Box<dyn std::error::Error>> {
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

pub fn auto_update_server() {
    let current_version = match get_current_server_version() {
        Ok(version) => version,
        Err(e) => {eprintln!("Error: {:?}, {}", e.kind(), e.to_string()); return;}
    };
    let latest_version = match get_latest_server_version() {
        Ok(version) => version,
        Err(e) => {eprintln!("Error: {:?}", e.to_string()); return;}
    };
    if needs_update(current_version, latest_version) {
        match download_latest_server() {
            Ok(_) => return,
            Err(e) => {eprintln!("Error: {:?}", e.to_string()); return;}
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_numbers_from_version() {
        assert_eq!(get_numbers_from_version(String::from("0.0.0")), vec![0u16; 3]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_get_numbers_from_version() {
        get_numbers_from_version(String::from("x.0.0"));
    }

    #[test]
    fn test_needs_update() {
        assert!(needs_update(String::from("0.0.0"), String::from("1.1.1")));
        assert_eq!(needs_update(String::from("0.0.0"), String::from("0.0.0")), false);
    }
}