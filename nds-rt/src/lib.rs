#![no_std]

use nds_rs::Hw;

/// Entry point called from the C runtime
///
/// # Safety
///
/// Must never be called by user code.
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    extern "Rust" {
        #[link_name = "__rust_user_main"]
        fn main(hw: Hw) -> !;
    }

    let peripherals = Hw::take().unwrap_unchecked();
    unsafe {
        main(peripherals);
    }
}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
