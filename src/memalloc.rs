extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use crate::debug;

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
            handle_alloc_error(layout);
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
            handle_alloc_error(layout);
        }
        ptr as *mut u8
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let ptr = realloc(ptr as CPtr, new_size);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr as *mut u8
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(_: Layout) -> ! {
    #[cfg(feature = "nocash_tty")]
    debug::NOCASH
        .lock()
        .print_with_params_no_alloc("Out of memory\0");
    loop {
        crate::interrupts::swi_wait_for_v_blank();
    }
}
