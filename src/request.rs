use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn parse(stream: &mut TcpStream) -> std::io::Result<Self> {
        let mut reader = BufReader::new(stream);
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid request",
            ));
        }

        let method = parts[0].to_string();
        let path = parts[1].to_string();

        // Read headers
        let mut headers = HashMap::new();
        loop {
            let mut header_line = String::new();
            reader.read_line(&mut header_line)?;

            if header_line.trim().is_empty() {
                break;
            }

            if let Some(idx) = header_line.find(':') {
                let key = header_line[..idx].trim().to_lowercase().to_string();
                let value = header_line[idx + 1..].trim().to_string();
                headers.insert(key, value);
            }
        }

        // Read body
        let mut body = Vec::new();
        if let Some(content_length) = headers.get("content-length") {
            if let Ok(length) = content_length.parse::<usize>() {
                let mut buffer = vec![0; length];
                reader.read_exact(&mut buffer)?;
                body = buffer;
            }
        }

        Ok(Self {
            method,
            path,
            headers,
            body,
        })
    }
}
