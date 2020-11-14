#![no_main]
#![no_std]

use panic_halt as _;

use stm32f407g_diymore as board;

use crate::board::{
    hal::prelude::*,
    hal::stm32,
    led::{LedColor, Leds},
    pushbtn::{BtnName, Btns},
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(p) = stm32::Peripherals::take() {
        // Initialize on-board LED
        let gpioe = p.GPIOE.split();
        let mut leds = Leds::new(gpioe);

        // Initialize push button
        let gpiod = p.GPIOD.split();
        let btns = Btns::new(gpiod);

        leds[LedColor::Red].off();

        // Press button to toggle LED
        let mut last_pressed = false;
        loop {
            let pressed = btns[BtnName::KN2].is_pressed();
            if pressed && !last_pressed {
                leds[LedColor::Red].toggle();
            }
            last_pressed = pressed;
        }
    }

    loop {
        continue;
    }
}
