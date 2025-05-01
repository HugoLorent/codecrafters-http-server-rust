use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

pub struct Response {
    pub status: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: &str) -> Self {
        Self {
            status: status.to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.headers
            .insert("Content-Length".to_string(), body.len().to_string());
        self.body = body;
        self
    }

    pub fn with_text_body(self, text: &str) -> Self {
        self.with_header("Content-Type", "text/plain")
            .with_body(text.as_bytes().to_vec())
    }

    pub fn send(self, stream: &mut TcpStream) -> std::io::Result<()> {
        let mut response = format!("{}\r\n", self.status);

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");
        stream.write_all(response.as_bytes())?;

        if !self.body.is_empty() {
            stream.write_all(&self.body)?;
        }

        Ok(())
    }
}
