#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(adt_const_params)]
#![allow(unused_parens)]

#[cfg(feature = "proc_macros")]
pub use nds_proc_macros::{entry, panic_screen};
pub use nds_sys as sys;

macro_rules! bit {
    ($shift: literal) => {
        (1 << $shift)
    };
}

pub mod background;
#[macro_use]
pub mod debug;
pub mod dma;
pub mod input;
pub mod interrupts;
mod memalloc;
mod panic;
pub mod sprite;
pub mod system;
pub mod video;
