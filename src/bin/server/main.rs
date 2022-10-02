use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender, Receiver};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

// fn handle_client(mut stream: TcpStream) {
//     let mut data = [0 as u8; 50]; // using 50 byte buffer
//     while match stream.read(&mut data) {
//         Ok(size) => {
//             // echo everything!
//             println!("{:?}", &data[0..size]);
//             data = data.map(|c| c.to_ascii_uppercase());
//             stream.write(&data[0..size]).unwrap();
//             true
//         },
//         Err(_) => {
//             println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
//             stream.shutdown(Shutdown::Both).unwrap();
//             false
//         }
//     } {}
// }
//
// fn handle_client_streams(streams: Arc<Mutex<Vec<TcpStream>>>, rx: Receiver<Vec<u8>>) {
//     let mut data = [0 as u8; 50]; // using 50 byte buffer
//     while match stream.read(&mut data) {
//         Ok(size) => {
//             // echo everything!
//             println!("{:?}", &data[0..size]);
//             let ss = streams.lock().unwrap();
//             println!("{:?}", ss);
//             for mut s in ss.iter() {
//                 data = data.map(|c| c.to_ascii_uppercase());
//                 s.write(&data[0..size]).unwrap();
//             }
//             true
//         },
//         Err(_) => {
//             println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
//             stream.shutdown(Shutdown::Both).unwrap();
//             false
//         }
//     } {}
// }
//
// fn main2() {
//     let mut streams: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
//     // let mut streams: Vec<TcpStream> = Vec::new();
//     let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
//     // accept connections and process them, spawning a new thread for each one
//     println!("Server listening on port 3333");
//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 let stream = Arc::new(stream);
//                 println!("New connection: {}", stream.peer_addr().unwrap());
//
//                 streams.lock().unwrap().push(&Arc::clone(stream));
//                 let streams2 = Arc::clone(&streams);
//                 thread::spawn(move|| {
//                     // connection succeeded
//                     handle_client_streams(&streams2, &stream)
//                 });
//             }
//             Err(e) => {
//                 println!("Error: {}", e);
//                 /* connection failed */
//             }
//         }
//     }
//     // close the socket server
//     drop(listener);
// }

fn handle_client_streams_mpsc(streams: Arc<Mutex<Vec<TcpStream>>>, rx: Receiver<[u8; 50]>) {
    loop {
        let r = rx.recv();
        println!("Recieved data {:?}", r.unwrap());
        let mut data = [0_u8; 50];
        match r {
            Ok(d) => data = d,
            Err(e) => eprintln!("Error: {}", e),
        }

        let streams_vec = streams.lock().unwrap();
        println!("streams_vec: {:?}", streams_vec);
        for mut stream in streams_vec.iter() {
            println!("Sending '{:?}' to {}", data, stream.peer_addr().unwrap());
            stream.write(&data);
        }
    } 
}

fn main() {
    let mut streams: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");

    let (tx, rx): (Sender<[u8; 50]>, Receiver<[u8; 50]>) = mpsc::channel();
    let streams2 = Arc::clone(&streams);
    let handler_thread = thread::spawn(move || {
        handle_client_streams_mpsc(Arc::clone(&streams2), rx);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("New connection from {:?}", s.peer_addr().unwrap());
                let mut data = [0_u8; 50];
                match s.read(&mut data) {
                    Ok(_size) => {
                        tx.send(data);
                        println!("Adding stream to streams");
                        streams.clone().lock().unwrap().push(s);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                eprint!("Error: {}", e);
            }
        }
    }

    handler_thread.join().expect("PANIC!!!!!");
}
