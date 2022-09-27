use std::net::{TcpStream, TcpListener, SocketAddr, Shutdown};
use std::io::{Read, Write};
use std::thread;

fn main () {
    let addr = SocketAddr::from(([0, 0, 0, 0], 25565));
    let listener = TcpListener::bind(addr).unwrap();

    let mut stream: TcpStream;

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("New connection: {}", s.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(&mut s)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(stream: &mut TcpStream) {
    let mut data = [0 as u8; 50];

    while match stream.read(&mut data) {
        Ok(size) => {
            println!("{:?}", data);
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occured. Terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
