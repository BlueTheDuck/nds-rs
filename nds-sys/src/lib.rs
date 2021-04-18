#![no_std]

#[macro_use]
extern crate bitflags;

macro_rules! bit {
    ($shift: literal) => {
        (1 << $shift)
    };
}

pub mod background;
pub mod dma;
pub mod input;
pub mod interrupts;
pub mod system;
pub mod video;
pub mod bindings;
