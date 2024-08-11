extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

use picolibc::bindings::{calloc, free, malloc, realloc};

type CPtr = *mut core::ffi::c_void;

#[global_allocator]
static ALLOC: MallocAlloc = MallocAlloc;

struct MallocAlloc;
unsafe impl GlobalAlloc for MallocAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = layout.size() as u32;
        let ptr = malloc(size);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: core::alloc::Layout) {
        free(ptr as CPtr);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let ptr = calloc(1, size as u32);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr as *mut u8
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let ptr = realloc(ptr as CPtr, new_size as u32);
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
