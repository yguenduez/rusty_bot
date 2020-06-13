use gilrs::{Axis, Event, Gilrs};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::thread;

use robot_core::tcp_server;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut active_gamepad = None;

    match TcpStream::connect("raspberrypi.local:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            loop {
                // Examine new events
                while let Some(Event { id, event, time }) = gilrs.next_event() {
                    active_gamepad = Some(id);
                }
                // You can also use cached gamepad state
                if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
                    let axis_x_val = gamepad.value(Axis::LeftStickX);
                    let axis_y_val = gamepad.value(Axis::LeftStickY);
                    let msg_dir = tcp_server::Direction::new(axis_x_val as f32, axis_y_val as f32);
                    stream
                        .write_all(serde_json::to_string(&msg_dir).unwrap().as_bytes())
                        .expect("Failed to write to server");
                    thread::sleep_ms(50);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to raspberry pi: {}", e);
        }
    }
    println!("Terminated.");
}
