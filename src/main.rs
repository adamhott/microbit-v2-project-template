#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;
use microbit::hal::prelude::*;
use microbit::hal::gpio::{p0::Parts as P0Parts, Level};
use microbit::hal::pac::Peripherals;
use microbit::hal::timer::Timer;


#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Taking board peripherals
    let board = Peripherals::take().unwrap();
    // Assigning Pinouts
    let gpio = P0Parts::new(board.P0);
    // Assigning Tmer
    let mut timer = Timer::new(board.TIMER0);

    // Set up Individual
    //Pin 9 
    let mut pin_9= gpio.p0_09.into_push_pull_output(Level::Low);
    //Pin 3 
    let mut pin_3 = gpio.p0_31.into_push_pull_output(Level::Low);
    //Pin 4
    let mut pin_4 = gpio.p0_28.into_push_pull_output(Level::Low);

    loop {
        rprintln!("Pin 9 Set to High Voltage (3v)!");
        // Set Pin Voltage to High (3v)
        pin_9.set_high().unwrap();
        //One second delay
        timer.delay_ms(1000_u32);
        // Set Pin Voltage to Low (0v)
        pin_9.set_low().unwrap();

        rprintln!("Pin 3 Set to High Voltage (3v)!");
        // Set Pin Voltage to High (3v)
        pin_3.set_high().unwrap();
        //One second delay
        timer.delay_ms(1000_u32);
        // Set Pin Voltage to Low (0v)
        pin_3.set_low().unwrap();

        rprintln!("Pin 4 Set to High Voltage (3v)!");
        // Set Pin Voltage to High (3v)
        pin_4.set_high().unwrap();
        //One second delay
        timer.delay_ms(1000_u32);
        // Set Pin Voltage to Low (0v)
        pin_4.set_low().unwrap();
    }

}