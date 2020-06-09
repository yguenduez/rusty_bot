use std::fmt;
use std::io::{stdin, BufRead, BufReader, Error, Write};
use std::net::{Ipv4Addr, SocketAddr};
use std::net::{TcpListener, TcpStream};
use std::thread;

use serde::Deserialize;

struct RingBuffer<T> {
    internal_buffer: Vec<T>,
    size: u16,
}

impl<T> RingBuffer<T> {
    pub fn new(size: u16) -> Self {
        RingBuffer {
            internal_buffer: Vec::new(),
            size: size,
        }
    }
    pub fn insert(&mut self, value: T) {
        self.internal_buffer.push(value);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Direction {
    x: i32,
    y: i32,
}

pub struct TCPServer {
    listener: TcpListener,
    buffer: RingBuffer<Direction>,
}

impl TCPServer {
    pub fn new() -> TCPServer {
        TCPServer {
            listener: TcpListener::bind("0.0.0.0:8888").unwrap(),
            buffer: RingBuffer::new(50),
        }
    }
    pub fn start_listening(&self) {
        match self.listener.accept() {
            Ok((_hi, addr)) => println!("new client: {:?}", addr),
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}

fn read_from_stream(stream: TcpStream) -> Result<Direction, Error> {
    let mut de = serde_json::Deserializer::from_reader(stream);
    let u = Direction::deserialize(&mut de)?;

    Ok(u)
}
