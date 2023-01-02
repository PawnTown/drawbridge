#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use tonic::{IntoStreamingRequest, IntoRequest};
use cecpub::pub_client::PubClient;
use cecpub::{ StreamEngineReply, StreamEngineInitRequest, StreamEngineRequest, stream_engine_request };

pub mod cecpub {
    tonic::include_proto!("cecpub");
}

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
        "ptcec" => start_ptcec_driver(args),
        "ssh" => start_ssh_driver(args),
        _ => {
            println!("Invalid driver name. Please use ptcec or ssh.");
            std::process::exit(1);
        }
    };
}

// Starts the ptcec bridge driver
#[tokio::main]
async fn start_ptcec_driver(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let client = PubClient::connect(args[2].as_ref()).await?;

    let mut initRequest = StreamEngineRequest {
        data: Some(stream_engine_request::Data::Init(StreamEngineInitRequest{
            engine: args[3],
            mode: args[4],
            session_id: genSessionId(),
        })),
    };

    let mut response_stream = client
        .stream_engine(initRequest)
        .into_streaming_request()
        .await?
        .into_inner();

    /*while let Some(response) = response_stream.next().await {
        println!("RESPONSE={:?}", response);
    }*/

    return Ok(());
}

// Starts the ssh bridge driver
#[tokio::main]
async fn start_ssh_driver(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn genSessionId() -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();
    format!("drawbridge_{}", in_ms)
}