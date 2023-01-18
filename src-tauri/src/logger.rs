use std::{time::SystemTime, path::PathBuf};

pub struct Logger {
    file: std::fs::File,
}

impl Logger {

    pub fn new(log_file: PathBuf) -> Result<Logger, Box<dyn std::error::Error>> {
        let file = std::fs::File::create(&log_file)?;
        Ok(Logger {
            file,
        })
    }

    pub fn debug_outgoing(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!("|>| {}", message))
    }

    pub fn debug_incomming(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!("|<| {}", message))
    }

    pub fn debug_info(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!("|i| {}", message))
    }

    fn write(&mut self, line: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Write;

        let sys_time = SystemTime::now();
        let full_line = format!("[{:?}] {}\n", humantime::format_rfc3339(sys_time).to_string(), line);
        self.file.write(full_line.as_bytes())?;
        self.file.flush()?;
        return Ok(());
    }
    
}

