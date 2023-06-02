use server::Server;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let mut server = Server::start("8080", Some(20));
    server.set_handler("/".to_string(), hello_handle);
    server.set_handler("/sleep".to_string(), sleep_handle);
    server.listen_and_serve().expect("Error listening");
}

fn hello_handle(stream: &mut TcpStream) {
    let (status_line, filename) = ("HTTP/1.1 200 OK", "./assets/hello.html");
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn sleep_handle(stream: &mut TcpStream) {
    let (status_line, filename) = ("HTTP/1.1 200 OK", "./assets/hello.html");
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    thread::sleep(Duration::from_secs(5));
    stream.write_all(response.as_bytes()).unwrap();
}
