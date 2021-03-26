//! Video related functions and constants.
//! Mostly "borrowed" from libnds's video.h

/// Display control register (main). Used to choose the mode for the Main Engine
pub static mut REG_DISPCNT: *mut u32 = 0x04000000 as *mut u32;
/// Display control register (sub). Used to choose the mode for the Sub Engine
pub static mut REG_DISPCNT_SUB: *mut u32 = 0x04001000 as *mut u32;

pub static VRAM_ENABLE: u8 = 1 << 7;

/// Control register for VRAM A
pub static mut VRAM_A_CR: *mut u8 = 0x04000240 as *mut u8;
/// Control register for VRAM B
pub static mut VRAM_B_CR: *mut u8 = 0x04000241 as *mut u8;
/// Control register for VRAM C
pub static mut VRAM_C_CR: *mut u8 = 0x04000242 as *mut u8;
/// Control register for VRAM D
pub static mut VRAM_D_CR: *mut u8 = 0x04000243 as *mut u8;
/// Control register for VRAM E
pub static mut VRAM_E_CR: *mut u8 = 0x04000244 as *mut u8;
/// Control register for VRAM F
pub static mut VRAM_F_CR: *mut u8 = 0x04000245 as *mut u8;
/// Control register for VRAM G
pub static mut VRAM_G_CR: *mut u8 = 0x04000246 as *mut u8;
/// Control register for VRAM H
pub static mut VRAM_H_CR: *mut u8 = 0x04000248 as *mut u8;
/// Control register for VRAM I
pub static mut VRAM_I_CR: *mut u8 = 0x04000249 as *mut u8;

/// When VRAM A is in LCD mode, this points to the first pixel (top left most)
pub static mut VRAM_A: *mut u16 = 0x6800000 as *mut u16;
/// When VRAM B is in LCD mode, this points to the first pixel (top left most)
pub static mut VRAM_B: *mut u16 = 0x6820000 as *mut u16;
/// When VRAM C is in LCD mode, this points to the first pixel (top left most)
pub static mut VRAM_C: *mut u16 = 0x6840000 as *mut u16;
/// When VRAM D is in LCD mode, this points to the first pixel (top left most)
pub static mut VRAM_D: *mut u16 = 0x6860000 as *mut u16;

/// Background palette (Main)
pub static mut BG_PALETTE: *mut u16 = 0x05000000 as *mut u16;
/// Background palette (Sub)
pub static mut BG_PALETTE_SUB: *mut u16 = 0x05000400 as *mut u16;
