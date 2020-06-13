use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{stdin, BufRead, BufReader, Error, Write};
use std::net::{Ipv4Addr, SocketAddr};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Serialize, Deserialize)]
pub struct Direction {
    x: f32,
    y: f32,
}

impl Direction {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }
}

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

pub struct TCPServer {
    listener: TcpListener,
    buffer: RingBuffer<Direction>,
}

impl TCPServer {
    pub fn new() -> TCPServer {
        TCPServer {
            listener: TcpListener::bind("raspberrypi.local:3333").unwrap(),
            buffer: RingBuffer::new(50),
        }
    }
    pub fn start_listening(&self) {
        match self.listener.accept() {
            Ok((stream, addr)) => {
                println!("new client: {:?}", addr);
                read_from_stream(stream);
            }
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}

fn read_from_stream(stream: TcpStream) -> Result<Direction, Error> {
    let mut de = serde_json::Deserializer::from_reader(stream);
    let u = Direction::deserialize(&mut de)?;

    println!("Direction: {}, {}", u.x, u.y);

    Ok(u)
}
