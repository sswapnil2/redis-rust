#![allow(unused_imports)]

mod protocol;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;
use protocol::resp::Resp;
use protocol::executor::Executor;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {

        match stream {
            Ok(_stream) => {
                thread::spawn(| | {
                    handle_client(_stream);
                });
            }
            Err(e) => {
                println!("Logging: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Option<()>{
    let mut buf = [0u8; 50];

    loop {
        let bytes = stream.read(&mut  buf).unwrap();
        if bytes == 0 {
            return None;
        }

        let mut out = Executor::execute(&buf)?;
        println!("Output: {}", out);
        stream.write_all(&mut out.as_bytes()).ok()?;

        // if let Some(resp) = Resp::parse(&buf) {
        //     let mut out = resp.raw.as_slice();
        //     stream.write_all(&mut out).unwrap();
        // }

    }

}
