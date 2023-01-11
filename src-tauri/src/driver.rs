use async_trait::async_trait;

pub mod ssh;
pub mod ptcec;

#[async_trait]
pub trait Driver {
    async fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn get_driver(driver_name: String) -> Option<Box<dyn Driver>> {
    if driver_name == "ptcec" {
        return Some(Box::new(ptcec::PtcecDriver{}));
    }

    if driver_name == "ssh" {
        return Some(Box::new(ssh::SshDriver{}));
    }

    return None;
}