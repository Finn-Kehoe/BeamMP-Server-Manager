use std::process::{Command, Child};
use std::io::ErrorKind;
use std::sync::Mutex;

use crate::util::error;

use regex::Regex;

#[derive(serde::Serialize)]
pub enum ServerStatus {
    Stopped,
    Starting,
    Running,
}

pub struct Server {
    pub server: Mutex<Child>,
    pub startup_is_finished: Mutex<bool>,
}

impl Server {
    pub fn start() -> Self {
        let server = _start_server();
        Self {server: Mutex::new(server), startup_is_finished: Mutex::new(false)}
    }
}

fn _start_server() -> Child {
    return if cfg!(target_os = "windows") {
        Command::new(r".\BeamMP-Server.exe")
            .spawn()
            .expect("")
    } else {
        Command::new("./BeamMP-Server-linux")
            .spawn()
            .expect("")
    };
}

#[tauri::command]
pub fn start_server(server: tauri::State<Server>) -> Result<(), error::Error> {
    *server.server.lock().unwrap() = _start_server();
    *server.startup_is_finished.lock().unwrap() = false;

    Ok(())
}

#[tauri::command]
pub fn close_server(server: tauri::State<Server>) -> Result<(), error::Error> {
    return match server.server.lock().unwrap().kill() {
        Ok(_) => Ok(()),
        Err(e) => {
            match e.kind() {
                ErrorKind::InvalidInput => Ok(()),
                _ => Err(error::Error::from(e))
            }
        }
    }
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
    match server.server.lock().unwrap().try_wait() {
        // if exit status is able to be collected then server has stopped
        Ok(Some(_)) => return Ok(ServerStatus::Stopped),
        // if it isn't than the server is running to some degree
        Ok(None) => {},
        Err(e) => {
            eprintln!("Error checking server status: {e}");
            return Ok(ServerStatus::Stopped);
        }
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