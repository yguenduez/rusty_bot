use rppal::gpio::{Gpio, OutputPin};
use rppal::system::DeviceInfo;

use robot_core::tcp_server;
use std::sync::Arc;
use std::thread;
use tcp_server::Direction;

use serde::{Deserialize, Serialize};

struct Motor {
    enable_pin: OutputPin,
    pin_1: OutputPin,
    pin_2: OutputPin,
}

impl Motor {
    fn new(enable: u8, pin_1: u8, pin_2: u8) -> Result<Motor, rppal::gpio::Error> {
        let enable_pin = Gpio::new()?.get(enable)?.into_output();
        let pin_1 = Gpio::new()?.get(pin_1)?.into_output();
        let pin_2 = Gpio::new()?.get(pin_2)?.into_output();

        Ok(Motor {
            enable_pin: enable_pin,
            pin_1: pin_1,
            pin_2: pin_2,
        })
    }
    fn forward(&mut self) {
        self.enable_pin.set_high();
        self.pin_2.set_low();
        self.pin_1.set_high();
    }
    fn backward(&mut self) {
        self.enable_pin.set_high();
        self.pin_1.set_low();
        self.pin_2.set_high();
    }
    fn stop(&mut self) {
        self.enable_pin.set_low();
        self.pin_1.set_low();
        self.pin_2.set_low();
    }
}

pub struct Robot {
    motor_1: Motor,
    motor_2: Motor,
}

impl Robot {
    pub fn new() -> Result<Robot, rppal::gpio::Error> {
        let motor_1 = Motor::new(23, 24, 25)?;
        let motor_2 = Motor::new(17, 26, 22)?;

        Ok(Robot {
            motor_1: motor_1,
            motor_2: motor_2,
        })
    }

    pub fn forward(&mut self) {
        self.motor_1.forward();
        self.motor_2.forward();
    }

    pub fn backward(&mut self) {
        self.motor_1.backward();
        self.motor_2.backward();
    }

    pub fn stop(&mut self) {
        self.motor_1.stop();
        self.motor_2.stop();
    }

    pub fn left(&mut self) {
        self.motor_1.forward();
        self.motor_2.backward();
    }

    pub fn right(&mut self) {
        self.motor_1.backward();
        self.motor_2.forward();
    }
}

#[derive(PartialEq, Eq)]
pub enum CurrentDirection {
    Left,
    Right,
    Forward,
    Backward,
    Stop,
}

pub fn get_direction(dir: tcp_server::Direction) -> CurrentDirection {
    if (dir.x > 0.8) {
        CurrentDirection::Right
    } else if (dir.x < -0.8) {
        CurrentDirection::Left
    } else if (dir.y > 0.8) {
        CurrentDirection::Forward
    } else if (dir.y < -0.8) {
        CurrentDirection::Backward
    } else {
        CurrentDirection::Stop
    }
}

pub struct RobotController {
    robot: Robot,
    current_dir: CurrentDirection,
}

impl RobotController {
    pub fn new(robot: Robot) -> RobotController {
        RobotController {
            robot: robot,
            current_dir: CurrentDirection::Stop,
        }
    }

    pub fn run(&mut self) {
        // listen to incoming data
        let mut tcp_server = tcp_server::TCPServer::new();
        match tcp_server.listener.accept() {
            Ok((stream, addr)) => {
                println!("new client: {:?}", addr);
                loop {
                    let mut de = serde_json::Deserializer::from_reader(&stream);
                    let u = Direction::deserialize(&mut de);
                    match u {
                        Err(e) => println!("Could not deserialize message. Error: {}", e),
                        Ok(dir) => {
                            self.set_robot_dir_from_direction(dir);
                        }
                    }
                    thread::sleep_ms(50);
                }
            }
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }

    pub fn set_robot_dir_from_direction(&mut self, dir: Direction) {
        let planned_dir = get_direction(dir);

        if self.current_dir == planned_dir {
            return;
        }

        match planned_dir {
            CurrentDirection::Forward => {
                self.robot.forward();
                self.current_dir = CurrentDirection::Forward;
            }
            CurrentDirection::Backward => {
                self.robot.backward();
                self.current_dir = CurrentDirection::Backward;
            }
            CurrentDirection::Left => {
                self.robot.left();
                self.current_dir = CurrentDirection::Left;
            }
            CurrentDirection::Right => {
                self.robot.right();
                self.current_dir = CurrentDirection::Right;
            }
            CurrentDirection::Stop => {
                self.robot.stop();
                self.current_dir = CurrentDirection::Stop;
            }
        }
    }
}
