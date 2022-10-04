use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender, Receiver};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::time::Duration;

type TcpMutex = Arc<Mutex<TcpStream>>;
type Buffer = (Vec<u8>, usize);

fn distribute_messages(streams: Arc<Mutex<Vec<TcpMutex>>>, rx: Receiver<Buffer>) {
    loop {
        // thread::sleep(Duration::from_millis(500));

        let r = rx.recv();
        println!("Recieved data {:?}", r.as_ref().unwrap());
        // let mut data = [0_u8; 50];
        // let mut size: usize = 0;
        let (mut data, mut size): Buffer = (Vec::new(), 0);
        match r {
            Ok(d) => (data, size) = d,
            Err(e) => eprintln!("One Error: {}", e),
        }

        // data = data.iter().map(|c| c.to_ascii_uppercase());
        for char in data.iter_mut() {
            char.to_ascii_uppercase();
        }

        let streams_vec = streams.lock().unwrap();
        // println!("Streams: {:?}", streams_vec);
        for stream in streams_vec.iter() {
            println!("Sending '{:?}' to {}", data, stream.lock().unwrap().peer_addr().unwrap());
            let _ = stream.lock().unwrap().write(&data[0..size]);
            // println!("Done sending");
        }
    } 
}

// fn get_messages()

fn client_handler(stream: TcpMutex, tx: Sender<Buffer>) {
    // let mut data = [0_u8; 50];
    // let size = stream.lock().unwrap().read_to_end(&mut data).unwrap();

    let mut size: usize = 0;
    let mut data: Vec<u8> = Vec::new();

    loop {
        // thread::sleep(Duration::from_millis(500));
        let r = stream.lock().unwrap().read_to_end(&mut data);
        match r {
            Ok(s) => { 
                size = s;
                let _ = tx.send((data.clone(), size));
            },
            Err(ref e) => { eprintln!("Two Error: {}", e); },
        }
    }
}

fn main() {
    let streams: Arc<Mutex<Vec<TcpMutex>>> = Arc::new(Mutex::new(Vec::new()));
    let mut client_handlers: Vec<thread::JoinHandle<()>> = Vec::new();
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");


    let (tx, rx): (Sender<Buffer>, Receiver<Buffer>) = mpsc::channel();
    let streams2 = Arc::clone(&streams);

    let distribute_messages_thread = thread::spawn(move || {
        distribute_messages(Arc::clone(&streams2), rx);
    });

    // Done this bit now
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                // s.set_nonblocking(true);
                s.set_read_timeout(Some(Duration::from_millis(10000)));
                s.set_write_timeout(Some(Duration::from_millis(10000)));

                println!("New connection from {:?}", s.peer_addr().unwrap());
                let s = Arc::new(Mutex::new(s));

                let tx2 = tx.clone();
                let s2 = Arc::clone(&s);
                // s2.lock().unwrap().set_read_timeout(Some(Duration::from_millis(1000)));
                let cur_thread = thread::spawn(move || {
                    client_handler(s2, tx2);
                });

                client_handlers.push(cur_thread);

                println!("Adding stream to streams");
                streams.lock().unwrap().push(Arc::clone(&s));

                // s2.lock().unwrap().set_write_timeout(Some(Duration::from_millis(1000)));
                // s2.lock().unwrap().set_nonblocking(true);
            }
            Err(e) => {
                eprint!("Error: {}", e);
            }
        }
    }

    for handler in client_handlers {
        handler.join().expect("See ya later clients.");
    }

    distribute_messages_thread.join().expect("PANIC!!!!!");
}
