use std::env;

use crate::{server::Server, web_handler::WebsiteHandler};

mod http;
mod server;
mod web_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let pub_path: String = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("\n{}\n", pub_path);
    let addr = "127.0.0.1:8080".to_string();
    let server = Server::new(addr.clone());
    server.run(WebsiteHandler::new(pub_path));
    println!("https-server run @ {}", addr);
}
