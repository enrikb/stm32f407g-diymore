//! On-board user push buttons

use crate::hal::prelude::*;

use crate::hal::gpio::gpiod::{self, PD, PD15};
use crate::hal::gpio::{Input, Floating};

/// User button
pub type BTN0 = PD15<Input<Floating>>;

/// User button names
pub enum BtnName {
    KN2,
}

// Array of the on-board user LEDs
pub struct Btns {
    btns: [Btn; 1],
}

impl Btns {
    pub fn new(gpiod: gpiod::Parts) -> Self {
        let btn = gpiod.pd15.into_floating_input();

        Btns {
            btns: [btn.into()],
        }
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<Btn> {
        self.btns.iter_mut()
    }
}

impl core::ops::Deref for Btns {
    type Target = [Btn];

    fn deref(&self) -> &[Btn] {
        &self.btns
    }
}

impl core::ops::DerefMut for Btns {
    fn deref_mut(&mut self) -> &mut [Btn] {
        &mut self.btns
    }
}

impl core::ops::Index<usize> for Btns {
    type Output = Btn; 

    fn index(&self, i: usize) -> &Btn {
        &self.btns[i]
    }
}

impl core::ops::Index<BtnName> for Btns {
    type Output = Btn;

    fn index(&self, n: BtnName) -> &Btn {
        &self.btns[n as usize]
    }
}

impl core::ops::IndexMut<usize> for Btns {
    fn index_mut(&mut self, i: usize) -> &mut Btn {
        &mut self.btns[i]
    }
}

impl core::ops::IndexMut<BtnName> for Btns {
    fn index_mut(&mut self, n: BtnName) -> &mut Btn {
        &mut self.btns[n as usize]
    }
}

/// One of the on-board user buttons 
pub struct Btn {
    pin: PD<Input<Floating>>,
}

macro_rules! ctor {
	($($ldx:ident),+) => {
		$(
			impl Into<Btn> for $ldx {
				fn into(self) -> Btn {
					Btn {
						pin: self.downgrade(),
					}
				}
			}
		)+
	}
}

ctor!(BTN0);

impl Btn {
    /// Returns true if button is pressed
    pub fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }
}