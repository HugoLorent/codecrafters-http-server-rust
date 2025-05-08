# Rust HTTP Server

A lightweight HTTP server implementation built in Rust as part of the [CodeCrafters Build Your Own HTTP Server](https://codecrafters.io/challenges/http-server) challenge.

## Features

- HTTP/1.1 compliant server with support for persistent connections
- Thread pool for handling concurrent requests
- File operations (GET/POST) with directory traversal protection
- Gzip compression support
- Echo and user-agent endpoints
- Proper error handling with custom error types

## Architecture

The server is organized into modular components:

- `main.rs`: Entry point with connection handling and persistent connection logic
- `request.rs`: HTTP request parsing
- `response.rs`: HTTP response construction
- `router.rs`: Request routing to appropriate handlers
- `handlers.rs`: Endpoint implementations (echo, user-agent, file operations)
- `thread_pool.rs`: Worker thread pool for request processing
- `errors.rs`: Custom error types and error handling
- `constants.rs`: HTTP status codes and content types

## Technical Implementation

- Built with pure Rust and minimal dependencies
- Uses standard library networking primitives
- Thread pool implementation for concurrency
- Secure file handling with path canonicalization to prevent directory traversal attacks
- Support for gzip compression using the `flate2` crate

## Learning Objectives

This project was created as a learning exercise to deepen understanding of:

- HTTP protocol implementation details
- Network programming in Rust
- Concurrent programming with thread pools
- Proper error handling in networked applications
- Safe file system operations

## Usage

To run the server:

```bash
cargo run -- <directory>
