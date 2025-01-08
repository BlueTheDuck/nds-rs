use nds_sys::system::PowerFlags;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Screen {
    Top,
    Bottom,
}

pub struct System {}
impl System {
    pub(crate) const unsafe fn new() -> Self {
        Self {}
    }

    pub fn shutdown(&mut self) {
        unsafe {
            nds_sys::system::systemShutDown();
        };
    }

    pub fn battery(&self) -> u32 {
        unsafe { nds_sys::system::getBatteryLevel() }
    }

    /// Controls weather the main engine should output to the top or bottom screen
    pub fn main_engine_on(&mut self, wanted: Screen) {
        let powercnt = unsafe { nds_sys::system::registers::POWCNT.read_volatile() };
        let mut powercnt = PowerFlags::from_bits_retain(powercnt);

        let actual = if powercnt.contains(PowerFlags::SWAP_LCD) {
            Screen::Top
        } else {
            Screen::Bottom
        };
        if actual != wanted {
            powercnt.set(PowerFlags::SWAP_LCD, wanted == Screen::Top);
            unsafe {
                nds_sys::system::registers::POWCNT.write_volatile(powercnt.bits());
            }
        }
    }

    /// Enables or disables the main and sub engines
    ///
    /// Passing `None` will not change the state of the engine
    ///
    pub fn enable_engines(&mut self, main: Option<bool>, sub: Option<bool>) {
        let powerctl = unsafe { nds_sys::system::registers::POWCNT.read_volatile() };
        let mut powerctl = PowerFlags::from_bits_retain(powerctl);
        if let Some(main) = main {
            powerctl.set(PowerFlags::MAIN_ENGINE, main);
        }
        if let Some(sub) = sub {
            powerctl.set(PowerFlags::SUB_ENGINE, sub);
        }
        unsafe {
            nds_sys::system::registers::POWCNT.write_volatile(powerctl.bits());
        }
    }

    /// Turns on or off the LCD backlight.
    /// Noop when the LCD is already in the desired state.
    ///
    /// # Safety
    ///
    /// When running in real hardware, turning on or off the LCD may damage the LCD
    /// circuitry depending on when the setting is changed.
    ///
    /// Call this function only once at the start of the program.
    ///
    pub unsafe fn enable_lcd(&mut self, enable: bool) {
        let powercnt = nds_sys::system::registers::POWCNT.read_volatile();
        let mut powercnt = PowerFlags::from_bits_retain(powercnt);
        let should_write = powercnt.contains(PowerFlags::POWER_LCD) != enable;
        powercnt.set(PowerFlags::POWER_LCD, enable);
        if should_write {
            nds_sys::system::registers::POWCNT.write_volatile(powercnt.bits());
        }
    }
}
