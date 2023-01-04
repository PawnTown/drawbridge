#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::File;
use std::fs;
use std::net::SocketAddr;
use std::str::FromStr;
use russh::{client, ChannelId};
use russh_keys::key;
use anyhow::Error;

#[tauri::command]
fn create_ptcec_unix_script(output: String, url: String, engine: String, mode: String, token: String) -> bool {
    let exec_path;
    match env::current_exe() {
        Ok(exe_path) => exec_path = exe_path.display().to_string(),
        Err(_) =>  return false,
    };

    let file = File::create(output.clone());

    if let Err(_err) = file.unwrap().write_all(format!("#!/bin/sh\n{} ptcec {} {} {} {}", exec_path, url, engine, mode, token).as_bytes()) {
        return false;
    }

    fs::set_permissions(output, fs::Permissions::from_mode(500)).unwrap();
    return true;
}

#[tauri::command]
fn create_ssh_unix_script(output: String, url: String, run_command: String) -> bool {
    let exec_path;
    match env::current_exe() {
        Ok(exe_path) => exec_path = exe_path.display().to_string(),
        Err(_) =>  return false,
    };

    let file = File::create(output.clone());

    if let Err(_err) = file.unwrap().write_all(format!("#!/bin/sh\n{} ssh {} \"{}\"", exec_path, url, run_command).as_bytes()) {
        return false;
    }

    fs::set_permissions(output, fs::Permissions::from_mode(500)).unwrap();
    return true;
}



use std::io::Write;
use std::os::unix::prelude::PermissionsExt;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use std::str::{self};
use tauri_plugin_store::PluginBuilder;
use tokio_stream::StreamExt;
use tokio::sync::mpsc;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use proto::pub_client::PubClient;
use proto::{ StreamEngineInitRequest, StreamEngineRequest, stream_engine_request };
use tokio_stream::wrappers::ReceiverStream;
use tonic::metadata::AsciiMetadataValue;
use tonic::transport::Channel;
use tonic::Request;
pub mod proto {
    tonic::include_proto!("proto");
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
        .plugin(PluginBuilder::default().build())
        .invoke_handler(tauri::generate_handler![create_ptcec_unix_script, create_ssh_unix_script])
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

    if args[1] == "ssh" {
        println!("Driver not implemented yet.");
        std::process::exit(1);
    }

    println!("Invalid driver name. Please use ptcec or ssh.");
    std::process::exit(1);
}

// Starts the ptcec bridge driver
async fn start_ptcec_driver(url: String, engine: String, mode: String, token: String, session_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_shared(url).unwrap().connect().await?;
    let mut auth_header = "Basic ".to_owned();
    auth_header.push_str(&token);

    let mut client = PubClient::with_interceptor(channel, |mut req: Request<()>| {
        req.metadata_mut().insert(
            "authorization",
            AsciiMetadataValue::try_from(auth_header.clone()).unwrap(),
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
            Ok(v) => println!("{}", v),
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

// Starts the ssh bridge driver
async fn start_ssh_driver(url: String, user: String, runCommand: String) -> Result<(), Box<dyn std::error::Error>> {
    // Open ssh session
    let config = russh::client::Config::default();
    let config = Arc::new(config);
    let sh = Client{};

    let key = russh_keys::key::KeyPair::generate_ed25519().unwrap();
    let mut agent = russh_keys::agent::client::AgentClient::connect_env().await.unwrap();
    agent.add_identity(&key, &[]).await.unwrap();
    let mut session = russh::client::connect(config, SocketAddr::from_str(&url).unwrap(), sh).await.unwrap();
    if session.authenticate_future(std::env::var(user).unwrap(), key.clone_public_key().unwrap(), agent).await.1.unwrap() {
        let mut channel = session.channel_open_session().await.unwrap();
        
        let stdin = stdin();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();
        loop {
            reader.read_line(&mut line).await.unwrap();
            channel.data(line.as_bytes()).await.unwrap();
            line.clear();
        }
    }

    return Ok(());
}

struct Client {
}

impl client::Handler for Client {
    type Error = anyhow::Error;
    type FutureUnit = futures::future::Ready<Result<(Self, client::Session), anyhow::Error>>;
    type FutureBool = futures::future::Ready<Result<(Self, bool), anyhow::Error>>;
 
    fn finished_bool(self, b: bool) -> Self::FutureBool {
        futures::future::ready(Ok((self, b)))
    }
    fn finished(self, session: client::Session) -> Self::FutureUnit {
        futures::future::ready(Ok((self, session)))
    }
    fn check_server_key(self, server_public_key: &key::PublicKey) -> Self::FutureBool {
        println!("check_server_key: {:?}", server_public_key);
        self.finished_bool(true)
    }
    fn channel_open_confirmation(self, channel: ChannelId, max_packet_size: u32, window_size: u32, session: client::Session) -> Self::FutureUnit {
        println!("channel_open_confirmation: {:?}", channel);
        self.finished(session)
    }
    fn data(self, channel: ChannelId, data: &[u8], session: client::Session) -> Self::FutureUnit {
        println!("data on channel {:?}: {:?}", channel, std::str::from_utf8(data));
        self.finished(session)
    }
 }
 