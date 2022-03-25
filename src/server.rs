use std::{io::Read, net::TcpListener};

use crate::http::{
    request::{ParseError, Request},
    Response, StatusCode,
};

pub struct Server {
    addr: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server {
    pub(crate) fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self, mut handler: impl Handler) {
        let lisener = TcpListener::bind(&self.addr).unwrap();
        let mut buf = [0; 1024];
        loop {
            match lisener.accept() {
                Ok((mut stream, _)) => match stream.read(&mut buf) {
                    Ok(_) => {
                        println!("Receive a request :{}", String::from_utf8_lossy(&buf));
                        let response = match Request::try_from(&buf[..]) {
                            Ok(r) => handler.handle_request(&r),
                            Err(e) => handler.handle_bad_request(&e),
                        };

                        if let Err(e) = response.send(&mut stream) {
                            println!("failed to send response :{}", e);
                        }
                    }
                    Err(e) => println!("failed to read from connection {}", e),
                },
                Err(e) => println!("failed to establish a  connection {}", e),
            }
        }
    }
}
