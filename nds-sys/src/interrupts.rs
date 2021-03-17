extern "C" {
    pub fn swiWaitForVBlank();
    pub fn swiIntrWait(waitForSet: u32, flags: u32);
}

#[repr(u32)]
pub enum WaitFor {
    /// Don't wait if this interrupt has already fired
    None = 0,
    /// Wait for the next time the interrupt is fired
    Next = 1,
}

/// Flags to mask de last interrupt
#[repr(u32)]
pub enum Flags {
    VBlank = 1,
    HBlank = 2,
}

