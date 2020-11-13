#![no_main]
#![no_std]

use panic_halt as _;

use stm32f407g_diymore as board;

use crate::board::{
    hal::prelude::*,
    hal::stm32,
    led::{LedColor, Leds},
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(p) = stm32::Peripherals::take() {
        let gpioe = p.GPIOE.split();

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpioe);

        // Endlessly blink the LED, delaying by executing the state write many times
        // in a row
        loop {
            for _ in 0..1_000_000 {
                leds[LedColor::Red].on();
            }

            for _ in 0..1_000_000 {
                leds[LedColor::Red].off();
            }
        }
    }

    loop {
        continue;
    }
}
