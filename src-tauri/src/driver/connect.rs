use async_trait::async_trait;
use serde_json::Value;

use crate::{storage, logger, middleware::Middleware};

use super::{Driver, ptcec, ssh};

pub struct ConnectDriver {}

#[async_trait]
impl Driver for ConnectDriver {
    async fn run(&self, args: Vec<String>, logger: Option<logger::Logger>, _: Middleware) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() != 3 {
            println!("Invalid number of arguments. Please use: drawbridge connect <remote-id>");
            std::process::exit(1);
        }

        let data = storage::load("remotes".to_string()).unwrap();
        let remotes: Value = serde_json::from_str(&data).unwrap();

        let mut remote: Option<Value> = None;
        for r in remotes.as_array().unwrap() {
            if r["id"].as_str().unwrap().eq(&args[2]) {
                remote = Some(r.clone());
                break;
            }
        }

        match remote {
            Some(r) => {
                let driver_name = r["driver"].as_str().unwrap();

                let middleware = Middleware::new(r["middlewareLua"].as_str().unwrap().to_string());
                
                if driver_name.eq("ptcec") {
                    // alias for: drawbridge ptcec <url> <engine> <mode> <token>
                    let driver = ptcec::PtcecDriver{};
                    return driver.run(vec![
                        args[0].to_string(),
                        driver_name.to_string(),
                        r["url"].as_str().unwrap().to_string(),
                        r["engine"].as_str().unwrap().to_string(),
                        r["mode"].as_str().unwrap().to_string(),
                        r["token"].as_str().unwrap().to_string(),
                    ], logger, middleware).await;
                }
            
                if driver_name.eq("ssh") {
                    // alias for: drawbridge ssh <url> <run-command>
                    let driver = ssh::SshDriver{};
                    return driver.run(vec![
                        args[0].to_string(),
                        driver_name.to_string(),
                        r["url"].as_str().unwrap().to_string(),
                        r["runCommand"].as_str().unwrap().to_string(),
                        r["privateKeyFile"].as_str().unwrap().to_string(),
                    ], logger, middleware).await;
                }

                println!("Driver {} not found.", driver_name);
                std::process::exit(1);
            },
            None => {
                println!("Remote not found.");
                std::process::exit(1);
            }
        }
    }
}
