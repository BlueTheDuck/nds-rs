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
pub unsafe fn panic(info: &PanicInfo) -> ! {
    let game_title = "TODO"; // TODO: Get game title from header
    println!("'{}' panicked", game_title);
    if let Some(location) = info.location() {
        println!(
            "{}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    }
    println!("{}", info.message());
    loop {}
}
