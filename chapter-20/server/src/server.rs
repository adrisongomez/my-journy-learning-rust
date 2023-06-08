use crate::router::Router;
use std::io::prelude::*;

pub struct Server {
    port: u16,
    router: Router,
}

impl Server {
    pub fn new(port: u16, router: Router) -> Server {
        Server { port, router }
    }

    pub fn listen_and_serve(&self) -> std::io::Result<()> {
        let listener = std::net::TcpListener::bind(format!("localhost:{}", self.port))?;
        println!("Server running on port {}", self.port);
        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [0; 1024];
            stream.read(&mut buffer)?;
            let request = crate::request::HTTPRequest::new(std::str::from_utf8(&buffer).unwrap());
            let mut response = crate::response::HTTPResponse::new(stream);
            self.router.handle_request(&request, &mut response);
            response.write_response_to_stream();
        }
        Ok(())
    }
}
