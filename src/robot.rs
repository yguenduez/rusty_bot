use rppal::gpio::{Gpio, OutputPin};
use rppal::system::DeviceInfo;

use crate::tcp_server::{TCPServer};

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

struct RobotController{
    tcp_server: TCPServer,
    robot: Robot
}

impl RobotController {
    pub fn new(robot: Robot)->RobotController{
        RobotController{
            robot: robot,
            tcp_server: TCPServer::new()
        }
    }

    pub fn run(&self){
        self.tcp_server.start_listening();
        loop {
            
        }
    }
}
