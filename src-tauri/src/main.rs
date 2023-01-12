#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod driver;
mod storage;
mod win_cmd;
use std::fs::File;
use std::{fs, env};
use std::io::Write;
use std::os::unix::prelude::PermissionsExt;

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
fn get_shortcut_extension() -> String {
    if cfg!(windows) {
       return "ink".to_string()
    } else if cfg!(unix) {
        return "".to_string()
    }
    return "".to_string();
}

#[tauri::command]
fn create_shortcut(output: String, id: String) -> bool {
    if cfg!(windows) {
       return create_win_shortcut(output, id);
    } else if cfg!(unix) {
        return create_unix_script(output, id);
    }
    return false;
}

fn create_unix_script(output: String, id: String) -> bool {
    let exec_path;
    match env::current_exe() {
        Ok(exe_path) => exec_path = exe_path.display().to_string(),
        Err(_) =>  return false,
    };

    let file = File::create(output.clone());

    if let Err(_err) = file.unwrap().write_all(format!("#!/bin/sh\n{} connect {}", exec_path, id).as_bytes()) {
        return false;
    }

    fs::set_permissions(output, fs::Permissions::from_mode(500)).unwrap();
    return true;
}

fn create_win_shortcut(output: String, id: String) -> bool {
    let exec_path;
    match env::current_exe() {
        Ok(exe_path) => exec_path = exe_path.display().to_string(),
        Err(_) =>  return false,
    };

    let file = File::create(output.clone());

    if let Err(_err) = file.unwrap().write_all(format!("#!/bin/sh\n{} connect {}", exec_path, id).as_bytes()) {
        return false;
    }

    win_cmd::run_temp_script(
        format!(
            "

            Set oWS = WScript.CreateObject(\"WScript.Shell\")
sLinkFile = \"{}\"
Set oLink = oWS.CreateShortcut(sLinkFile)
    oLink.TargetPath = \"{}\"
    oLink.Arguments = \"connect {}\"
oLink.Save
        ",output, exec_path, id),
        "mk_shortcut.vbs",
    ).unwrap();
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
        .invoke_handler(tauri::generate_handler![create_shortcut, get_shortcut_extension, save_data, load_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Starts the bridge cli
#[tokio::main]
async fn start_bridge(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let driver = driver::get_driver(args[1].clone());

    if driver.is_some() {
        return driver.unwrap().run(args).await;
    }

    println!("Invalid driver name. Please use ptcec or ssh.");
    std::process::exit(1);
}
 