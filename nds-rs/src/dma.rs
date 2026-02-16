//! API to control the DMA
//!
//! The Nintendo DS contains a DMA (Direct Memory Access)
//! hardware that allows the console to copy and fill
//! memory sections without using the CPU.
//! The DMA can only respond to ONE channel at a time, and it will prioratize lower channels first.
//! In case an operation is being processed, and a new one with a higher priority
//! is received, the one with the lowest priority will be put on hold and the other will be fulfilled.
//! **4 cycles** must be waited after issuing a request before cancelling/overwriting it;
//! (See the first 4 lines of [`wait_for`]); failing to do so may lead to a lock up (See point 1 below)
//!
//!
//! All functions here are NOT synchronous (except for [`wait_for`]), they return immediatly.
//! Use [`is_busy`] and [`wait_for`] to check and wait for a channel to be available
//! # Warning
//! The DMA can violate any safety guaranty in Rust, since it can be used to overwrite any memory segment
//! using any value. It can be challenging to properly **and safely** use this hardware, even with this API.
//! When developing, one should keep in mind the following things:
//!  1. There is a delay of 2 cycles after issuing a request (Writing [`Flags::ENABLED`]) and the DMA actually starting. **Don't touch the channel during that period**, it _will_ lock up.
//!  2. Toggling OFF the bit [`ENABLED`](Flags::ENABLED) will halt the DMA immediatly. This is **not recommended** unless the channel was programmed to [autorepeat](Flags::REPEAT). In any case, **wait at least 4 cycles after starting before halting the channel**.
//!  3. [Wait](wait_for) until the channel has finished before issuing a command, otherwise the current operation will be overwritten (in the best case, see points 1 and 2 for the worst case). ([`copy`](copy) and [`fill`](fill) are the only non-`unsafe` functions here, and automatically call [wait_for])
//!  4. _The hardware itself_ doesn't have access to the CPU cache, therefore, data in the stack (such as local variables) may not be available yetin main memory. Static variables and [`Box`](alloc::boxed::Box)ed values are ok, flushing the cache is also an option. ([copy<T>()](copy) and [fill<T>()](fill) take care of this issue by themselves)

use crate::interrupts::swi_intr_wait;
use core::arch::asm;
use core::mem::size_of;
use nds_sys::{
    dma::{self, DmaControl, calc_cr},
    interrupts,
};

pub use nds_sys::dma::Channel;

/// Checks if the specified [`Channel`] is busy
pub fn is_busy(ch: Channel) -> bool {
    unsafe {
        asm!("nop");
        asm!("nop");
        asm!("nop");
        asm!("nop");
    }
    let cr = calc_cr(ch);
    let flags = DmaControl::from_bits(unsafe { cr.read_volatile() });
    flags.busy()
}

/// Hangs until the specified [`Channel`] becomes available.
/// This function uses interrupts, so extra care should be taken
/// to make sure interrupts [`DMA0`](interrupts::Flags::DMA0), [`DMA1`](interrupts::Flags::DMA1),
/// [`DMA2`](interrupts::Flags::DMA2) and [`DMA3`](interrupts::Flags::DMA3) are enabled ([`irq_enable`](crate::interrupts::irq_enable)).
/// On debug builds, this function panics if the required interrupt is not enabled, but on release this may hang for ever
pub fn wait_for(ch: Channel) {
    unsafe {
        asm!("nop");
        asm!("nop");
        asm!("nop");
        asm!("nop");
    }
    if !is_busy(ch) {
        return;
    }
    let irq = match ch {
        Channel::Ch0 => interrupts::Flags::DMA0,
        Channel::Ch1 => interrupts::Flags::DMA1,
        Channel::Ch2 => interrupts::Flags::DMA2,
        Channel::Ch3 => interrupts::Flags::DMA3,
    };
    if cfg!(debug_assertions) {
        let ok;
        let ime = unsafe { interrupts::REG_IME.read_volatile() };
        if ime == 1 {
            let ie = unsafe { interrupts::Flags::from_bits(interrupts::REG_IE.read_volatile()) };
            if let Some(ie) = ie {
                ok = (ie & irq).bits() != 0;
            } else {
                ok = false;
            }
        } else {
            ok = false;
        }
        if !ok {
            panic!("wait_for requires DMA interrupts to be enabled");
        }
    }
    swi_intr_wait(irq, false);
}

/// Copies `src` into `dst` using DMA channel 3.
/// Calls [`wait_for`] on [`Channel::Ch3`](Channel 3), so if interrupt [`DMA3`](interrupts::Flags::DMA3) is disabled a panic may occur.
/// Panics if `size_of::<T>()` is neither 2 nor 4.
/// In case `src.len() != dst.len()` then only `min(src.len(), dst.len())` elements will be copied.
pub fn copy<T>(src: &[T], dst: &mut [T])
where
    T: Sized + Copy,
{
    wait_for(Channel::Ch3);
    let len = src.len().min(dst.len());
    match size_of::<T>() {
        2 => unsafe {
            dma::copy_half_words(
                Channel::Ch3,
                src.as_ptr() as *const u16,
                dst.as_mut_ptr() as *mut u16,
                len,
            );
        },
        4 => unsafe {
            dma::copy_words(
                Channel::Ch3,
                src.as_ptr() as *const u32,
                dst.as_mut_ptr() as *mut u32,
                len,
            );
        },
        _ => {
            panic!("Can only run copy<T>() if T is either 2 or 4 bytes");
        }
    }
}

/// Fills `dst` by copying `value` using DMA channel 3.
/// Copies `src` into `dst` using DMA channel 3.
/// Calls [`wait_for`] on [`Channel::Ch3`](Channel 3), so if interrupt [`DMA3`](interrupts::Flags::DMA3) is disabled a panic may occur.
/// Panics if `size_of::<T>()` is neither 2 nor 4.
pub fn fill<T>(value: T, dst: &mut [T])
where
    T: Sized + Copy,
{
    wait_for(Channel::Ch3);

    match size_of::<T>() {
        2 => unsafe {
            dma::fill_half_words(
                Channel::Ch3,
                &value as *const T as _,
                dst.as_mut_ptr() as *mut u16,
                dst.len(),
            );
        },
        4 => unsafe {
            dma::fill_words(
                Channel::Ch3,
                &value as *const T as _,
                dst.as_mut_ptr() as *mut u32,
                dst.len(),
            );
        },
        _ => {
            panic!("Can only run fill<T>() if T is either 2 or 4 bytes");
        }
    }
}
