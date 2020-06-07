#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::thread;
use std::time::Duration;

mod robot;
mod tcp_server;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Lets run a dc motor with h-bridge");

    let mut robot = robot::Robot::new()?;

    let mut times = 0;
    // Lets drive in a circle
    loop {
        thread::sleep_ms(1000);
        robot.left();
        thread::sleep_ms(1000);
        robot.right();
        thread::sleep_ms(1000);
        robot.backward();
        thread::sleep_ms(1000);
        robot.forward();
        thread::sleep_ms(1000);
        robot.stop();
        if (times >= 5) {
            break;
        }
        times += 1;
    }

    Ok(())
}
