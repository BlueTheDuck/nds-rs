/// Shuts down the console
pub fn shutdown() {
    unsafe {
        nds_sys::system::systemShutDown();
    }
}

/// Returns the battery level
pub fn battery() -> u32 {
    unsafe { nds_sys::system::getBatteryLevel() }
}

pub enum Screen {
    Top,
    Bottom,
}

/// Controls where the Main engine should render
pub fn main_engine_on(screen: Screen) {
    let mut powercnt = unsafe { nds_sys::system::REG_POWERCNT.read_volatile() };
    match screen {
        Screen::Top => {
            powercnt |= nds_sys::system::PowerFlags::SWAP_LCD.bits();
        }
        Screen::Bottom => {
            powercnt &= !nds_sys::system::PowerFlags::SWAP_LCD.bits();
        }
    }
    unsafe {
        nds_sys::system::REG_POWERCNT.write_volatile(powercnt);
    }
}
