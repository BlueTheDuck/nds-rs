#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(adt_const_params)]
#![allow(unused_parens, dead_code)]

pub use nds_sys as sys;

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

#[doc(hidden)]
mod private {
    /// This trait is sealed and cannot be implemented outside of this crate.
    /// It is used to prevent users from implementing special traits for marker
    /// traits and types.
    pub trait Sealed {}

    impl<'g, L> Sealed for crate::video::backgrounds::DirectBitmapLayer<'g, L> {}
    impl Sealed for crate::video::Layer0 {}
    impl Sealed for crate::video::Layer1 {}
    impl Sealed for crate::video::Layer2 {}
    impl Sealed for crate::video::Layer3 {}
    impl Sealed for crate::video::graphics::Mode5 {}
    impl Sealed for crate::video::graphics::VramA {}
}
