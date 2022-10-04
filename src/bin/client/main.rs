use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::io::prelude::*;
use std::str::from_utf8;
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

type TcpMutex = Arc<Mutex<TcpStream>>;
type Buffer = (Vec<u8>, usize);

fn get_input() -> String {
    let mut line = String::new();
    let _b = std::io::stdin().read_line(&mut line).unwrap();
    line
}

fn network_recv_thread(stream: TcpMutex) {
    let (mut data, mut size): Buffer = (Vec::new(), 0);
    loop {
        let d = stream.lock().unwrap().read_to_end(&mut data);
        // println!("{:?}", data);
        match d {
            Ok(size) => {
                let text = from_utf8(&data[0..size]).unwrap();
                println!("Recieved: {}", text);
            }
            Err(_e) => {
                // eprintln!("Error: {}", _e);
            }
        }
    }
}

// There's either something brokey here or something brokey in server/main.rs and I can't really
// tell where lollers. Something is blocking somehwere and that's stopping messages from being sent
// here. Is it possible that the reciever on the server is blocking the thread here? Alternatively
// it just looks like the thread here is being blocked on writing to the stream somehow.
fn network_send_thread(stream: TcpMutex) {
    loop {
        let input = get_input();
        if input.eq("exit\n") {
            break;
        } else {
            println!("Sending: {}", input);
            // let _ = stream.lock().unwrap().write(&input.into_bytes());
            match stream.lock() {
                Ok(mut s) => { 
                    s.write(&input.into_bytes());
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}

fn main() {
    let stream = TcpStream::connect("localhost:3333").unwrap();
    stream.set_nonblocking(true);
    stream.set_read_timeout(Some(Duration::from_millis(1000)));
    stream.set_write_timeout(Some(Duration::from_millis(10)));

    let stream = Arc::new(Mutex::new(stream));
    let stream2 = Arc::clone(&stream);
    let network_recv_handle = thread::spawn(move || {
        network_recv_thread(stream2);
    });

    let stream2 = Arc::clone(&stream);
    let network_send_handle = thread::spawn(move || {
        network_send_thread(stream2);
    });

    network_recv_handle.join().expect("PANIC!!!!");
    network_send_handle.join().expect("PANIC!!!!");
}
