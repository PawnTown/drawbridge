use async_trait::async_trait;
use tonic::transport::Channel;
use tonic::{Request, metadata::AsciiMetadataValue};
use proto::pub_client::PubClient;
use proto::{ StreamEngineInitRequest, StreamEngineRequest, stream_engine_request };
use std::sync::{Arc};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio_stream::StreamExt;
use std::str::{self};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::Mutex;

pub mod proto {
    tonic::include_proto!("proto");
}

use crate::logger;
use crate::middleware::Middleware;

use super::Driver;

pub struct PtcecDriver {}

#[async_trait]
impl Driver for PtcecDriver {
    async fn run(&self, args: Vec<String>, logger: Option<logger::Logger>, middleware: Middleware) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() != 6 {
            println!("Invalid number of arguments. Please use: drawbridge ptcec <url> <engine> <mode> <token>");
            std::process::exit(1);
        }
        return ptcec_run(args[2].clone(), args[3].clone(), args[4].clone(), args[5].clone(), gen_session_id(), logger, middleware).await;
    }
}

async fn ptcec_run(url: String, engine: String, mode: String, token: String, session_id: String, logger: Option<logger::Logger>, middleware: Middleware) -> Result<(), Box<dyn std::error::Error>> {
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

    // Logger
    let logger_o_ptr = Arc::new(Mutex::new(logger));
    let logger_ptr_a = Arc::clone(&logger_o_ptr);

    // Initialize
    let mut guard = logger_ptr_a.lock().await; {
        if guard.is_some() {
            if guard.as_mut().unwrap().debug_info(&format!("Initialize PTCEC with Engine={} Mode={} SessionId={}", &engine, &mode, &session_id).to_string()).is_err() {/* ignored */}
        }
        drop(guard);
    }
    let init_req = StreamEngineRequest {
        data: Some(stream_engine_request::Data::Init(StreamEngineInitRequest{
            engine: engine,
            mode: mode,
            session_id: session_id,
        })),
    };
    tx.send(init_req).await.unwrap();

    let middleware_ptr = Arc::new(middleware);
    

    let middleware_ptr_a = Arc::clone(&middleware_ptr);
    let logger_ptr_b = Arc::clone(&logger_o_ptr);
    tokio::spawn(async move {
        let stdin = stdin();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();
        loop {
            match reader.read_line(&mut line).await {
                Err(e) => {
                    let mut guard = logger_ptr_b.lock().await; {
                        if guard.is_some() {
                            if guard.as_mut().unwrap().debug_error(&format!("Failed to read stdin: {}", e)).is_err() {/* ignored */}
                        }
                        drop(guard);
                    };
                    break;
                },
                _ => (),
            }

            line = line.replace("\n", "");
            let oline = line.clone();

            match middleware_ptr_a.handle_out(oline.clone()) {
                Ok(new_line) => {
                    line = new_line;
                },
                Err(e) => {
                    let mut guard = logger_ptr_a.lock().await; {
                        if guard.is_some() {
                            if guard.as_mut().unwrap().debug_error(&format!("Failed to apply outgoing middleware: {}", e)).is_err() {/* ignored */}
                        }
                        drop(guard);
                    };
                    continue;
                },
            };

            if oline != "" && line == "" {
                continue;
            }

            line.push_str("\n");

            let mut guard = logger_ptr_a.lock().await; {
                if guard.is_some() {
                    let mut logline = line.clone();
                    logline.pop();
                    if guard.as_mut().unwrap().debug_outgoing(&logline).is_err() {/* ignored */}
                }
                drop(guard);
            }
            
            match tx.send(StreamEngineRequest {
                data: Some(stream_engine_request::Data::Stdin(line.as_bytes().to_vec())),
            }).await {
                Err(e) => {
                    let mut guard = logger_ptr_b.lock().await; {
                        if guard.is_some() {
                            if guard.as_mut().unwrap().debug_error(&format!("Failed to send to remote: {}", e)).is_err() {/* ignored */}
                        }
                        drop(guard);
                    };
                    break;
                },
                _ => (),
            }
            line.clear();
        }
    });

    let in_stream = ReceiverStream::new(rx);
    let mut stream = client
        .stream_engine(in_stream)
        .await
        .unwrap()
        .into_inner();
    
    let middleware_ptr_b = Arc::clone(&middleware_ptr);
    let logger_ptr_c = Arc::clone(&logger_o_ptr);
    while let Some(item) = stream.next().await {
        match str::from_utf8(item.unwrap().stdout.as_ref()) {
            Ok(oline) => {
                let mut line = oline.clone().to_string();
                match middleware_ptr_b.handle_in(line) {
                    Ok(new_line) => {
                        line = new_line;
                    },
                    Err(e) => {
                        let mut guard = logger_ptr_c.lock().await; {
                            if guard.is_some() {
                                if guard.as_mut().unwrap().debug_error(&format!("Failed to apply incomming middleware: {}", e)).is_err() {/* ignored */}
                            }
                            drop(guard);
                        };
                        continue;
                    },
                };

                if oline != "" && line == "" {
                    continue;
                }

                let mut guard = logger_ptr_c.lock().await;
                if guard.is_some() {
                    if guard.as_mut().unwrap().debug_incomming(&line.clone()).is_err() {/* ignored */}
                }
                drop(guard);
                println!("{}", line)
            },
            Err(e) => {
                let mut guard = logger_ptr_c.lock().await; {
                    if guard.is_some() {
                        if guard.as_mut().unwrap().debug_error(&format!("Failed to parse stdout: {}", e)).is_err() {/* ignored */}
                    }
                    drop(guard);
                };
                break;
            },
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