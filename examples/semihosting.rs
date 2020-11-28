//! Sends "Hello, world!" through semihosting interfaces.
//! ---

#![no_main]
#![no_std]

extern crate panic_semihosting;
extern crate stm32f407g_diymore as board;
extern crate embedded_hal as hal;
use cortex_m_semihosting::{debug, hprintln};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {

	hprintln!("Hello, world!").ok();

	debug::exit(debug::EXIT_SUCCESS);
    loop { continue; }
}
