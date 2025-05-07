// router.rs
use crate::constants;
use crate::handlers::{handle_get_file, handle_post_file, handle_user_agent};
use crate::request::Request;
use crate::response::Response;

pub enum Route {
    Root,
    Echo(String),
    UserAgent,
    GetFile(String),
    PostFile(String),
    NotFound,
}

pub fn parse_route(method: &str, path: &str) -> Route {
    match (method, path) {
        ("GET", "/") => Route::Root,
        ("GET", "/user-agent") => Route::UserAgent,
        ("GET", path) if path.starts_with("/echo/") => Route::Echo(path[6..].to_string()),
        ("GET", path) if path.starts_with("/files/") => Route::GetFile(path.to_string()),
        ("POST", path) if path.starts_with("/files/") => Route::PostFile(path.to_string()),
        _ => Route::NotFound,
    }
}

pub fn handle_route(route: Route, request: &Request) -> Response {
    match route {
        Route::Root => Response::new(constants::HTTP_OK),
        Route::Echo(content) => Response::new(constants::HTTP_OK).with_text_body(&content),
        Route::UserAgent => handle_user_agent(request.headers.get("user-agent")),
        Route::GetFile(path) => handle_get_file(&path),
        Route::PostFile(path) => handle_post_file(&path, &request.body),
        Route::NotFound => Response::new(constants::HTTP_NOT_FOUND),
    }
}
