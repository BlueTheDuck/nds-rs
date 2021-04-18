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
//!  1. There is a delay of 2 cycles after issuing a request (Writing [`Flags::ENABLED`]) and the DMA actually starting.
//! **Don't touch the channel during that period**, it _will_ lock up.
//!  2. Toggling OFF the bit [`ENABLED`](Flags::ENABLED) will halt the DMA immediatly. 
//! This is **not recommended** unless the channel was programmed to [autorepeat](Flags::REPEAT). 
//! In any case, **wait at least 4 cycles after starting before halting the channel**.
//!  3. [Wait](wait_for) until the channel has finished before issuing a command, otherwise the current operation will be overwritten
//! (in the best case, see points 1 and 2 for the worst case).
//! ([`copy`](copy) and [`fill`](fill) are the only non-`unsafe` functions here, and automatically call [wait_for])
//!  4. _The hardware itself_ doesn't have access to the CPU cache, therefore, data in the stack (such as local variables) may not be available yet
//! in main memory. Static variables and [`Box`](alloc::boxed::Box)ed values are ok, flushing the cache is also an option. ([copy<T>()](copy) and [fill<T>()](fill)
//! take care of this issue by themselves)

use crate::interrupts::swi_intr_wait;
use core::mem::size_of;
use nds_sys::{
    dma::{calc_cr, calc_registers, Flags},
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
    let flags = unsafe { Flags::from_bits_unchecked(cr.read_volatile()) };
    (flags & Flags::ENABLED).bits() != 0
}

/// Hangs until the specified [`Channel`] becomes available.
/// This function uses interrupts, so extra care should be taken
/// to make sure interrupts [`DMA0`](interrupts::Flags::DMA0), [`DMA1`](interrupts::Flags::DMA1),
/// [`DMA2`](interrupts::Flags::DMA2) and [`DMA3`](interrupts::Flags::DMA3) are enabled ([`irq_enable`](crate::interrupts::irq_enable)).
/// On debug builds, this function panics if the required interrupt is not enabled, but on release this may hang for ever
pub fn wait_for(ch: Channel) {
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

/// Fills `dst` with `len` words of `src`.
/// This function operates in words (32 bits), so the amount of bytes copied will be `len*4`
/// # Safety
/// **It is not recommended to use this function**, it is only provided as a compat with the C version of libnds.
/// This function works by instructing the DMA to copy `len` words from `src` to `dst`, so the caller must make
/// sure that:
///  - only [`Copy`] values are being copied,
///  - [`dst`, `dst + len`) is valid **and** won't overwrite unrelated data,
///  - and the channel is free, since [`copy_words`] will overwrite it's settings
pub unsafe fn copy_words(ch: Channel, src: *const u32, dst: *mut u32, len: usize) {
    let (src_cr, dst_cr, cr, _) = calc_registers(ch);
    src_cr.write_volatile(src as *const usize);
    dst_cr.write_volatile(dst as *mut usize);
    let flags = (Flags::ENABLE | Flags::WORDS).bits() | (len as u32);
    cr.write_volatile(flags);
}

/// Fills `dst` with `len` half words of `src`.
/// This function operates in half words (16 bits), so the amount of bytes copied will be `len*2`
/// # Safety
/// **It is not recommended to use this function**, it is only provided as a compat with the C version of libnds.
/// This function works by instructing the DMA to copy `len` half words from `src` to `dst`, so the caller must make
/// sure that:
///  - only [`Copy`] values are being copied,
///  - [`dst`, `dst + len`) is valid **and** won't overwrite unrelated data,
///  - and the channel is free, since [`copy_half_words`] will overwrite it's settings
pub unsafe fn copy_half_words(ch: Channel, src: *const u16, dst: *mut u16, len: usize) {
    let (src_cr, dst_cr, cr, _) = calc_registers(ch);
    src_cr.write_volatile(src as *const usize);
    dst_cr.write_volatile(dst as *mut usize);
    let flags = Flags::ENABLE.bits() | (len as u32);
    cr.write_volatile(flags);
}

/// Fills `dst` with `len` copies of `value`.
/// This function operates in words (32 bits), so the amount of bytes written will be `len*4`
/// # Safety
/// **It is not recommended to use this function**, it is only provided as a compat with the C version of libnds.
/// This function works by instructing the DMA to copy `len` times `value` words over `dst`, so the caller must make
/// sure that:
///  - only [`Copy`] values are being copied,
///  - [`dst`, `dst + len`) is valid **and** won't overwrite unrelated data,
///  - and the channel is free, since [`fill_words`] will overwrite it's settings
pub unsafe fn fill_words(ch: Channel, value: u32, dst: *mut u32, len: usize) {
    let (src_cr, dst_cr, cr, fill_cr) = calc_registers(ch);
    fill_cr.write_volatile(value as u32);
    src_cr.write_volatile(fill_cr as *const usize);
    dst_cr.write_volatile(dst as *mut usize);
    let flags = (Flags::ENABLE | Flags::WORDS | Flags::FIX_SRC).bits() | (len as u32);
    cr.write_volatile(flags);
}

/// Fills `dst` with `len` copies of `value`.
/// This function operates in half words (16 bits), so the amount of bytes written will be `len*2`
/// # Safety
/// **It is not recommended to use this function**, it is only provided as a compat with the C version of libnds.
/// This function works by instructing the DMA to copy `len` times `value` half words over `dst`, so the caller must make
/// sure that:
///  - only [`Copy`] values are being copied,
///  - [`dst`, `dst + len`) is valid **and** won't overwrite unrelated data,
///  - and the channel is free, since [`fill_half_words`] will overwrite it's settings
pub unsafe fn fill_half_words(ch: Channel, value: u16, dst: *mut u16, len: usize) {
    let (src_cr, dst_cr, cr, fill_cr) = calc_registers(ch);
    fill_cr.write_volatile(value as u32);
    src_cr.write_volatile(fill_cr as *const usize);
    dst_cr.write_volatile(dst as *mut usize);
    let flags = (Flags::ENABLE | Flags::FIX_SRC).bits() | (len as u32);
    cr.write_volatile(flags);
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
    let (src_cr, dst_cr, cr, _) = calc_registers(Channel::Ch3);
    let mut flags: Flags = Flags::ENABLE;
    match size_of::<T>() {
        4 => {
            flags |= Flags::WORDS;
        }
        2 => {
            flags |= Flags::HALFWORDS;
        }
        _ => {
            panic!("Can only run copy<T>() if T is either 2 or 4 bytes");
        }
    }
    let flags: u32 = flags.bits() | core::cmp::min(src.len(), dst.len()) as u32;
    unsafe {
        src_cr.write_volatile(src.as_ptr() as *const usize);
        dst_cr.write_volatile(dst.as_mut_ptr() as *mut usize);
        cr.write_volatile(flags);
    }
}

/// Copies `len` elements from `src`. Starts copying at `from`, and copies to `to`.
/// Hangs if the channel is busy.
/// Panics if `size_of::<T>()` is neither 2 nor 4; but doesn't do any bounds check
pub unsafe fn copy_within_unchecked<T>(src: &mut [T], from: usize, to: usize, len: usize)
where
    T: Sized + Copy,
{
    wait_for(Channel::Ch3);
    let (src_cr, dst_cr, cr, _) = calc_registers(Channel::Ch3);
    let mut flags: Flags = Flags::ENABLE;
    match size_of::<T>() {
        4 => {
            flags |= Flags::WORDS;
        }
        2 => {
            flags |= Flags::HALFWORDS;
        }
        _ => {
            panic!("Can only run copy<T>() if size_of::<T>() is either 2 or 4 bytes");
        }
    }
    let flags: u32 = flags.bits() | len as u32;
    let src = src.as_mut_ptr() as *mut T;
    src_cr.write_volatile(src.add(from) as *const usize);
    dst_cr.write_volatile(src.add(to) as *mut usize);
    cr.write_volatile(flags);
}

// /// Copies `len` elements from `src`. Starts copying at `from`, and copies to `to`.
// /// Hangs if the channel is busy.
// /// Panics if `size_of::<T>()` is neither 2 nor 4.
// pub fn copy_within<T>(src: &mut [T], from: usize, to: usize, len: usize) -> Result<(), ()>
// where
//     T: Sized + Copy,
// {
//     // Out-of-bounds read || Out-of-bounds write
//     if src.len() < from + len || src.len() < to + len {
//         return Err(());
//     }
//     //debug_assert!(to <= from || to > from+len,"The segments to be copied overlap");
//     unsafe { copy_within_unchecked::<T>(src, from, to, len) }
//     return Ok(());
// }

/// Fills `dst` by copying `value` using DMA channel 3.
/// Copies `src` into `dst` using DMA channel 3.
/// Calls [`wait_for`] on [`Channel::Ch3`](Channel 3), so if interrupt [`DMA3`](interrupts::Flags::DMA3) is disabled a panic may occur.
/// Panics if `size_of::<T>()` is neither 2 nor 4.
pub fn fill<T>(value: T, dst: &mut [T])
where
    T: Sized + Copy,
{
    wait_for(Channel::Ch3);
    let (src_cr, dst_cr, cr, fill_cr) = calc_registers(Channel::Ch3);
    let mut flags: Flags = Flags::ENABLE | Flags::FIX_SRC | Flags::INT_REQ;
    let value_conv: u32;
    match size_of::<T>() {
        4 => {
            flags |= Flags::WORDS;
            value_conv = unsafe { core::mem::transmute_copy(&value) };
        }
        2 => {
            flags |= Flags::HALFWORDS;
            value_conv = unsafe { core::mem::transmute_copy::<_, u16>(&value) } as u32;
        }
        _ => {
            panic!("Can only run fill<T>() if T is either 2 or 4 bytes");
        }
    }
    let flags: u32 = flags.bits() | dst.len() as u32;
    unsafe {
        fill_cr.write_volatile(value_conv);
        src_cr.write_volatile(fill_cr as *const usize);
        dst_cr.write_volatile(dst.as_mut_ptr() as *mut usize);
        cr.write_volatile(flags);
    }
}
