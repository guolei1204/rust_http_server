use std::fs;

use crate::{
    http::{request::Request, Method, Response, StatusCode},
    server::Handler,
};
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_from_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(&path) {
            Ok(p) => {
                if p.starts_with(&self.public_path) {
                    return fs::read_to_string(path).ok();
                } else {
                    println!("Directory Tranversal Attack Attemped :{}", file_path);
                    return None;
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> crate::http::Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, self.read_from_file("index.html")),
                path => match self.read_from_file(path) {
                    Some(c) => Response::new(StatusCode::OK, Some(c)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
