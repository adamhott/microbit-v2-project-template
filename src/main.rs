#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;
use microbit::hal::prelude::*;
use microbit::Board;
use microbit::hal::gpio::Level;
use microbit::hal::timer::Timer;


#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Initialize the board
    let board = Board::take().unwrap();

    // Accessing pins from the board and set level to low
    // Pinouts here: https://tech.microbit.org/hardware/edgeconnector/#pins-and-signals
    let mut pin_0 = board.pins.p0_02.into_push_pull_output(Level::Low);
    let mut pin_1 = board.pins.p0_03.into_push_pull_output(Level::Low);
    let mut pin_2 = board.pins.p0_04.into_push_pull_output(Level::Low);

    // Assigning Timer
    let mut timer = Timer::new(board.TIMER0);


    loop {
        rprintln!("Pin 0 Set to High Voltage (3v)!");
        // Set Pin Voltage to High (3v)
        pin_0.set_high().unwrap();
        //One second delay
        timer.delay_ms(1000_u32);
        // Set Pin Voltage to Low (0v)
        pin_0.set_low().unwrap();

        rprintln!("Pin 1 Set to High Voltage (3v)!");
        // Set Pin Voltage to High (3v)
        pin_1.set_high().unwrap();
        //One second delay
        timer.delay_ms(1000_u32);
        // Set Pin Voltage to Low (0v)
        pin_1.set_low().unwrap();

        rprintln!("Pin 2 Set to High Voltage (3v)!");
        // Set Pin Voltage to High (3v)
        pin_2.set_high().unwrap();
        //One second delay
        timer.delay_ms(1000_u32);
        // Set Pin Voltage to Low (0v)
        pin_2.set_low().unwrap();
    }

}