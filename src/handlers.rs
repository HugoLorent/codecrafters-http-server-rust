use std::env;
use std::fs;

use crate::constants::CONTENT_TYPE_OCTET_STREAM;
use crate::constants::CONTENT_TYPE_PLAIN;
use crate::constants::{HTTP_BAD_REQUEST, HTTP_CREATED, HTTP_NOT_FOUND, HTTP_OK};
use crate::response::Response;

pub fn handle_user_agent(user_agent: Option<&String>) -> Response {
    match user_agent {
        Some(agent) => Response::new(HTTP_OK)
            .with_header("Content-Type", CONTENT_TYPE_PLAIN)
            .with_text_body(agent),
        None => Response::new(HTTP_BAD_REQUEST),
    }
}

pub fn handle_get_file(path: &str) -> Response {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        return Response::new(HTTP_BAD_REQUEST);
    }

    let directory = &args[2];
    let file_path = path.strip_prefix("/files/").unwrap_or("");
    let full_path = format!("{}/{}", directory, file_path);

    match fs::read(full_path) {
        Ok(content) => Response::new(HTTP_OK)
            .with_header("Content-Type", CONTENT_TYPE_OCTET_STREAM)
            .with_body(content),
        Err(_) => Response::new(HTTP_NOT_FOUND),
    }
}

pub fn handle_post_file(path: &str, content: &[u8]) -> Response {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        return Response::new(HTTP_BAD_REQUEST);
    }

    let directory = &args[2];
    let file_path = path.strip_prefix("/files/").unwrap_or("");
    let full_path = format!("{}/{}", directory, file_path);

    match fs::write(&full_path, content) {
        Ok(_) => Response::new(HTTP_CREATED),
        Err(e) => {
            eprintln!("Error writing file {}: {}", full_path, e);
            Response::new(HTTP_BAD_REQUEST)
        }
    }
}
