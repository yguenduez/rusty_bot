use std::error::Error;
use std::thread;
use std::time::Duration;

mod robot;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Lets run a dc motor with h-bridge");

    let mut robot = robot::Robot::new()?;
    let robot_controller = robot::RobotController::new(robot);

    let mut times = 0;
    // Lets drive in a circle
    /*
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
    */
    robot_controller.run();
    Ok(())
}
