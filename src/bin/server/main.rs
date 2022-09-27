use std::net::{TcpStream, TcpListener, SocketAddr};
use std::io::{Read, Write};

fn main () {
    let addr = SocketAddr::from(([127, 0, 0, 1], 25565));
    let listener = TcpListener::bind(addr).unwrap();

    let mut stream: TcpStream;
    for stream in listener.incoming() {
        let mut data = [0 as u8; 128];
        match stream {
            Ok(mut s) => {
                println!("New connection: {}", s.peer_addr().unwrap());
                match s.read(&mut data) {
                    Ok(d) => { println!("Data: {}", d) }
                    Err(e) => { eprintln!("Error: {}", e) }
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
}
