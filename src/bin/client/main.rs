use std::io::prelude::*;
use std::net::{TcpStream, SocketAddr};

fn main() -> std::io::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 25565));
    let mut stream = TcpStream::connect(addr)?;

    stream.write(&[1])?;
    let a = stream.read(&mut [0; 128])?;
    println!("{}", a);
    Ok(())
}
