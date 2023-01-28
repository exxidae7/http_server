use http::Method;
use http::Request;
use website_handler::WebsiteHandler;
mod http;
use std::env;
mod server;

use server::Server;
mod website_handler;
fn main() {
    let default_path = format!("{}/public" ,env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}