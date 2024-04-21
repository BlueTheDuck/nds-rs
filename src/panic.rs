use crate::cache;
use crate::debug;

use crate::println;

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        cache::dc_flush_all();
    }
    println!("Panic! At the DS");
    #[cfg(feature = "nocash_tty")]
    {
        unsafe {
            // SAFETY: No other code will run after this function,
            // so there is no problem in freeing the lock
            debug::NOCASH.force_unlock();
        }
        {
            let mut nocash = debug::NOCASH.lock();
            nocash.print_with_params_no_alloc("r0: %r0%\0");
            nocash.print_with_params_no_alloc("sp: %sp%\0");
            nocash.print_with_params_no_alloc("lr: %lr%\0");
            nocash.print_with_params_no_alloc("pc: %pc%\0");
        }
    }
    if let Some(arg) = info.message() {
        println!("Error msg: {}", arg);
    }
    panic_screen();
}

fn panic_screen() -> ! {
    extern "C" {
        fn __nds_panic_screen() -> !;
    }
    unsafe {
        __nds_panic_screen();
    }
}

#[cfg(feature = "default_panic_screen")]
#[no_mangle]
pub extern "C" fn __nds_panic_screen() -> ! {
    use nds_sys::bindings;

    unsafe {
        bindings::consoleDemoInit();
        bindings::consoleClear();
        bindings::printf(b"An error has ocurred\0".as_ptr() as *const _);
    }
    loop {
        crate::interrupts::swi_wait_for_v_blank();
    }
}
