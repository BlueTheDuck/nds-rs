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
mod peripherals;
pub mod sprite;
pub mod system;
pub mod video;
pub use peripherals::Hw;
pub mod display;

#[doc(hidden)]
mod private {
    /// This trait is sealed and cannot be implemented outside of this crate.
    /// It is used to prevent users from implementing special traits for marker
    /// traits and types.
    pub trait Sealed {}

    impl<'g, L> Sealed for crate::display::backgrounds::DirectBitmapLayer<'g, L> {}
}
