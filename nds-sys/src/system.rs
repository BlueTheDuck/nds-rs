use super::bitflags::bitflags;

extern "C" {
    pub fn systemShutDown();
    pub fn getBatteryLevel() -> u32;
}

pub mod registers {
    pub static mut POWCNT: *mut u16 = 0x4000304 as *mut u16;
}

bitflags! {
    pub struct PowerFlags: u16 {
        /// When set, the main engine will render on the TOP screen
        const SWAP_LCD = bit!(15);
        const SUB_ENGINE = bit!(9);
        const MAIN_ENGINE = bit!(1);
        const POWER_LCD = bit!(0);
    }
}
