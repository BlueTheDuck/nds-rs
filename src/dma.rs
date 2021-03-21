//! The Nintendo DS contains a DMA (Direct Memory Access)
//! hardware that allows the console to copy and fill
//! memory sections without using the CPU
//! All fill and copy operations are NOT synchronous, they return immediatly.
//! Use `is_busy` and `wait_for` to check and wait for a channel

use core::ops::BitAnd;

pub use nds_sys::dma::Channel;
use nds_sys::dma::{calc_cr, calc_registers, Flags};

use crate::interrupts::swi_wait_for_v_blank;

/// Checks if the specified channel is busy
pub fn is_busy(ch: Channel) -> bool {
    let cr = calc_cr(ch);
    let flags = unsafe { Flags::from_bits_unchecked(cr.read_volatile()) };
    return (flags & Flags::ENABLED).bits() != 0;
}

/// Hangs until the specified channel becomes available
pub fn wait_for(ch: Channel) {
    // TODO: Improve this. DMA has interrupts
    loop {
        if !is_busy(ch) {
            break;
        }
        swi_wait_for_v_blank();
    }
}

/// Copies `len*4` bytes from `src` into `dst`.
pub unsafe fn copy_words(ch: Channel, src: *const u32, dst: *mut u32, len: usize) {
    let (src_cr, dst_cr, cr, _) = calc_registers(ch);
    src_cr.write_volatile(src as *const usize);
    dst_cr.write_volatile(dst as *mut usize);
    let flags = (Flags::ENABLE | Flags::WORDS).bits() | (len as u32);
    cr.write_volatile(flags);
}

/// Copies `len*2` bytes from `src` into `dst`.
pub unsafe fn copy_half_words(ch: Channel, src: *const u16, dst: *mut u16, len: usize) {
    let (src_cr, dst_cr, cr, _) = calc_registers(ch);
    src_cr.write_volatile(src as *const usize);
    dst_cr.write_volatile(dst as *mut usize);
    let flags = Flags::ENABLE.bits() | (len as u32);
    cr.write_volatile(flags);
}

/// Fills `dst` with `len*4` copies of `value`.
pub unsafe fn fill_words(ch: Channel, value: u32, dst: *mut u32, len: usize) {
    let (src_cr, dst_cr, cr, fill_cr) = calc_registers(ch);
    fill_cr.write_volatile(value as u32);
    src_cr.write_volatile(fill_cr as *const usize);
    dst_cr.write_volatile(dst as *mut usize);
    let flags = (Flags::ENABLE | Flags::WORDS | Flags::FIX_SRC).bits() | (len as u32);
    cr.write_volatile(flags);
}

/// Fills `dst` with `len*2` copies of `value`.
pub unsafe fn fill_half_words(ch: Channel, value: u16, dst: *mut u16, len: usize) {
    let (src_cr, dst_cr, cr, fill_cr) = calc_registers(ch);
    fill_cr.write_volatile(value as u32);
    src_cr.write_volatile(fill_cr as *const usize);
    dst_cr.write_volatile(dst as *mut usize);
    let flags = (Flags::ENABLE | Flags::FIX_SRC).bits() | (len as u32);
    cr.write_volatile(flags);
}
