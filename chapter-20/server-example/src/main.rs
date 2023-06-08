use server::request::HTTPRequest;
use server::response::{HTTPResponse, HTTPStatusCode};
use server::server::Server;
use server::router::{Router, HTTPMethod};
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    let mut router = Router::new();
    router.add_route(HTTPMethod::GET, "/", hello_handle);
    router.add_route(HTTPMethod::GET, "/sleep", sleep_handle);
    let server = Server::new(3000, router);
    server.listen_and_serve().unwrap()
}


fn hello_handle(_: &HTTPRequest, response: &mut HTTPResponse) {
    let contents = fs::read_to_string("./assets/hello.html").unwrap();
    response.set_status_code(HTTPStatusCode::OK);
    response.write_response(&contents);
}

fn sleep_handle(_: &HTTPRequest, response: &mut HTTPResponse) {
    let contents = fs::read_to_string("./assets/sleep.html").unwrap();
    // simulate expensive computation
    thread::sleep(Duration::from_secs(5));
    response.set_status_code(HTTPStatusCode::OK);
    response.write_response(&contents)
}
