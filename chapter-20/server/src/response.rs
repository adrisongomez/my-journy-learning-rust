use std::net::TcpStream;
use std::io::prelude::*;

#[derive(Debug)]
pub enum HTTPStatusCode {
    OK,
    Created,
    NotFound,
}

impl HTTPStatusCode {
    pub fn get_value(&self) -> (u16, &'static str) {
        match self {
            HTTPStatusCode::OK => (200, "OK"),
            HTTPStatusCode::Created => (201, "Created"),
            HTTPStatusCode::NotFound => (404, "Not Found"),
        }
    }
}

#[derive(Debug)]
pub struct HTTPResponse {
    stream: TcpStream,
    content: String,
    status_code: Option<HTTPStatusCode>,
}

impl HTTPResponse {
    pub fn new(stream: TcpStream) -> HTTPResponse {
        HTTPResponse {
            status_code: None,
            stream,
            content: String::new(),
        }
    }

    pub fn set_status_code(&mut self, status_code: HTTPStatusCode) {
        self.status_code = Some(status_code);
    }

    pub fn write_response(&mut self, content: &str) {
        self.content = content.to_string();
    }

    fn build_response(&self) -> String {
        let (status_code, msg) = match &self.status_code {
            Some(x) => (x.get_value().0, x.get_value().1),
            None => (500, "Internal Server Error"),
        };
        format!("HTTP/1.1 {} {}\r\n\r\n{}", status_code, msg, self.content)
    }

    pub fn write_response_to_stream(&mut self) {
        let response = self.build_response();
        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::{net::TcpListener, thread, time::Duration};
    use std::io::prelude::*;

    use super::{HTTPResponse, HTTPStatusCode};

    #[test]
    fn should_create_an_http_response() {
        // create a thread an open a tcp listener
        thread::spawn(|| {
            let listener = TcpListener::bind("localhost:3000").unwrap();
            for stream in listener.incoming() {
                print!("{:?}", stream);
            }
        });
        thread::sleep(Duration::from_secs(1));
        let stream = std::net::TcpStream::connect("localhost:3000").unwrap();
        let http_response = HTTPResponse::new(stream);

        assert_eq!(
            http_response.build_response(),
            "HTTP/1.1 500 Internal Server Error\r\n\r\n"
        );
    }

    #[test]
    fn should_create_an_http_response_write_in_channel() {
        // create a thread an open a tcp listener
        let listener = TcpListener::bind("localhost:3001").unwrap();
        thread::spawn(|| {
            thread::sleep(Duration::from_secs(1));
            let mut stream = std::net::TcpStream::connect("localhost:3001").unwrap();
            stream.write("GET / HTTP/1.1\r\n\r\n".as_bytes()).unwrap();
        });

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let mut http_response = HTTPResponse::new(stream);
            http_response.write_response("Hello World");
            http_response.set_status_code(HTTPStatusCode::OK);
            http_response.write_response_to_stream();
            break;
        }
        drop(listener); // close the listener socket
    }

    #[test]
    fn should_write_response() {
        thread::spawn(|| {
            let listener = TcpListener::bind("localhost:3002").unwrap();
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                let response = "HTTP/1.1 200 OK\r\n\r\nHello World";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        });
        thread::sleep(Duration::from_secs(1));
        let stream = std::net::TcpStream::connect("localhost:3002").unwrap();
        let mut http_response = HTTPResponse::new(stream);
        http_response.write_response("Hello World");
        http_response.set_status_code(HTTPStatusCode::OK);
        http_response.write_response_to_stream();
        assert_eq!(http_response.content, "Hello World");
    }
}
