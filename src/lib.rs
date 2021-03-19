#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

pub use nds_entry::entry;

extern crate alloc;

use alloc::string::String;
use core::alloc::{GlobalAlloc, Layout};
use core::fmt::Write;

macro_rules! bit {
    ($shift: literal) => {
        (1 << $shift)
    };
}

pub mod input;
pub mod interrupts;
pub mod video;

pub mod debug {
    extern "C" {
        fn nocashMessage(msg: *const u8);
    }
    pub fn no_cash_message(msg: &str) {
        let mut msg_str: [u8; 256] = unsafe { core::mem::zeroed() };
        for (i, b) in msg.bytes().take(255).enumerate() {
            msg_str[i] = b;
        }
        // SAFETY: msg_str was originally filled with 256 0's, but we copied up to 255 bytes from msg to it. So at least the last byte is still 0
        unsafe {
            nocashMessage(msg_str.as_ptr());
        };
    }
}

type CPtr = *mut core::ffi::c_void;
extern "C" {
    fn malloc(size: usize) -> CPtr;
    fn free(ptr: CPtr) -> ();
    fn calloc(num: usize, size: usize) -> CPtr;
    fn realloc(ptr: CPtr, size: usize) -> CPtr;
}

#[global_allocator]
static ALLOC: MallocAlloc = MallocAlloc;
struct MallocAlloc;
unsafe impl GlobalAlloc for MallocAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = layout.size();
        let ptr = malloc(size);
        if ptr.is_null() {
            crate::handle_alloc_error(layout);
        }
        ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: core::alloc::Layout) {
        free(ptr as CPtr);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let ptr = calloc(1, size);
        if ptr.is_null() {
            crate::handle_alloc_error(layout);
        }
        ptr as *mut u8
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let ptr = realloc(ptr as CPtr, new_size);
        if ptr.is_null() {
            crate::handle_alloc_error(layout);
        }
        ptr as *mut u8
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(_: Layout) -> ! {
    crate::debug::no_cash_message("Allocation failed!");
    loop {
        crate::interrupts::swi_wait_for_v_blank();
    }
}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::debug::no_cash_message("Panic! At the DS\0");
    if let Some(arg) = info.message() {
        let mut out = String::with_capacity(256);
        if let Err(_) = write!(&mut out, "{}", arg) {
            crate::debug::no_cash_message(
                "Additionally, errors ocurred while trying to format the error message",
            );
        } else {
            crate::debug::no_cash_message(out.as_str());
        }
    }
    loop {
        crate::interrupts::swi_wait_for_v_blank();
    }
}
