use async_trait::async_trait;

use crate::logger;

pub mod connect;
pub mod ssh;
pub mod ptcec;

#[async_trait]
pub trait Driver {
    async fn run(&self, args: Vec<String>, log: Option<logger::Logger>) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn get_driver(driver_name: String) -> Option<Box<dyn Driver>> {
    if driver_name.eq("ptcec") {
        return Some(Box::new(ptcec::PtcecDriver{}));
    }

    if driver_name.eq("ssh") {
        return Some(Box::new(ssh::SshDriver{}));
    }

    if driver_name.eq("connect") {
        return Some(Box::new(connect::ConnectDriver{}));
    }

    return None;
}