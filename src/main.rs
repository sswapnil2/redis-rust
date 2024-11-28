#![allow(unused_imports)]

mod protocol;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::sync::{Arc, Mutex};
use std::thread;
use protocol::executor::Executor;
use protocol::store::Store;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let mut store = Arc::new(Mutex::new(Store::new()));

    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {

        match stream {
            Ok(_stream) => {
                let app_state = Arc::clone(&store);
                thread::spawn(move | | {
                    handle_client(app_state, _stream);
                });
            }
            Err(e) => {
                println!("Logging: {}", e);
            }
        }
    }
}

fn handle_client(app_state:  Arc<Mutex<Store>>, mut stream: TcpStream) {
    let mut buf = [0u8; 1024];

    loop {

        match stream.read(&mut  buf) {
            Ok(0) => {
                return;
            },
            Ok(bytes) => {

                let input = &buf[..bytes];

                // Lock the store to execute the command
                let response = {
                    let mut store = app_state.lock().unwrap();
                    Executor::execute(&mut store, input)
                };

                // Send the response back to the client
                if let Some(out) = response {
                    if let Err(e) = stream.write_all(out.as_bytes()) {
                        println!("Error writing to client: {}", e);
                        return;
                    }
                }

            },
            Err(e) => {
                println!("Error reading from client: {}", e);
                return;
            }
        }
    }

}
