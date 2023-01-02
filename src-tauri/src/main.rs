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
use std::str;
use tokio_stream::StreamExt;
use tokio::sync::mpsc;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use cecpub::pub_client::PubClient;
use cecpub::{ StreamEngineInitRequest, StreamEngineRequest, stream_engine_request };
use tokio_stream::wrappers::ReceiverStream;
use tonic::metadata::AsciiMetadataValue;
use tonic::transport::Channel;
use tonic::Request;
pub mod cecpub {
    tonic::include_proto!("cecpub");
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Starts the bridge cli
#[tokio::main]
async fn start_bridge(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    if args[1] == "ptcec" {
        if args.len() != 6 {
            println!("Invalid number of arguments. Please use: drawbridge ptcec <url> <engine> <mode> <token>");
            std::process::exit(1);
        }
        return start_ptcec_driver(args[2].clone(), args[3].clone(), args[4].clone(), args[5].clone(), gen_session_id()).await;
    }

    if args[1] == "ptcec" {
        println!("Driver not implemented yet.");
        std::process::exit(1);
    }

    println!("Invalid driver name. Please use ptcec or ssh.");
    std::process::exit(1);
}

// Starts the ptcec bridge driver
async fn start_ptcec_driver(url: String, engine: String, mode: String, token: String, session_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static(url.as_ref()).connect().await?;
    let mut authHeader = "Basic ".to_owned();
    authHeader.push_str(&token);

    let mut client = PubClient::with_interceptor(channel, |mut req: Request<()>| {
        req.metadata_mut().insert(
            "Authorization",
            AsciiMetadataValue::from_static(authHeader.as_ref()),
        );
        Ok(req)
    });

    let (tx, rx) = mpsc::channel(80);

    // Initialize
    let init_req = StreamEngineRequest {
        data: Some(stream_engine_request::Data::Init(StreamEngineInitRequest{
            engine: engine,
            mode: mode,
            session_id: session_id,
        })),
    };
    tx.send(init_req).await.unwrap();

    tokio::spawn(async move {
        let stdin = stdin();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();
        loop {
            reader.read_line(&mut line).await.unwrap();
            tx.send(StreamEngineRequest {
                data: Some(stream_engine_request::Data::Stdin(line.as_bytes().to_vec())),
            }).await.unwrap();
            line.clear();
        }
    });

    let in_stream = ReceiverStream::new(rx);
    let mut stream = client
        .stream_engine(in_stream)
        .await
        .unwrap()
        .into_inner();


    while let Some(item) = stream.next().await {
        match str::from_utf8(item.unwrap().stdout.as_ref()) {
            Ok(v) => println!("\treceived: {}", v),
            Err(_) => {/* ignored */},
        };
    }

    return Ok(());
}

fn gen_session_id() -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();
    format!("drawbridge_{}", in_ms)
}