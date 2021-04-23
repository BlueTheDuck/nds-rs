#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(const_generics)]

pub use nds_entry::entry;
pub use nds_sys as sys;

extern crate alloc;

use alloc::string::String;
use core::alloc::{GlobalAlloc, Layout};
use core::fmt::Write;

pub mod background;
pub mod dma;
pub mod input;
pub mod interrupts;
pub mod system;
pub mod video;

pub mod debug;

type CPtr = *mut core::ffi::c_void;
extern "C" {
    fn malloc(size: usize) -> CPtr;
    fn free(ptr: CPtr);
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
    crate::debug::no_cash_message("Panic! At the DS\n\0");
    if let Some(arg) = info.message() {
        let mut out = String::with_capacity(256);
        if write!(&mut out, "{}", arg).is_ok() {
            crate::debug::no_cash_message(out.as_str());
            unsafe {
                crate::debug::printf(out.as_ptr());
            }
        } else {
            static extra_err_msg: &str = "Additionally, errors ocurred while trying to format the error message";
            crate::debug::no_cash_message(
                extra_err_msg
            );
            unsafe {
                crate::debug::printf(extra_err_msg.as_ptr());
            }
        }
    }
    loop {
        crate::interrupts::swi_wait_for_v_blank();
    }
}
