#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

const HTTP_OK: &str = "HTTP/1.1 200 OK";
const HTTP_NOT_FOUND: &str = "HTTP/1.1 404 Not Found";
const HTTP_BAD_REQUEST: &str = "HTTP/1.1 400 Bad Request";
const CONTENT_TYPE_PLAIN: &str = "Content-Type: text/plain";
const CRLF: &str = "\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                if let Err(e) = handle_connection(stream) {
                    eprintln!("Error handling connection: {}", e);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(&stream);

    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

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

    let parts: Vec<&str> = request_line.split_whitespace().collect();

    let response = if parts.len() >= 3 {
        let method = parts[0];
        let path = parts[1];

        match (method, path) {
            ("GET", "/") => format!("{}{}{}{}", HTTP_OK, CRLF, CRLF, CRLF),
            ("GET", p) if p.starts_with("/echo/") => handle_echo_endpoint(p),
            ("GET", "/user-agent") => handle_user_agent_endpoint(&headers),
            _ => format!("{}{}{}", HTTP_NOT_FOUND, CRLF, CRLF),
        }
    } else {
        format!("{}{}{}", HTTP_BAD_REQUEST, CRLF, CRLF)
    };

    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn handle_echo_endpoint(path: &str) -> String {
    let echo_content = &path[6..]; // 6 = "/echo/".len()
    format!(
        "{}{}{}{}{}{}{}{}",
        HTTP_OK,
        CRLF,
        CONTENT_TYPE_PLAIN,
        CRLF,
        format_args!("Content-Length: {}", echo_content.len()),
        CRLF,
        CRLF,
        echo_content
    )
}

fn handle_user_agent_endpoint(headers: &HashMap<String, String>) -> String {
    match headers.get("user-agent") {
        Some(user_agent) => format!(
            "{}{}{}{}{}{}{}{}",
            HTTP_OK,
            CRLF,
            CONTENT_TYPE_PLAIN,
            CRLF,
            format_args!("Content-Length: {}", user_agent.len()),
            CRLF,
            CRLF,
            user_agent
        ),
        None => format!("{}{}{}", HTTP_BAD_REQUEST, CRLF, CRLF),
    }
}
