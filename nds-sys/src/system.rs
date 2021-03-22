use super::bitflags::bitflags;

extern "C" {
    pub fn systemShutDown();
    pub fn getBatteryLevel() -> u32;
}

pub static mut REG_POWERCNT: *mut u16 = 0x4000304 as *mut u16;

bitflags! {
    pub struct PowerFlags: u16 {
        /* const ARM9_DIRECT = bit!(16);
        const POWER_LCD = (PowerFlags::ARM9_DIRECT).bits() | bit!(0);
        const SWAP_LCD = (PowerFlags::ARM9_DIRECT).bits() | bit!(15); */
        const SWAP_LCD = bit!(15);
    }
}


