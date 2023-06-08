use crate::router::HTTPMethod;
#[derive(Debug)]
pub struct HTTPRequest {
    pub method: HTTPMethod ,
    pub uri: String,
    #[allow(dead_code)]
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
        let method = HTTPMethod::parse_method(lines[0]).unwrap();
        let uri = lines[1].to_string();
        HTTPRequest {
            method,
            uri,
            raw_request: raw_content.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::HTTPRequest;
    use crate::router::HTTPMethod;

    #[test]
    fn should_parse_an_http_request() {
        let raw_content = "GET / HTTP/1.1\r\n\r\n";
        let request = HTTPRequest::new(raw_content);
        assert_eq!(request.method, HTTPMethod::GET);
        assert_eq!(request.uri, "/");
        assert_eq!(request.raw_request, raw_content)
    }
}