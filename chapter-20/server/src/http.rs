pub mod http {
    use std::net::TcpStream;

    #[derive(Debug)]
    pub struct HTTPRequest {
        pub method: String,
        pub uri: String,
        raw_request: String,
    }

    impl HTTPRequest {
        pub fn new(raw_content: &str) -> HTTPRequest {
            let lines = raw_content
                .lines()
                .next()
                .unwrap()
                .split(" ")
                .collect::<Vec<&str>>();
            let method = lines[0].to_string();
            let uri = lines[1].to_string();
            HTTPRequest {
                method,
                uri,
                raw_request: raw_content.to_string(),
            }
        }
    }

    #[derive(Debug)]
    pub enum HTTPStatusCode {
        OK,
        CreatedOk,
        NotFound,
    }

    impl HTTPStatusCode {
        fn get_value(&self) -> (u16, &'static str) {
            match self {
                HTTPStatusCode::OK => (200, "OK"),
                HTTPStatusCode::CreatedOk => (201, "Created"),
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

        pub fn write_response(&mut self, content: &str) {
            self.content = content.to_string();
        }

        pub fn build_response(&self) -> String {
            let (status_code, msg) = match &self.status_code {
                Some(x) => (x.get_value().0, x.get_value().1),
                None => (500, "Internal Server Error"),
            };
            format!(
                "HTTP/1.1 {} {}\r\n\r\n{}",
                status_code, msg, self.content
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::http::HTTPRequest;

    #[test]
    fn should_parse_an_http_request() {
        let raw_content = "GET / HTTP/1.1\r\n\r\n";
        let request = HTTPRequest::new(raw_content);
        assert_eq!(request.method, "GET");
        assert_eq!(request.uri, "/");
    }
}
