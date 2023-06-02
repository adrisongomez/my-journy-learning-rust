use std::{net::{TcpListener, TcpStream}, io::Read};

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_incoming(stream)
    }
    println!("Hello, world!");
    Ok(())
}

fn handle_incoming(stream: Result<TcpStream, std::io::Error>){
    let mut value = String::new();
    let mut conn = Box::new(stream.unwrap());
    Read::read_to_string(conn.as_mut(), &mut value).expect("Error trying to read the connection");
    println!("New message: {}", value);
}
