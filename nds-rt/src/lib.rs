#![no_std]

use nds_rs::Hw;

#[no_mangle]
pub extern "C" fn main() -> ! {
    extern "Rust" {
        #[link_name = "rust_main"]
        fn main(hw: Hw) -> !;
    }

    let peripherals = Hw::take().unwrap();
    unsafe {
        main(peripherals);
    }
}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
