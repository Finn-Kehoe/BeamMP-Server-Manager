use std::process::{Command, Child};
use std::io::{Error, ErrorKind};
use std::sync::Mutex;

use crate::util::error;

pub struct Server {
    pub server: Mutex<Child>
}

impl Server {
    pub fn start() -> Self {
        let server = start_server();
        Self {server: Mutex::new(server)}
    }
}

pub fn start_server() -> Child {
    return if cfg!(target_os = "windows") {
        Command::new(r".\BeamMP-Server.exe")
            .spawn()
            .expect("")
    } else {
        Command::new("./BeamMP-Server")
            .spawn()
            .expect("")
    };
}

#[tauri::command]
pub fn close_server(server: tauri::State<Server>) -> Result<(), error::Error> {
    return match server.server.lock().unwrap().kill() {
        Ok(_) => Ok(()),
        Err(e) => {
            match e.kind() {
                ErrorKind::InvalidInput => Ok(()),
                _ => Err(error::Error::from(Error::new(ErrorKind::Other, "unable to close server")))
            }
        }
    }
}

#[tauri::command]
pub fn restart_server(server: tauri::State<Server>) -> Result<(), error::Error> {
    // TODO: ensure this closes the correct server
    close_server(server.clone())?;

    *server.server.lock().unwrap() = start_server();

    Ok(())
}