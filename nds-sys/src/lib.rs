#![no_std]

macro_rules! bit {
    ($shift: literal) => {
        (1 << $shift)
    };
}

pub mod input;
pub mod video;
pub mod interrupts;