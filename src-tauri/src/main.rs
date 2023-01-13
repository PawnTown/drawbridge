#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(target_family = "windows")]
mod win_cmd;

mod driver;
mod storage;
use std::env;

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

    if let Err(_err) = file.unwrap().write_all(format!("#!/bin/sh\n{} connect {}", exec_path, id).as_bytes()) {
        return false;
    }

    fs::set_permissions(output, fs::Permissions::from_mode(500)).unwrap();
    return true;
}

#[cfg(target_family = "windows")]
fn os_create_shortcut(output: String, id: String) -> bool {
    use directories::ProjectDirs;
    use fs::File;
    use std::fs;
    use std::io::Write;

    let exec_path;
    match env::current_exe() {
        Ok(exe_path) => exec_path = exe_path.display().to_string(),
        Err(_) =>  return false,
    };

    let mut bat_file;
    if let Some(proj_dirs) = ProjectDirs::from("im", "khad",  "drawbridge") {
        bat_file = proj_dirs.data_local_dir().to_path_buf();
        bat_file.push(format!("engine_{}.bat", id));

        let file = File::create(bat_file.clone());
        if let Err(_err) = file.unwrap().write_all(format!("{} connect {}", exec_path, id).as_bytes()) {
            return false;
        }
    } else {
        return false;
    }

    win_cmd::run_temp_script(
        format!(
            "
Set oWS = WScript.CreateObject(\"WScript.Shell\")
Set oLink = oWS.CreateShortcut(\"{}\")
oLink.TargetPath = \"{}\"
oLink.Arguments = \"connect {}\"
oLink.IconLocation = \"{}, 0\"
oLink.Save()
        ", output, bat_file.to_string_lossy(), id, exec_path),
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
        .invoke_handler(tauri::generate_handler![create_shortcut, get_os, save_data, load_data])
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
 