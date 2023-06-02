use std::{net::TcpStream, io::Write};

fn main() -> Result<(), std::io::Error> {
    let mut conn = TcpStream::connect("127.0.0.1:8080").expect("Fail connection");
    println!("Connected to the server");
    conn.write(
        "hello from client".as_bytes()
    ).expect("Fail sending the message to the server");
    conn.shutdown(std::net::Shutdown::Both).expect("Fail shutting down the connection");
    println!("Hello, world!");
    Ok(())
}
