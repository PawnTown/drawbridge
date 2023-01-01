#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::env;

use pub_api::{ StreamEngine };
mod pub_api;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        start_tauri();
        return
    }

    start_bridge(args);
}

// Starts the UI
fn start_tauri() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Starts the bridge cli
async fn start_bridge(args: Vec<String>) {
    let _ = match args[1].as_ref() {
        "ptcec" => start_ptcec_driver(args).await,
        "ssh" => start_ssh_driver(args).await,
        _ => {
            println!("Invalid driver name. Please use ptcec or ssh.");
            std::process::exit(1);
        }
    };
}

// Starts the ptcec bridge driver
async fn start_ptcec_driver(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static(args[2].as_ref())
        .connect()
        .await?;
    //loop {
        let request = tonic::Request::new(pub_api::Empty {});
        let response = client.get_stream(request).await?;
        let stream = response.into_inner();
        println!("STREAM: {:?}", stream);
    //}
    Ok(())
}

// Starts the ssh bridge driver
async fn start_ssh_driver(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}