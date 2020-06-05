use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::system::DeviceInfo;

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

struct Robot {
    motor_1: Motor,
    motor_2: Motor,
}

impl Robot {
    fn new() -> Result<Robot, rppal::gpio::Error> {
        let motor_1 = Motor::new(23, 24, 25)?;
        let motor_2 = Motor::new(17, 27, 23)?;

        Ok(Robot {
            motor_1: motor_1,
            motor_2: motor_2,
        })
    }

    fn forward(&mut self) {
        self.motor_1.forward();
        self.motor_2.forward();
    }

    fn backward(&mut self) {
        self.motor_1.backward();
        self.motor_2.backward();
    }

    fn stop(&mut self) {
        self.motor_1.stop();
        self.motor_2.stop();
    }

    fn left(&mut self) {
        self.motor_1.forward();
        self.motor_2.backward();
    }

    fn right(&mut self) {
        self.motor_1.backward();
        self.motor_2.forward();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Lets run a dc motor with h-bridge");

    let mut robot = Robot::new()?;

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
        if (times >= 10) {
            break;
        }
        times += 1;
    }

    Ok(())
}
