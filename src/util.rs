use std::net::{TcpStream, Shutdown};
use std::io::{self, Write};
use std::{fs};


pub fn folder_exists(folder_path: &str) -> bool {
    fs::metadata(folder_path).is_ok()
}

pub fn create_folder(folder_path: &str) {
    if !folder_exists(&folder_path) {
        if let Err(err) = fs::create_dir(&folder_path) {
            eprintln!("Failed to create the folder: {}", err);
        } else {
            println!("Folder created successfully: {}", folder_path);
        }
    } else {
        println!("Folder already exists: {}", folder_path);
    }
}

pub fn get_user_folder(user_name: &str) -> String {
    let base_path = get_cli_base_folder();
    let folder_path = format!("{}/{}", base_path, user_name);
    return folder_path;
}

pub fn get_home_dir() -> String {
    let home_dir = std::env::var("HOME").expect("Failed to get the home directory");
    home_dir
}

pub fn get_cli_base_folder() -> String {
    let home_dir = get_home_dir();
    let folder_path = format!("{}/.palworld_oneshot", home_dir);
    return folder_path;
}

pub fn is_port_in_use(port: u16) -> bool {
    match TcpStream::connect(("localhost", port)) {
        Ok(_) => {
            // The port is in use
            true
        }
        Err(_) => {
            // The port is not in use
            false
        }
    }
}

pub fn get_random_safe_port() -> u16 {
    let min_port = 3000; // avoid system ports
    let max_port = 9999; // use only those ports with 4 digits

    loop {
        let port = min_port + (rand::random::<u16>() % (max_port - min_port + 1));

        if !is_port_in_use(port) {
            return port;
        }
    }
}