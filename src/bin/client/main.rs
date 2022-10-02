use std::io::prelude::*;
use std::str::from_utf8;
use std::net::{TcpStream, SocketAddr};

fn main() {
    // let conn = TcpStream::connect("localhost:3333");
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected");

            let data = get_input();
            stream.write(&data.into_bytes()).unwrap();
            println!("Sent data");

            let mut data = [0 as u8; 8];
            match stream.read(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("Received: {}", text);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
    get_input();
}

fn get_input() -> String {
    let mut line = String::new();
    let b = std::io::stdin().read_line(&mut line).unwrap();
    line
}
