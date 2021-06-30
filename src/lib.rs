#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(const_generics)]
#![allow(unused_parens)]

pub use nds_entry::entry;
pub use nds_sys as sys;

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

pub mod background;
#[macro_use]
pub mod debug;
pub mod dma;
pub mod input;
pub mod interrupts;
pub mod system;
pub mod video;

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
    unsafe {
        debug::NOCASH.print_with_params_no_alloc("Out of memory\0");
    }
    loop {
        crate::interrupts::swi_wait_for_v_blank();
    }
}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("Panic! At the DS");
    unsafe {
        debug::NOCASH.print_with_params_no_alloc("r0: %r0%\0");
        debug::NOCASH.print_with_params_no_alloc("sp: %sp%\0");
        debug::NOCASH.print_with_params_no_alloc("lr: %lr%\0");
        debug::NOCASH.print_with_params_no_alloc("pc: %pc%\0");
    }
    if let Some(arg) = info.message() {
        println!("{}", arg);
    }
    loop {
        crate::interrupts::swi_wait_for_v_blank();
    }
}
