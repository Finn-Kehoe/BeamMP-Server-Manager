use std::process::{Command, Child};
use std::io::ErrorKind;
use std::sync::Mutex;

use crate::util::error;
use crate::settings::manager_settings;

use regex::Regex;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(serde::Serialize)]
pub enum ServerStatus {
    Stopped,
    Starting,
    Running,
}

pub struct Server {
    pub server: Mutex<Option<Child>>,
    pub startup_is_finished: Mutex<bool>,
}

impl Server {
    pub fn start() -> Self {
        let server = _start_server();
        Self {server: Mutex::new(server), startup_is_finished: Mutex::new(false)}
    }
}

#[cfg(target_os = "windows")]
fn _start_server() -> Option<Child> {
    if manager_settings::_read_manager_settings().unwrap().show_server_terminal == true {
        return match Command::new(r".\BeamMP-Server.exe").spawn() {
            Ok(ch) => Some(ch),
            Err(_) => None,
        }
    } else {
        return match Command::new(r".\BeamMP-Server.exe")
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .spawn() {
                Ok(ch) => Some(ch),
                Err(_) => None,
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn _start_server() -> Option<Child> {
    return match Command::new("./BeamMP-Server-linux").spawn() {
        Ok(ch) => Some(ch),
        Err(_) => None,
    }
}

#[tauri::command]
pub fn start_server(server: tauri::State<Server>) -> Result<(), error::Error> {
    *server.server.lock().unwrap() = _start_server();
    *server.startup_is_finished.lock().unwrap() = false;

    Ok(())
}

#[tauri::command]
pub fn close_server(server: tauri::State<Server>) -> Result<(), error::Error> {
    return match server.server.lock().unwrap().as_mut() {
        Some(svr) => {
            match svr.kill() {
                Ok(_) => Ok(()),
                Err(e) => {
                    match e.kind() {
                        ErrorKind::InvalidInput => Ok(()),
                        _ => Err(error::Error::from(e))
                    }
                }
            }
        },
        None => Err(error::Error::from(std::io::Error::new(ErrorKind::Other, "server not open"))),
    };
}

#[tauri::command]
pub fn restart_server(server: tauri::State<Server>) -> Result<(), error::Error> {
    close_server(server.clone())?;

    *server.server.lock().unwrap() = _start_server();
    *server.startup_is_finished.lock().unwrap() = false;

    Ok(())
}

#[tauri::command]
pub fn check_server_status(server: tauri::State<Server>) -> Result<ServerStatus, error::Error> {
    // attempt to collect server's process exit status
    match server.server.lock().unwrap().as_mut() {
        Some(svr) => {
            match svr.try_wait() {
                // if exit status is able to be collected then server has stopped
                Ok(Some(_)) => return Ok(ServerStatus::Stopped),
                // if it isn't than the server is running to some degree
                Ok(None) => {},
                Err(e) => {
                    eprintln!("Error checking server status: {e}");
                    return Ok(ServerStatus::Stopped);
                }
            }
        },
        None => return Ok(ServerStatus::Stopped),
    }

    if *server.startup_is_finished.lock().unwrap() {
        return Ok(ServerStatus::Running);
    } else {
        // looking for "[INFO] ALL SYSTEMS STARTED SUCCESSFULLY, EVERYTHING IS OKAY" in server log file
        let mut log_file_path = std::env::current_dir()?;
        log_file_path.push("Server.log");
    
        if log_file_path.is_file() {
            let log_contents_string: String = std::fs::read_to_string(&log_file_path)?;

            let re = Regex::new(r"\[INFO\] ALL SYSTEMS STARTED SUCCESSFULLY, EVERYTHING IS OKAY").unwrap();
            if re.is_match(&log_contents_string) {
                *server.startup_is_finished.lock().unwrap() = true;
                return Ok(ServerStatus::Running);
            } else {
                return Ok(ServerStatus::Starting);
            }
        } else {
            return Ok(ServerStatus::Stopped);
        }
    }
}