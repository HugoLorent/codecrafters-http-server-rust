use std::io::{BufRead, BufReader, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let reader = BufReader::new(&stream);
                let request_line = reader.lines().next().unwrap().unwrap();

                let status_line = match request_line.as_str() {
                    "GET / HTTP/1.1" => "HTTP/1.1 200 OK",
                    _ => "HTTP/1.1 404 Not Found",
                };

                let response = format!("{status_line}\r\n\r\n");
                stream.write(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
