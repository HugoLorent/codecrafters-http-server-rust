// errors.rs
use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur in the HTTP server
#[derive(Error, Debug)]
pub enum HttpError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid HTTP request: {0}")]
    InvalidRequest(String),

    #[error("Path traversal attempt detected: {0}")]
    PathTraversal(String),

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("Invalid Content-Length: {0}")]
    InvalidContentLength(String),

    #[error("Malformed request header: {0}")]
    MalformedHeader(String),

    #[error("Directory not specified")]
    DirectoryNotSpecified,
}

/// Result type alias for HTTP server operations
pub type Result<T> = std::result::Result<T, HttpError>;
