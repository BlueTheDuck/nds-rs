//! Video related functions and constants.
//! Mostly "borrowed" from libnds's video.h



/// Display control register (main). Used to choose the mode for the Main Engine
pub static mut REG_DISPCNT: *mut u32 = 0x04000000 as *mut u32;
/// Display control register (sub). Used to choose the mode for the Sub Engine
pub static mut REG_DISPCNT_SUB: *mut u32 = 0x04001000 as *mut u32;

pub static VRAM_ENABLE: u8 = 1 << 7;

/// Control register for VRAM A
pub static mut VRAM_A_CR: *mut u8 = 0x04000240 as *mut u8;
/// When VRAM A is in LCD mode, this points to the first pixel (top left most)
pub static mut VRAM_A: *mut u16 = 0x6800000 as *mut u16;
