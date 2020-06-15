use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt;
use std::io::{stdin, BufRead, BufReader, Error, Write};
use std::net::{Ipv4Addr, SocketAddr};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Serialize, Deserialize)]
pub struct Direction {
    pub x: f32,
    pub y: f32,
}

impl Direction {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }
}

pub struct TCPServer {
    pub listener: TcpListener,
    handle: Option<thread::JoinHandle<()>>,
}

impl TCPServer {
    pub fn new() -> TCPServer {
        TCPServer {
            listener: TcpListener::bind("raspberrypi.local:3333").unwrap(),
            handle: None,
        }
    }
    pub fn start(&mut self) {
        let stream_r = self.listener.accept();
        match stream_r {
            Ok((stream, _)) => {
                self.handle = Some(thread::spawn(move || loop {
                    let mut de = serde_json::Deserializer::from_reader(&stream);
                    let u = Direction::deserialize(&mut de);
                }));
            }
            Err(e) => println!("Could not connect! Error: {}", e),
        }
    }
}
