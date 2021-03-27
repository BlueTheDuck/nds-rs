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

/// Flags to mask the interrupts
#[repr(u32)]
pub enum Flags {
    /// Vertical blank
    VBlank = bit!(0),
    /// Horizontal blank
    HBlank = bit!(1),
    /// Cout match
    VCount = bit!(2),
    Timer0 = bit!(3),
    Timer1 = bit!(4),
    Timer2 = bit!(5),
    Timer3 = bit!(6),
    Network = bit!(7),
    Dma0 = bit!(8),
    Dma1 = bit!(9),
    Dma2 = bit!(10),
    Dma3 = bit!(11),
    Keys = bit!(12),
    Cart = bit!(13),
    IpcSync = bit!(16),
    FifoEmpty = bit!(17),
    FifoNotEmpty = bit!(18),
    Card = bit!(19),
    CardLine = bit!(20),
    GeometryFifo = bit!(21),
    Lid = bit!(22),
    Spi = bit!(23),
    Wifi = bit!(24),
    All = bit!(25) - 1,
}
