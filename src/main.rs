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
    // Configure a timeout for connection if necessary
    stream.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;

    let mut keep_alive = true;

    while keep_alive {
        let request = match Request::parse(&mut stream) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Error parsing request: {:?}", e);
                Response::new(HTTP_BAD_REQUEST).send(&mut stream)?;
                break;
            }
        };

        // Determine if connection should stay open
        keep_alive = should_keep_alive(&request);

        // Router use
        let route = parse_route(&request.method, &request.path);
        let mut response = handle_route(route, &request);

        // Add appropriate Connection header
        if keep_alive {
            response = response.with_header("Connection", "keep-alive");
        } else {
            response = response.with_header("Connection", "close");
        }

        response.send(&mut stream)?;

        if !keep_alive {
            break;
        }
    }

    Ok(())
}

fn should_keep_alive(request: &Request) -> bool {
    if let Some(connection) = request.headers.get("connection") {
        return connection.to_lowercase() != "close";
    }

    // In HTTP/1.1, connections are persistents by default
    true
}
