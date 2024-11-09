#![allow(unused_imports)]

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let mut arr = [0u8; 512];
        match stream {
            Ok(mut _stream) => {
                _stream.read(&mut arr).unwrap();
                let input = String::from_utf8(arr.to_vec()).unwrap();

                let size = input.split("\n")
                    .filter(|&s| { s == "PING" })
                    .count();

                for _ in 0..size {
                    _stream.write(b"+PONG\r\n").unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
