#![no_std]
#![feature(adt_const_params)]
#![allow(clippy::unusual_byte_groupings)]

#[macro_use]
extern crate bitflags;

macro_rules! bit {
    ($shift: literal) => {
        (1 << $shift)
    };
}

pub mod background;
pub mod console;
pub mod debug;
pub mod dma;
pub mod input;
pub mod interrupts;
pub mod sprite;
pub mod system;
pub mod video;
