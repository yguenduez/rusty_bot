# Rusty Bot

I already had a little robot running with [C++ and ROS](https://gitlab.com/ysngndz/raspi_robot_driver) and tried this project with Rust.
Beware of hacky code, stupid logic running around and heavy use of 3rd party!


## Overview

The project consists of:

* a `robot` application, which is running on the raspberry pi. It is just putting pins on `high` or `low` to control some dc motors.
* a `robot_core`, which serves as a library, used by the `robot` application. Here the tcp server's "logic" is defined.
* a `gamepad_client` executable, which takes the first gamepad client accessable from the pc and listens for gamepad inputs. It then sends serialized json (thanks serde library!) onto the openend tcp sever port of the pi: `raspberrypi.local:3333`.

To put is very simple this image below will demonstrate the complexity of the project:

<img src="images/overview.png" alt="Overview" style="width:600px;"/>

## How to start

You will have to connect the pins of the pi to the raspberry - see source which pins :).

Then:

* First start the pi application (on the pi!)
  ```bash
  cd robot
  cargo run
  ```
* Make sure both, PC and PI are within the same subnet
* Then start the application on the PC with the gamepad connected
  ```bash
  cd gamepad_client
  cargo run
  ```

## Result 

If all works out. You should have a moving robot - except the robot is too phat.

![Fat Robot](images/heavy.gif) ![Light Robot](images/lightweight.gif)


## Beware

If you are using the raspi pi zero, you will need to target the armv6 architecture for compiling.



