use std::process::Command;
use std::io::{self, Write};
use std::fs;

use crate::settings::manager_settings;

use reqwest;
use serde_json;
use regex::Regex;

fn extract_version_from_string(input: String) -> Option<String> {
    let re = Regex::new(r"\d+.\d+.\d+").unwrap();
    let re_output = re.captures(&input)?;
    return match re_output.get(0) {
        Some(matched) => {
            let version = String::from(matched.as_str());
            Some(version)
        }
        None => None,
    };
}

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
            return match e.kind() {
                // if the error is "NotFound", we return the lowest possible version so that the server can be downloaded
                io::ErrorKind::NotFound => Ok(String::from("0.0.0")),
                _ => Err(e),
            }
        }
    };

    let raw_string_version = String::from_utf8(version_command_output).expect("server version should be string");
    let string_version = extract_version_from_string(raw_string_version).unwrap();

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
    // removing any possible extra characters in name string
    let processed_name = extract_version_from_string(raw_name).unwrap();

    Ok(processed_name)
}

fn get_numbers_from_version(version: String) -> Vec<u16> {
    // versions look like: x.x.x
    let mut numbers: Vec<u16> = Vec::new();
    let mut current_number = String::new();
    let mut version_chars = version.chars().peekable();
    while let Some(chr) = version_chars.next() {
        // numbers in the version string are separated by '.'
        // so if the character is not '.', it has to be a number
        if chr != '.' {
            current_number.push(chr);
            // peeking at the next value to see if the current character is the last in the string
            // if it is, we push the current number and break the loop
            if version_chars.peek().is_none() {
                numbers.push(current_number.parse::<u16>().unwrap());
                current_number.clear();
                break;
            }
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
        .user_agent("BeamMP-Server-Manager") // user agent is required for github api
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
    let is_linux = !cfg!(target_os = "windows");
    let mut distro_info = String::new();
    if is_linux {
        distro_info = String::from_utf8(Command::new("cat").args(["/etc/issue"]).output().expect("").stdout).unwrap();
    }
    // looping through each asset until the correct one for the users OS is found
    for asset in release_assets.as_array().unwrap() {
        if !is_linux {
            if asset["name"].to_string().contains("BeamMP-Server.exe") {
                download_link = asset["browser_download_url"].as_str().unwrap().to_string();
            }
        } else {
            if distro_info.contains("Ubuntu") {
                if asset["name"].to_string().contains("ubuntu") && asset["name"].to_string().contains("x86_64") {
                    download_link = asset["browser_download_url"].as_str().unwrap().to_string();
                }
            } else if distro_info.contains("Debian") {
                if asset["name"].to_string().contains("debian") && asset["name"].to_string().contains("x86_64") {
                    download_link = asset["browser_download_url"].as_str().unwrap().to_string();
                }
            } else {
                panic!("Unsupported OS");
            }
        }
    }

    // downloading file from previously found link
    let downloaded_file = client.get(download_link)
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
    if manager_settings::_read_manager_settings().unwrap().auto_update {
        let current_version = match get_current_server_version() {
            Ok(version) => version,
            Err(e) => {eprintln!("Error: {:?}, {}", e.kind(), e.to_string()); return;}
        };
        let latest_version = match get_latest_server_version() {
            Ok(version) => version,
            Err(e) => {eprintln!("Error: {:?}", e.to_string()); return;}
        };
        if needs_update(current_version.clone(), latest_version.clone()) {
            match download_latest_server() {
                Ok(_) => {println!("Server Updated: {} -> {}", current_version, latest_version); return;},
                Err(e) => {eprintln!("Error: {:?}", e.to_string()); return;}
            };
        }
    } else {
        return
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

    #[test]
    fn test_get_current_version() {
        assert_eq!(get_current_server_version().unwrap(), String::from("3.2.1"))
    }

    #[test]
    fn test_get_latest_version() {
        assert_eq!(get_latest_server_version().unwrap(), String::from("3.2.1"))
    }

    #[test]
    fn test_needs_update_real() {
        assert_eq!(needs_update(get_current_server_version().unwrap(), get_latest_server_version().unwrap()), false)
    }

    #[test]
    fn test_auto_update() {
        auto_update_server();
    }

    #[test]
    fn test_extract_version_from_string() {
        assert_eq!(extract_version_from_string(String::from("\u{1b}[2K\u{1b}[0GBeamMP-Server v3.1.1\r\n\u{1b}[1G\u{1b}[2K\u{1b}[0G\u{1b}[1G")), Some(String::from("3.2.1")))
    }
}