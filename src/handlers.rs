// handlers.rs
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::constants::CONTENT_TYPE_OCTET_STREAM;
use crate::constants::CONTENT_TYPE_PLAIN;
use crate::constants::{HTTP_BAD_REQUEST, HTTP_CREATED, HTTP_NOT_FOUND, HTTP_OK};
use crate::errors::{HttpError, Result};
use crate::response::Response;

pub fn handle_echo(content: &str, accept_encoding: Option<&String>) -> Response {
    match accept_encoding {
        Some(header) => {
            let response = Response::new(HTTP_OK);
            let encodings: Vec<&str> = header.split(",").map(|header| header.trim()).collect();
            let gzip = encodings.iter().find(|&header| *header == "gzip");
            if let Some(_gzip) = gzip {
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(content.as_bytes()).unwrap();
                let compressed_content = encoder.finish().unwrap();

                return response
                    .with_header("Content-Encoding", "gzip")
                    .with_header("Content-Type", CONTENT_TYPE_PLAIN)
                    .with_body(compressed_content);
            }
            response.with_text_body(content)
        }
        None => Response::new(HTTP_OK).with_text_body(content),
    }
}

pub fn handle_user_agent(user_agent: Option<&String>) -> Response {
    match user_agent {
        Some(agent) => Response::new(HTTP_OK)
            .with_header("Content-Type", CONTENT_TYPE_PLAIN)
            .with_text_body(agent),
        None => Response::new(HTTP_BAD_REQUEST),
    }
}

pub fn handle_get_file(path: &str) -> Response {
    match get_file_internal(path) {
        Ok(content) => Response::new(HTTP_OK)
            .with_header("Content-Type", CONTENT_TYPE_OCTET_STREAM)
            .with_body(content),
        Err(err) => {
            eprintln!("Error handling GET file: {:?}", err);
            match err {
                HttpError::FileNotFound(_) => Response::new(HTTP_NOT_FOUND),
                HttpError::PathTraversal(_) | HttpError::PermissionDenied(_) => {
                    Response::new(HTTP_BAD_REQUEST)
                }
                _ => Response::new(HTTP_BAD_REQUEST),
            }
        }
    }
}

fn get_file_internal(path: &str) -> Result<Vec<u8>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        return Err(HttpError::DirectoryNotSpecified);
    }

    let directory = Path::new(&args[2]);
    let file_path = path.strip_prefix("/files/").unwrap_or("");

    // Security against directory traversal
    let target_path = directory.join(file_path);
    let canonical_target = target_path.canonicalize().map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => HttpError::FileNotFound(target_path.to_path_buf()),
        std::io::ErrorKind::PermissionDenied => {
            HttpError::PermissionDenied(target_path.to_path_buf())
        }
        _ => HttpError::Io(e),
    })?;

    let canonical_dir = directory.canonicalize().map_err(HttpError::Io)?;

    // Verify that the path is within the authorized directory
    if !canonical_target.starts_with(&canonical_dir) {
        return Err(HttpError::PathTraversal(format!(
            "Path traversal attempt: {}",
            target_path.display()
        )));
    }

    fs::read(&canonical_target).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => HttpError::FileNotFound(canonical_target),
        std::io::ErrorKind::PermissionDenied => HttpError::PermissionDenied(canonical_target),
        _ => HttpError::Io(e),
    })
}

pub fn handle_post_file(path: &str, content: &[u8]) -> Response {
    match post_file_internal(path, content) {
        Ok(_) => Response::new(HTTP_CREATED),
        Err(err) => {
            eprintln!("Error handling POST file: {:?}", err);
            Response::new(HTTP_BAD_REQUEST)
        }
    }
}

fn post_file_internal(path: &str, content: &[u8]) -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        return Err(HttpError::DirectoryNotSpecified);
    }

    let directory = Path::new(&args[2]);
    let file_path = path.strip_prefix("/files/").unwrap_or("");

    // Security against directory traversal
    let target_path = directory.join(file_path);

    // Create parent directories if needed
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent).map_err(HttpError::Io)?;
    }

    let canonical_dir = directory.canonicalize().map_err(HttpError::Io)?;

    // For files that don't exist yet, check the parent directory
    // to ensure it's within the authorized directory
    if let Some(parent) = target_path.parent() {
        let canonical_parent = parent.canonicalize().map_err(HttpError::Io)?;

        if !canonical_parent.starts_with(&canonical_dir) {
            return Err(HttpError::PathTraversal(format!(
                "Path traversal attempt: {}",
                target_path.display()
            )));
        }
    }

    fs::write(&target_path, content).map_err(HttpError::Io)
}
