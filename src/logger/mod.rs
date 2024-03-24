use std::fs::OpenOptions;
use std::io::Write;

use chrono::Local;

pub(crate) struct Logger {
    buffer: String,
}

impl Logger {
    pub(crate) fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub(crate) fn log(&mut self, message: &str) {
        let time = Local::now().to_string();
        self.buffer
            .push_str(format!("{} | LOG: {}\n", time, message).as_str());
    }

    pub(crate) fn write_to_file(&mut self) -> std::io::Result<()> {
        let mut log_file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("log.txt")?;

        log_file.write_all(self.buffer.as_bytes())?;
        log_file.sync_all()?;
        Ok(())
    }

    pub(crate) fn get_buffer(&self) -> &str {
        &self.buffer
    }
}
