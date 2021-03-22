#![no_std]

#[macro_use]
extern crate bitflags;

macro_rules! bit {
    ($shift: literal) => {
        (1 << $shift)
    };
}

pub mod input;
pub mod video;
pub mod interrupts;
pub mod dma;
pub mod system;