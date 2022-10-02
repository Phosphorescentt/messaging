use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender, Receiver};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

type TcpMutex = Arc<Mutex<TcpStream>>;
type Buffer = ([u8; 50], usize);

fn handle_client_streams_mpsc(streams: Arc<Mutex<Vec<TcpMutex>>>, rx: Receiver<Buffer>) {
    loop {
        let r = rx.recv();
        println!("Recieved data {:?}", r.unwrap());
        let mut data = [0_u8; 50];
        let mut size: usize = 0;
        match r {
            Ok(d) => (data, size) = d,
            Err(e) => eprintln!("Error: {}", e),
        }

        let streams_vec = streams.lock().unwrap();
        // println!("streams_vec: {:?}", streams_vec);
        for stream in streams_vec.iter() {
            data = data.map(|c| c.to_ascii_uppercase());
            println!("Sending '{:?}' to {}", data, stream.lock().unwrap().peer_addr().unwrap());
            let _ = stream.lock().unwrap().write(&data[0..size]);
            println!("Done sending");
        }
    } 
}

fn handle_new_message(stream: TcpMutex, tx: Sender<Buffer>) {
    let mut data = [0_u8; 50];
    let size = stream.lock().unwrap().peek(&mut data).unwrap();

    let _ = tx.send((data, size));
}

fn main() {
    let streams: Arc<Mutex<Vec<TcpMutex>>> = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");

    let (tx, rx): (Sender<Buffer>, Receiver<Buffer>) = mpsc::channel();
    let streams2 = Arc::clone(&streams);

    let handler_thread = thread::spawn(move || {
        handle_client_streams_mpsc(Arc::clone(&streams2), rx);
    });

    // This means that messages are only processed every time a client connects to the server
    // something needs to just be perma-listening to the stream to see if there's anything coming
    // in and then if there is it needs to do something with that. Currently it seems liek there 
    // are no issues with locking or blocking or anything like that, just the fact that we're only
    // handling stuff when a new connection comes in.
    //
    // TL; DR the architecture of this part is completely borked.
    // This for loop should set up new connections and then hand them off to another thread to do
    // some networking magic rather than spawning a new thread for each new connection.
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                println!("New connection from {:?}", s.peer_addr().unwrap());

                let s = Arc::new(Mutex::new(s));
                println!("Adding stream to streams");
                streams.lock().unwrap().push(Arc::clone(&s));

                let s2 = Arc::clone(&s);
                let tx2 = tx.clone();
                thread::spawn(move || {
                    handle_new_message(s2, tx2);
                });
            }
            Err(e) => {
                eprint!("Error: {}", e);
            }
        }
    }

    handler_thread.join().expect("PANIC!!!!!");
}
