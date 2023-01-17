
use std::process::{Stdio, Command};

use async_trait::async_trait;

use crate::logger;

use super::Driver;

pub struct SshDriver {}

#[async_trait]
impl Driver for SshDriver {
    async fn run(&self, args: Vec<String>, logger: Option<logger::Logger>) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() != 4 && args.len() != 5 {
            println!("Invalid number of arguments. Please use: drawbridge ssh <url> <run-command> [private-key-file]");
            std::process::exit(1);
        }

        let mut private_key_file = "".to_string();
        if args.len() == 5 {
            private_key_file = args[4].clone();
        }

        match start_ssh_driver(args[2].clone(), args[3].clone(), private_key_file).await {
            Err(e) => {
                println!("Something failed: {}", e);
                std::process::exit(1);
            },
            _ => Ok(()),
        }
    }
}

// Starts the ssh bridge driver
async fn start_ssh_driver(host: String, run_command: String, private_key_file: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Vec::new();
    
    let priv_key_arg = "-i".to_owned();

    if !private_key_file.eq("") {
        args.push(&priv_key_arg);
        args.push(&private_key_file);
    }

    args.push(&host);
    args.push(&run_command);

    let mut child = Command::new("ssh")
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()?;

    child.wait()?;

    return Ok(());
}
