use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt;
use std::io::{stdin, BufRead, BufReader, Error, Write};
use std::net::{Ipv4Addr, SocketAddr};
use std::net::{TcpListener, TcpStream};
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
}

impl TCPServer {
    pub fn new() -> TCPServer {
        TCPServer {
            listener: TcpListener::bind("raspberrypi.local:3333").unwrap(),
        }
    }
}
