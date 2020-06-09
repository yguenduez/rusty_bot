use std::fmt;
use std::io::{stdin, BufRead, BufReader, Error, Write};
use std::net::{Ipv4Addr, SocketAddr};
use std::thread;
use std::net::{TcpListener, TcpStream};

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
struct Direction {
    x: i32,
    y: i32,
}

pub struct TCPServer {
    listener: TcpListener,
    buffer: Vec<Direction>
}

impl TCPServer {
    pub fn new() -> TCPServer {
        TCPServer {
            listener: TcpListener::bind("0.0.0.0:8888").unwrap(),
            buffer: vec![]
        }
    }
    pub fn start_listening(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Err(e) => eprintln!("Failed to connect: {}", e),
                Ok(stream) => {
                    thread::spawn(move || {
                        read_from_stream(stream).unwrap();
                    });
                }
            }
        }
    }
}

fn read_from_stream(stream: TcpStream) -> Result<Direction, Error> {
    let mut de = serde_json::Deserializer::from_reader(stream);
    let u = Direction::deserialize(&mut de)?;

    Ok(u)
}
