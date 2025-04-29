#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

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
    let reader = BufReader::new(&stream);
    let request_line = match reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => return Err(e),
        None => return Ok(()),
    };

    let response = match request_line.as_str() {
        "GET / HTTP/1.1" => String::from("HTTP/1.1 200 OK\r\n\r\n"),
        req if req.starts_with("GET /echo/") => handle_echo_path(&request_line),
        _ => String::from("HTTP/1.1 404 Not Found\r\n\r\n"),
    };

    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}

fn handle_echo_path(request_line: &str) -> String {
    let echo_content = &request_line["GET /echo/".len()..request_line.len() - " HTTP/1.1".len()];
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        echo_content.len(),
        echo_content
    )
}
