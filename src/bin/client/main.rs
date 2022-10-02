use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::io::prelude::*;
use std::str::from_utf8;
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

fn get_input() -> String {
    let mut line = String::new();
    let _b = std::io::stdin().read_line(&mut line).unwrap();
    line
}

fn network_recv_thread(stream: Arc<Mutex<TcpStream>>) {
    let mut data = [0_u8, 50];
    loop {
        let d = stream.lock().unwrap().read(&mut data);
        match d {
            Ok(_) => {
                let text = from_utf8(&data).unwrap();
                println!("Recieved: {}", text);
            }
            Err(_e) => {
                // eprintln!("Error: {}", _e);
            }
        }
    }
}

fn main() {
    let stream = Arc::new(Mutex::new(TcpStream::connect("localhost:3333").unwrap()));
    let _ = stream.lock().unwrap().set_nonblocking(true);
    let stream2 = Arc::clone(&stream);
    let network_handle = thread::spawn(move || {
        network_recv_thread(stream2);
    });

    loop {
        let input = get_input();
        if input.eq("exit\n") {
            break;
        } else {
            println!("Sending: {}", input);
            let _ = stream.lock().unwrap().write(&input.into_bytes());
        }
    }
    network_handle.join().expect("PANIC!!!!");
}
