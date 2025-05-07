// request.rs
use crate::errors::{HttpError, Result};
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
    pub fn parse(stream: &mut TcpStream) -> Result<Self> {
        let mut reader = BufReader::new(stream);
        let mut request_line = String::new();
        reader.read_line(&mut request_line).map_err(HttpError::Io)?;

        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(HttpError::InvalidRequest(
                "Request line must contain method, path and HTTP version".to_string(),
            ));
        }

        let method = parts[0].to_string();
        let path = parts[1].to_string();

        // Read headers
        let mut headers = HashMap::new();
        loop {
            let mut header_line = String::new();
            reader.read_line(&mut header_line).map_err(HttpError::Io)?;

            if header_line.trim().is_empty() {
                break;
            }

            match header_line.find(':') {
                Some(idx) => {
                    let key = header_line[..idx].trim().to_lowercase().to_string();
                    let value = header_line[idx + 1..].trim().to_string();
                    headers.insert(key, value);
                }
                None => {
                    return Err(HttpError::MalformedHeader(header_line));
                }
            }
        }

        // Read body
        let mut body = Vec::new();
        if let Some(content_length) = headers.get("content-length") {
            let length = content_length
                .parse::<usize>()
                .map_err(|_| HttpError::InvalidContentLength(content_length.clone()))?;

            let mut buffer = vec![0; length];
            reader.read_exact(&mut buffer).map_err(HttpError::Io)?;
            body = buffer;
        }

        Ok(Self {
            method,
            path,
            headers,
            body,
        })
    }
}
