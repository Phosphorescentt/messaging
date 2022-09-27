use std::io::prelude::*;
use std::str::from_utf8;
use std::net::{TcpStream, SocketAddr};

fn main() -> std::io::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 25565));
    let mut stream = TcpStream::connect(addr)?;

    stream.write(&[1, 2])?;

    let mut data = [0 as u8; 1];
    match stream.read_exact(&mut data) {
        Ok(_) => {
            let text = from_utf8(&data).unwrap();
            println!("Data: {}", text);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    Ok(())
}
