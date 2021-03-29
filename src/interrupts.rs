pub use interrupts::Flags;
use nds_sys::interrupts::{self, swiIntrWait, swiWaitForVBlank, irqEnable, irqDisable};

pub mod registers {
    pub use nds_sys::interrupts::{REG_IE, REG_IME};
}

/// Waits until the next VBlank.
/// (Same as [`swi_intr_wait(Flags::VBLANK, true)`](swi_intr_wait).
pub fn swi_wait_for_v_blank() {
    unsafe {
        swiWaitForVBlank();
    }
}

/// Waits until one of the interrupts specified in `flags` is fired.
/// If that interrupt happened, but hasn't been processed yet, then
/// `swi_intr_wait` returns immediatly only if `wait_for_next` is `false`.
pub fn swi_intr_wait(flags: Flags, wait_for_next: bool) {
    unsafe {
        swiIntrWait(if wait_for_next { 1 } else { 0 }, flags.bits());
    }
}

/// Enable the interrupts specified in `irq`.
/// OR different flags to enable many interrupts at once.
/// This function is unsafe since changing the state of interrupts
/// can (will!) break other code (for example: [`wait_for`](crate::dma::wait_for))
pub unsafe fn irq_enable(irq: Flags) {
    irqEnable(irq.bits());
}

/// Disable the interrupts specified in `irq`.
/// OR different flags to disable many interrupts at once.
/// This function is unsafe since changing the state of interrupts
/// can (will!) break other code (for example: [`wait_for`](crate::dma::wait_for))
pub unsafe fn irq_disable(irq: Flags) {
    irqDisable(irq.bits());
}
