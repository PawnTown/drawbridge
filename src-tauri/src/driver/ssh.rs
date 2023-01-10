use std::net::SocketAddr;
use std::sync::{Arc};
use std::time::Duration;
use std::str::{FromStr};

use anyhow::{Result};
use async_trait::async_trait;
use russh::*;
use russh_keys::*;
use tokio::io::{BufReader, stdin, AsyncBufReadExt, AsyncWriteExt, AsyncReadExt};

use super::Driver;

pub struct SshDriver {}

#[async_trait]
impl Driver for SshDriver {
    async fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() != 6 {
            println!("Invalid number of arguments. Please use: drawbridge ssh <url> <user> <private-key-path> <run-command>");
            std::process::exit(1);
        }

        match start_ssh_driver(args[2].clone(), args[3].clone(), args[4].clone(), args[5].clone()).await {
            Err(e) => {
                println!("Something failed: {}", e);
                std::process::exit(1);
            },
            _ => Ok(()),
        }
    }
}

// Starts the ssh bridge driver
async fn start_ssh_driver(host: String, user: String, private_key_path: String, run_command: String) -> Result<(), anyhow::Error> {
    
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
    channel.exec(true, run_command).await.unwrap();
    let mut stream = channel.into_stream();

    // Start async stuff
    let stdin = stdin();
    let mut reader = BufReader::new(stdin);

    let mut line_in= String::new();

    match tokio::spawn(async move {
        loop {
            println!("loop");
            let mut line_out = vec![];
            tokio::select! {
                res = stream.read(&mut line_out) => {
                    match res {
                        Err(e) => {
                            println!("Error reading from stream: {}", e);
                            break;
                        },
                        _ => {
                            let line_out_str = String::from_utf8(line_out).unwrap();
                            println!("{}", line_out_str);
                        }
                    }
                },
                res = reader.read_line(&mut line_in) => {
                    match res {
                        Err(e) => {
                            println!("Error reading from stream: {}", e);
                            break;
                        },
                        _ => {
                            stream.write(&line_in.as_bytes()).await.unwrap();
                            stream.flush().await.unwrap();
                        },
                    }
                }
            }
        }
        anyhow::Ok::<()>(())
    }).await? {
        _ => {},
    }

   return Ok(());
}

struct Client {}

#[async_trait]
impl client::Handler for Client {
    type Error = anyhow::Error;
 
    async fn check_server_key(
        self,
        _server_public_key: &key::PublicKey,
    ) -> Result<(Self, bool), Self::Error> {
        Ok((self, true))
    }
 }
