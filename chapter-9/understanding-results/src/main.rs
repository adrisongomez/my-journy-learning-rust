use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let _greeting_file_result = File::open("hello.txt"); // This return a Result.

    match _greeting_file_result {
        Ok(_) => println!("File exists"),
        Err(_) => println!("File doesn't exists"),
    }

    // match assigment
    let _greeting_file = match _greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let _greeting_file_result = File::open("hello.txt").unwrap(); 
    let _greeting_file_result = File::open("hello.txt").expect("Fail");
    // Others way to handle when is Err -> `unwrap` and `expect` they call
    // panic! and unwrap don't require args and expect use a msg argument.
}

use std::io::{self, Read};

fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn __read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn ___read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

use std::fs;

fn ____read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
