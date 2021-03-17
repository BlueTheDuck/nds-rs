pub use interrupts::{Flags, WaitFor};
use nds_sys::interrupts::{self, swiIntrWait, swiWaitForVBlank};

/// Waits until the next VBlank
/// (Same as `swi_intr_wait(WaitFor::Next, Flags::VBlank)`)
pub fn swi_wait_for_v_blank() {
    unsafe {
        swiWaitForVBlank();
    }
}

pub fn swi_intr_wait(wait_for: WaitFor, flags: Flags) {
    unsafe {
        swiIntrWait(wait_for as u32, flags as u32);
    }
}
