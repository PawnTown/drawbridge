
use std::{process::{Stdio}, sync::Arc};
use tokio::{process::Command, io::{BufReader, AsyncBufReadExt, BufWriter, self, AsyncWriteExt}};
use tokio::sync::Mutex;
use async_trait::async_trait;

use crate::logger;
use crate::middleware::Middleware;

use super::Driver;

pub struct SshDriver {}

#[async_trait]
impl Driver for SshDriver {
    async fn run(&self, args: Vec<String>, logger: Option<logger::Logger>, middleware: Middleware) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() != 4 && args.len() != 5 {
            println!("Invalid number of arguments. Please use: drawbridge ssh <url> <run-command> [private-key-file]");
            std::process::exit(1);
        }

        let mut private_key_file = "".to_string();
        if args.len() == 5 {
            private_key_file = args[4].clone();
        }

        match start_ssh_driver(args[2].clone(), args[3].clone(), private_key_file, logger, middleware).await {
            Err(e) => {
                println!("Something failed: {}", e);
                std::process::exit(1);
            },
            _ => Ok(()),
        }
    }
}

async fn start_ssh_driver(host: String, run_command: String, private_key_file: String, logger: Option<logger::Logger>, middleware: Middleware) -> Result<(), Box<dyn std::error::Error>> {
    // Logger
    let logger_o_ptr = Arc::new(Mutex::new(logger));
    let logger_ptr_a = Arc::clone(&logger_o_ptr);
    let logger_ptr_b = Arc::clone(&logger_o_ptr);
    
    // Args
    let mut args = Vec::new();
    let priv_key_arg = "-i".to_owned();
    if !private_key_file.eq("") {
        args.push(&priv_key_arg);
        args.push(&private_key_file);
    }
    args.push(&host);
    args.push(&run_command);

    // Start Process
    let mut guard = logger_ptr_b.lock().await; {
        if guard.is_some() {
            if guard.as_mut().unwrap().debug_info(&format!("Initialize SSH with Command=ssh Args={:?}", &args).to_string()).is_err() {/* ignored */}
        }
        drop(guard);
    }
    let mut child = match Command::new("ssh")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .kill_on_drop(true)
        .spawn() 
    {
        Ok(c) => c,
        Err(e) => return Err(format!("Unable to start process `ssh`. {}", e).into()),
    };

    let stdout = child.stdout.take().expect("child did not have a handle to stdout");
    let stderr = child.stderr.take().expect("child did not have a handle to stderr");
    let stdin = child.stdin.take().expect("child did not have a handle to stdin");

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();
    let mut stdin_writer = BufWriter::new(stdin);
    let mut stdin_reader = BufReader::new(io::stdin()).lines();

    loop {
        tokio::select! {
            result = stdin_reader.next_line() => {
                match result {
                    Ok(Some(mut line)) => {
                        let oline = line.clone();

                        match middleware.handle_out(line.into()) {
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

                        let mut guard = logger_ptr_a.lock().await; {
                            if guard.is_some() {
                                if guard.as_mut().unwrap().debug_outgoing(&line.clone()).is_err() {/* ignored */}
                            }
                            drop(guard);
                        };

                        if !oline.eq("") && line.eq("") {
                            let mut guard = logger_ptr_a.lock().await; {
                                if guard.is_some() {
                                    if guard.as_mut().unwrap().debug_info(&format!("Filtered out by middleware[message_out]: {}", oline)).is_err() {/* ignored */}
                                }
                                drop(guard);
                            };
                            continue;
                        } else if !oline.eq(&line) {
                            let mut guard = logger_ptr_a.lock().await; {
                                if guard.is_some() {
                                    if guard.as_mut().unwrap().debug_info(&format!("Changed by middleware[message_out]: '{}' -> '{}'", oline, line)).is_err() {/* ignored */}
                                }
                                drop(guard);
                            };
                        }

                        line.push_str("\n");

                        match stdin_writer.write_all(line.as_bytes()).await {
                            Ok(_) => {},
                            Err(e) => {
                                let mut guard = logger_ptr_a.lock().await; {
                                    if guard.is_some() {
                                        if guard.as_mut().unwrap().debug_error(&format!("Failed to write to remote stdin: {}", e)).is_err() {/* ignored */}
                                    }
                                    drop(guard);
                                };
                            },
                        };

                        match stdin_writer.flush().await {
                            Ok(_) => {},
                            Err(e) => {
                                let mut guard = logger_ptr_a.lock().await; {
                                    if guard.is_some() {
                                        if guard.as_mut().unwrap().debug_error(&format!("Failed to flush to remote stdin: {}", e)).is_err() {/* ignored */}
                                    }
                                    drop(guard);
                                };
                            },
                        };
                    },
                    Err(e) => {
                        let mut guard = logger_ptr_a.lock().await; {
                            if guard.is_some() {
                                if guard.as_mut().unwrap().debug_error(&format!("Failed to read from local stdin: {}", e)).is_err() {/* ignored */}
                            }
                            drop(guard);
                        };
                        break;
                    },
                    _ => (),
                }
            }
            result = stdout_reader.next_line() => {
                match result {
                    Ok(Some(mut line)) => {
                        let oline = line.clone();

                        match middleware.handle_in(line.into()) {
                            Ok(new_line) => {
                                line = new_line;
                            },
                            Err(e) => {
                                let mut guard = logger_ptr_a.lock().await; {
                                    if guard.is_some() {
                                        if guard.as_mut().unwrap().debug_error(&format!("Failed to apply incomming middleware: {}", e)).is_err() {/* ignored */}
                                    }
                                    drop(guard);
                                };
                                continue;
                            },
                        };

                        if !oline.eq("") && line.eq("") {
                            let mut guard = logger_ptr_a.lock().await; {
                                if guard.is_some() {
                                    if guard.as_mut().unwrap().debug_info(&format!("Filtered out by middleware[message_in]: {}", oline)).is_err() {/* ignored */}
                                }
                                drop(guard);
                            };
                            continue;
                        } else if !oline.eq(&line) {
                            let mut guard = logger_ptr_a.lock().await; {
                                if guard.is_some() {
                                    if guard.as_mut().unwrap().debug_info(&format!("Changed by middleware[message_in]: '{}' -> '{}'", oline, line)).is_err() {/* ignored */}
                                }
                                drop(guard);
                            };
                        }

                        let mut guard = logger_ptr_a.lock().await; {
                            if guard.is_some() {
                                if guard.as_mut().unwrap().debug_incomming(&line.clone()).is_err() {/* ignored */}
                            }
                            drop(guard);
                        }
                        println!("{}", line);
                    },
                    Err(e) => {
                        let mut guard = logger_ptr_a.lock().await; {
                            if guard.is_some() {
                                if guard.as_mut().unwrap().debug_error(&format!("Failed to read from remote stdout: {}", e)).is_err() {/* ignored */}
                            }
                            drop(guard);
                        };
                        break;
                    },
                    _ => (),
                }
            }
            result = stderr_reader.next_line() => {
                match result {
                    Ok(Some(mut line)) => {
                        let oline = line.clone();

                        match middleware.handle_in(line.into()) {
                            Ok(new_line) => {
                                line = new_line;
                            },
                            Err(e) => {
                                let mut guard = logger_ptr_a.lock().await; {
                                    if guard.is_some() {
                                        if guard.as_mut().unwrap().debug_error(&format!("Failed to apply incomming middleware: {}", e)).is_err() {/* ignored */}
                                    }
                                    drop(guard);
                                };
                                continue;
                            },
                        };

                        if !oline.eq("") && line.eq("") {
                            let mut guard = logger_ptr_a.lock().await; {
                                if guard.is_some() {
                                    if guard.as_mut().unwrap().debug_info(&format!("Filtered out by middleware[message_in]: {}", oline)).is_err() {/* ignored */}
                                }
                                drop(guard);
                            };
                            continue;
                        } else if !oline.eq(&line) {
                            let mut guard = logger_ptr_a.lock().await; {
                                if guard.is_some() {
                                    if guard.as_mut().unwrap().debug_info(&format!("Changed by middleware[message_in]: '{}' -> '{}'", oline, line)).is_err() {/* ignored */}
                                }
                                drop(guard);
                            };
                        }

                        let mut guard = logger_ptr_a.lock().await; {
                            if guard.is_some() {
                                if guard.as_mut().unwrap().debug_incomming(&line.clone()).is_err() {/* ignored */}
                            }
                            drop(guard);
                        }
                        println!("{}", line);
                    },
                    Err(e) => {
                        let mut guard = logger_ptr_a.lock().await; {
                            if guard.is_some() {
                                if guard.as_mut().unwrap().debug_error(&format!("Failed to read from remote stderr: {}", e)).is_err() {/* ignored */}
                            }
                            drop(guard);
                        };
                        break;
                    },
                    _ => (),
                }
            }
            result = child.wait() => {
                match result {
                    Ok(exit_code) => println!("Child process exited with {}", exit_code),
                    _ => (),
                }
                break // child process exited
            }
        };
    };

    return Ok(())
}