extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

type CPtr = *mut core::ffi::c_void;

#[global_allocator]
static ALLOC: MallocAlloc = MallocAlloc;

struct MallocAlloc;
unsafe impl GlobalAlloc for MallocAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ptr = libc::malloc(layout.size());
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: core::alloc::Layout) {
        libc::free(ptr as CPtr);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = libc::calloc(1, layout.size());
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr as *mut u8
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let ptr = libc::realloc(ptr as CPtr, new_size);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr as *mut u8
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(_: Layout) -> ! {
    println!("Out of memory!\0");
    loop {
        crate::interrupts::swi_wait_for_v_blank();
    }
}
