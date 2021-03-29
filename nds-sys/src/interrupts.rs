extern "C" {
    pub fn swiWaitForVBlank();
    pub fn swiIntrWait(waitForSet: u32, flags: u32);
    pub fn irqEnable(irq: u32);
    pub fn irqDisable(irq: u32);
}

pub static mut REG_IE: *mut u32 = 0x04000210 as *mut _;
pub static mut REG_IME: *mut u32 = 0x04000208 as *mut _;

bitflags! {
    pub struct Flags: u32 {
        const VBLANK = bit!(0);
        const HBLANK = bit!(1);
        const VCOUNT = bit!(2);
        const TIMER0 = bit!(3);
        const TIMER1 = bit!(4);
        const TIMER2 = bit!(5);
        const TIMER3 = bit!(6);
        const NETWORK = bit!(7);
        const DMA0 = bit!(8);
        const DMA1 = bit!(9);
        const DMA2 = bit!(10);
        const DMA3 = bit!(11);
        const KEYS = bit!(12);
        const CART = bit!(13);
        const IPCSYNC = bit!(16);
        const FIFOEMPTY = bit!(17);
        const FIFONOTEMPTY = bit!(18);
        const CARD = bit!(19);
        const CARDLINE = bit!(20);
        const GEOMETRYFIFO = bit!(21);
        const LID = bit!(22);
        const SPI = bit!(23);
        const WIFI = bit!(24);
        const ALL = bit!(25) - 1;
    }
}
