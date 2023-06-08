use crate::request::HTTPRequest;
use crate::response::{HTTPResponse, HTTPStatusCode};
use std::collections::HashMap;
use std::fmt::Display;

pub struct Router {
    routes: HashMap<String, Box<dyn Fn(&HTTPRequest, &mut HTTPResponse)>>,
}

fn router_key(method: &HTTPMethod, uri: &str) -> String {
    format!("{}::{}", method, uri)
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<F>(&mut self, method: HTTPMethod, uri: &str, handler: F)
    where
        F: Fn(&HTTPRequest, &mut HTTPResponse) + 'static,
    {
        self.routes
            .insert(router_key(&method, uri), Box::new(handler));
    }

    pub fn handle_request(&self, request: &HTTPRequest, response: &mut HTTPResponse) {
        let handler = self
            .routes
            .get(&router_key(&request.method, request.uri.as_str()));
        match handler {
            Some(handler) => handler(request, response),
            None => {
                response.set_status_code(HTTPStatusCode::NotFound);
                response.write_response("Not Found");
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HTTPMethod {
    pub fn parse_method(method_string: &str) -> Result<HTTPMethod, Box<dyn std::error::Error>> {
        match method_string {
            "GET" => Ok(HTTPMethod::GET),
            "POST" => Ok(HTTPMethod::POST),
            "PUT" => Ok(HTTPMethod::PUT),
            "DELETE" => Ok(HTTPMethod::DELETE),
            _ => Err("Invalid HTTP method".into()),
        }
    }
}

impl Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method = match self {
            HTTPMethod::GET => "GET",
            HTTPMethod::POST => "POST",
            HTTPMethod::PUT => "PUT",
            HTTPMethod::DELETE => "DELETE",
        };
        write!(f, "{}", method)
    }
}

#[cfg(test)]
mod test {
    use super::HTTPMethod;
    #[test]
    fn should_parse_an_http_method() {
        let method = HTTPMethod::parse_method("GET").unwrap();
        assert_eq!(method, HTTPMethod::GET);
    }
    #[test]
    fn should_add_route() {
        let mut router = super::Router::new();
        router.add_route(HTTPMethod::GET, "/", |_, _| {});
        assert_eq!(router.routes.len(), 1);
    }
}
