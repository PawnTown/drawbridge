
use std::process::{Stdio, Command};

use async_trait::async_trait;

use super::Driver;

pub struct SshDriver {}

#[async_trait]
impl Driver for SshDriver {
    async fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() != 4 {
            println!("Invalid number of arguments. Please use: drawbridge ssh <url> <run-command>");
            std::process::exit(1);
        }

        match start_ssh_driver(args[2].clone(), args[3].clone()).await {
            Err(e) => {
                println!("Something failed: {}", e);
                std::process::exit(1);
            },
            _ => Ok(()),
        }
    }
}

// Starts the ssh bridge driver
async fn start_ssh_driver(host: String, run_command: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("ssh")
        .args(&[
            &host,
            &run_command,
        ])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()?;

    child.wait()?;

    return Ok(());
}
