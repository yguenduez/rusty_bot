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

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Lets run a dc motor with h-bridge");

    let mut motor = Motor::new(23, 24, 25)?;

    let mut times = 0;
    loop {
        motor.stop();
        thread::sleep_ms(500);
        motor.forward();
        thread::sleep_ms(3000);
        motor.backward();
        thread::sleep_ms(3000);

        if (times >= 10) {
            break;
        }
        times += 1;
    }

    Ok(())
}
