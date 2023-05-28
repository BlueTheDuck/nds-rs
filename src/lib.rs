#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(adt_const_params)]
#![allow(unused_parens, dead_code)]

#[cfg(feature = "proc_macros")]
pub use nds_proc_macros::{entry, panic_screen};
pub use nds_sys as sys;

pub mod background;
#[macro_use]
pub mod debug;
pub mod cache;
pub mod dma;
#[cfg(feature = "embedded-graphics-core")]
pub mod embedded_graphics;
pub mod input;
pub mod interrupts;
pub mod macros;
mod memalloc;
mod panic;
pub mod sprite;
pub mod system;
pub mod video;
