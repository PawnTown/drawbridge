use std::io::Write;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::str::{self, FromStr};

use anyhow::Result;
use async_trait::async_trait;
use russh::*;
use russh::client::Msg;
use russh_keys::*;
use tokio::io::{BufReader, stdin, AsyncBufReadExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use super::Driver;

pub struct SshDriver {}

#[async_trait]
impl Driver for SshDriver {
    async fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() != 6 {
            println!("Invalid number of arguments. Please use: drawbridge ssh <url> <user> <private-key-path> <run-command>");
            std::process::exit(1);
        }

        return start_ssh_driver(args[2].clone(), args[3].clone(), args[4].clone(), args[5].clone()).await;
    }
}

// Starts the ssh bridge driver
async fn start_ssh_driver(host: String, user: String, private_key_path: String, run_command: String) -> Result<(), Box<dyn std::error::Error>> {
    
    // Open SSH Session
    let key_pair = load_secret_key(private_key_path, None)?;
    let config = client::Config {
        connection_timeout: Some(Duration::from_secs(5)),
        ..<_>::default()
    };
    let config = Arc::new(config);
    let sh = Client {};
    let mut session = client::connect(config, SocketAddr::from_str(&host).unwrap(), sh).await?;
    let _auth_res = session
        .authenticate_publickey(user, Arc::new(key_pair))
        .await?;
    
    // Create new channel
    let mut channel = session.channel_open_session().await.unwrap();

    // Remote stdin
    tokio::spawn(async move {
        println!("Waiting for data");
        while let Some(msg) = channel.wait().await {
            match msg {
                russh::ChannelMsg::Data { ref data } => {
                    match str::from_utf8(data) {
                        Ok(v) => println!("{}", v),
                        Err(_) => {/* ignored */},
                    };
                }
                _ => {}
            }
        }
        println!("Exited reading");
    });

    Ok(())

    /*channel.exec(false, &run_command).await.unwrap();

    let stdin = stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();
    loop {
        println!("> ");
        reader.read_line(&mut line).await.unwrap();
        channel.exec(false, &line).await.unwrap();
        line.clear();
    }*/
}

struct Client {}

#[async_trait]
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
    fn check_server_key(self, _server_public_key: &key::PublicKey) -> Self::FutureBool {
        self.finished_bool(true)
    }
    /*fn channel_open_confirmation(self, channel: ChannelId, max_packet_size: u32, window_size: u32, session: client::Session) -> Self::FutureUnit {
        self.finished(session)
    }
    fn data(self, channel: ChannelId, data: &[u8], session: client::Session) -> Self::FutureUnit {
        self.stdout.send(data.to_vec());
        self.finished(session)
    }*/
 }
