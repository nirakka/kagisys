#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;


// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_PWM: u8 = 23;

// Servo configuration. Change these values based on your servo's verified safe
// minimum and maximum values.
//
// Period: 20 ms (50 Hz). Pulse width: min. 1200 μs, neutral 1500 μs, max. 1800 μs.
const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 450;
const PULSE_NEUTRAL_US: u64 = 900;
const PULSE_MAX_US: u64 = 800;


#[get("/")]
fn index() -> &'static str {
     light_on();
    "Hello, world!"
}

#[get("/light-on")]
fn on() -> &'static str {
     light_on();
    "Hello, world!"
}

#[get("/light-off")]
fn off() -> &'static str {
     light_off();
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, on, off]).launch();
}

fn light_on() -> Result<(), Box<dyn Error>> {
    // Retrieve the GPIO pin and configure it as an output.
    let mut pin = Gpio::new()?.get(GPIO_PWM)?.into_output();

    pin.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_NEUTRAL_US),
    )?;

    println!("Light:On");


    thread::sleep(Duration::from_millis(1000));

    Ok(())

    // When the pin variable goes out of scope, software-based PWM is automatically disabled.
    // You can manually disable PWM by calling the clear_pwm() method.
}

fn light_off() -> Result<(), Box<dyn Error>> {
    // Retrieve the GPIO pin and configure it as an output.
    let mut pin = Gpio::new()?.get(GPIO_PWM)?.into_output();
    // Rotate the servo to the opposite side.
    pin.set_pwm(
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MIN_US),
    )?;

    println!("Light:Off");


    thread::sleep(Duration::from_millis(1000));


    Ok(())

    // When the pin variable goes out of scope, software-based PWM is automatically disabled.
    // You can manually disable PWM by calling the clear_pwm() method.
}
