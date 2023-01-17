use std::{time::SystemTime, path::PathBuf};

pub struct Logger {
    file: std::fs::File,
    log_file: PathBuf,
}

impl Logger {

    pub fn new(log_file: PathBuf) -> Result<Logger, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::create(&log_file)?;
        Ok(Logger {
            file,
            log_file,
        })
    }

    pub fn info(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!("info  |> {}", message))
    }

    pub fn error(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!("error |> {}", message))
    }

    pub fn debug(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!("warn  |> {}", message))
    }

    fn write(&mut self, line: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Write;

        let sys_time = SystemTime::now();
        let full_line = format!("[{:?}] {}", humantime::format_rfc3339(sys_time).to_string(), line);
        self.file.write_all(full_line.as_bytes())?;
        self.file.sync_all()?;
        return Ok(());
    }
    
}

