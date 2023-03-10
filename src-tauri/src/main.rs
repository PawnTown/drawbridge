#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(target_family = "windows")]
mod win_bat;

#[cfg(target_family = "windows")]
mod win_exe;

mod driver;
mod storage;
mod settings;
mod logger;
mod middleware;
use std::{env, time::SystemTime};

#[tauri::command]
fn load_data(key: String) -> String {
    match storage::load(key) {
        Ok(json) => return json.to_string(),
        Err(_) => return "null".to_string(),
    }
}

#[tauri::command]
fn save_data(key: String, val: String) -> bool {
    match storage::save(key, val) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

#[tauri::command]
fn get_os() -> String {
    if cfg!(windows) {
       return "win".to_string()
    } else if cfg!(unix) {
        return "unix".to_string()
    }
    return "unix".to_string();
}

#[tauri::command]
fn create_shortcut(output: String, id: String) -> bool {
    return os_create_shortcut(output, id);
}

#[cfg(target_family = "unix")]
fn os_create_shortcut(output: String, id: String) -> bool {
    use fs::File;
    use std::fs;
    use std::io::Write;
    use std::os::unix::prelude::PermissionsExt;

    let exec_path;
    match env::current_exe() {
        Ok(exe_path) => exec_path = exe_path.display().to_string(),
        Err(_) =>  return false,
    };

    let file = File::create(output.clone());

    if let Err(_err) = file.unwrap().write_all(format!("#!/bin/sh\n\"{}\" connect {}", exec_path, id).as_bytes()) {
        return false;
    }

    fs::set_permissions(output, fs::Permissions::from_mode(500)).unwrap();
    return true;
}

#[cfg(target_family = "windows")]
fn os_create_shortcut(output: String, id: String) -> bool {
    let exec_path;
    match env::current_exe() {
        Ok(exe_path) => exec_path = exe_path.display().to_string(),
        Err(_) =>  return false,
    };

    if output.ends_with(".exe") {
        win_exe::compile_cs_file(exec_path, id, output).unwrap();
    } else {
        win_bat::create_bat_shortcut(exec_path, id, output).unwrap();
    }
    
    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        start_tauri();
        return
    }

    match start_bridge(args) {
        Ok(_) => {/* ignored */},
        Err(_) => {/* ignored */},
    };
}

// Starts the UI
fn start_tauri() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_shortcut, get_os, save_data, load_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_logger() -> Option<logger::Logger> {
    use std::path::Path;
    use rand::distributions::{Alphanumeric, DistString};

    let logs_enabled = settings::get_setting_bool("enableLogs".to_string());
    if logs_enabled.is_none() || logs_enabled == Some(false) {
        return None;
    }

    match settings::get_setting_str("logFile".to_string()) {
        Some(mut path) => {
            if path.eq("") {
                return None;
            }

            let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
            path = path.replace("{rnd}", &Alphanumeric.sample_string(&mut rand::thread_rng(), 6));
            path = path.replace("{ts}", &format!("{}", ts.as_nanos()));

            let path = Path::new(&path);
            match logger::Logger::new(path.to_path_buf()) {
                Ok(logger) => Some(logger),
                Err(_) => None
            }
        },
        None => {
            return None;
        }
    }
}

// Starts the bridge cli
#[tokio::main]
async fn start_bridge(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let driver = driver::get_driver(args[1].clone());
    let logger: Option<logger::Logger> = get_logger();
    let middleware = middleware::Middleware::new("".into());

    if driver.is_some() {
        // Todo: add logger initialization
        return driver.unwrap().run(args, logger, middleware).await;
    }

    println!("Invalid driver name. Please use ptcec or ssh.");
    std::process::exit(1);
}
 