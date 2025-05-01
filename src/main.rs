mod constants;
mod handlers;
mod request;
mod response;

use std::net::{TcpListener, TcpStream};
use std::thread;

use constants::HTTP_BAD_REQUEST;
use handlers::{handle_echo, handle_get_file, handle_post_file, handle_user_agent};
use request::Request;
use response::Response;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    println!("Accepted new connection");
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let request = match Request::parse(&mut stream) {
        Ok(req) => req,
        Err(_) => {
            return Response::new(HTTP_BAD_REQUEST).send(&mut stream);
        }
    };

    let response = match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => Response::new(constants::HTTP_OK),
        ("GET", path) if path.starts_with("/echo/") => handle_echo(path),
        ("GET", "/user-agent") => handle_user_agent(request.headers.get("user-agent")),
        ("GET", path) if path.starts_with("/files/") => handle_get_file(path),
        ("POST", path) if path.starts_with("/files/") => handle_post_file(path, &request.body),
        _ => Response::new(constants::HTTP_NOT_FOUND),
    };

    response.send(&mut stream)
}
