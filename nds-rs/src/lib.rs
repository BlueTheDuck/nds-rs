#![no_std]
#![feature(alloc_error_handler)]
#![feature(adt_const_params)]
#![allow(unused_parens, dead_code)]

// Reexport internal crates
pub use nds_sys as sys;
#[macro_use]
pub extern crate nds_proc_macros;

#[macro_use]
pub mod debug;
pub mod background;
pub mod cache;
pub mod dma;
#[cfg(feature = "embedded-graphics-core")]
pub mod embedded_graphics;
pub mod input;
pub mod interrupts;
pub mod macros;
mod memalloc;
mod peripherals;
pub mod sprite;
pub mod system;
pub mod video;
pub use peripherals::Hw;
pub mod header;
pub mod runtime;

#[doc(hidden)]
mod private {
    /// This trait is sealed and cannot be implemented outside of this crate.
    /// It is used to prevent users from implementing special traits for marker
    /// traits and types.
    pub trait Sealed {}

    impl<L2, L3, R> Sealed for crate::background::GraphicsMode<L2, L3, R> {}
    impl Sealed for crate::background::MainGraphicsModeSettings {}
    impl Sealed for crate::background::SubGraphicsModeSettings {}
}
