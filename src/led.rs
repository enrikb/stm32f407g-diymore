//! On-board user LEDs

use crate::hal::prelude::*;

use crate::hal::gpio::gpioe::{self, PE, PE0};
use crate::hal::gpio::{Output, PushPull};

/// User LED
pub type LD0 = PE0<Output<PushPull>>;

/// User LED colors
pub enum LedColor {
    Red,
}

// Array of the on-board user LEDs
pub struct Leds {
    leds: [Led; 1],
}

impl Leds {
    pub fn new(gpioe: gpioe::Parts) -> Self {
        let led = gpioe.pe0.into_push_pull_output();

        Leds {
            leds: [led.into()],
        }
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<Led> {
        self.leds.iter_mut()
    }
}

impl core::ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.leds
    }
}

impl core::ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}

impl core::ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.leds[i]
    }
}

impl core::ops::Index<LedColor> for Leds {
    type Output = Led;

    fn index(&self, c: LedColor) -> &Led {
        &self.leds[c as usize]
    }
}

impl core::ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.leds[i]
    }
}

impl core::ops::IndexMut<LedColor> for Leds {
    fn index_mut(&mut self, c: LedColor) -> &mut Led {
        &mut self.leds[c as usize]
    }
}

/// One of the on-board user LEDs
pub struct Led {
    pin: PE<Output<PushPull>>,
}

macro_rules! ctor {
	($($ldx:ident),+) => {
		$(
			impl Into<Led> for $ldx {
				fn into(self) -> Led {
					Led {
						pin: self.downgrade(),
					}
				}
			}
		)+
	}
}

ctor!(LD0);

impl Led {
    /// Turns the LED off
    pub fn off(&mut self) {
        self.pin.set_high().ok();
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pin.set_low().ok();
    }

    /// Toggles the LED
    pub fn toggle(&mut self) {
        if let Ok(true) = self.pin.is_low() {
            self.pin.set_high().ok();
        } else {
            self.pin.set_low().ok();
        }
    }
}
