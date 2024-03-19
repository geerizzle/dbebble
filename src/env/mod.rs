use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub(crate) struct APIKeys {
    pub(crate) client_id: String,
    pub(crate) api_key: String,
}

impl Default for APIKeys {
    fn default() -> Self {
        let file: Result<File, std::io::Error> = File::open(".env");
        if file.is_err() {
            panic!(".env file is not found")
        }
        let buffer = BufReader::new(file.unwrap());
        let (mut client_id, mut api_key) = (String::new(), String::new());
        for line in buffer.lines() {
            let line = line.unwrap();
            let (key, value) = line.split_at(line.find("=").unwrap());
            if key == "CLIENT_ID" {
                client_id = value[1..].to_string();
            }
            if key == "API_KEY" {
                api_key = value[1..].to_string();
            }
        }
        Self { client_id, api_key }
    }
}
