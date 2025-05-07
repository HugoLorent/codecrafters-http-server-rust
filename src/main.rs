// main.rs
mod constants;
mod errors;
mod handlers;
mod request;
mod response;
mod router;
mod thread_pool;

use std::net::{TcpListener, TcpStream};

use constants::HTTP_BAD_REQUEST;
use request::Request;
use response::Response;
use router::{handle_route, parse_route};
use thread_pool::ThreadPool;

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:4221") {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            std::process::exit(1);
        }
    };

    let pool = ThreadPool::new(5); // Create a thread pool with 5 worker threads

    println!("Server started on http://127.0.0.1:4221");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    println!("Accepted new connection");
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let request = match Request::parse(&mut stream) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Error parsing request: {:?}", e);
            return Response::new(HTTP_BAD_REQUEST).send(&mut stream);
        }
    };

    // Router use
    let route = parse_route(&request.method, &request.path);
    let response = handle_route(route, &request);

    response.send(&mut stream)
}
