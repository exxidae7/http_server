use crate::http::{ StatusCode, Method };
use std::fs;
use super::http::{ Request, Response };
use super::server::Handler;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET =>
                match request.path() {
                    "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                    _ => Response::new(StatusCode::NotFound, None),
                }
            Method::POST => todo!(),
            Method::PUT => todo!(),
            Method::DELETE => todo!(),
            Method::HEAD => todo!(),
            Method::CONNECT => todo!(),
            Method::OPTIONS => todo!(),
            Method::TRACE => todo!(),
            Method::PATCH => todo!(),
        }
    }
}