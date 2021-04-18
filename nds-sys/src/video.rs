//! Video related functions and constants.
//! Mostly "borrowed" from libnds's video.h

pub const BG_GFX: *mut u16 = 0x06000000 as *mut u16;

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

bitflags! {
    pub struct Flags: u32 {
        /// Use extended palette
        const EXT_PALETTE = bit!(30);
        /// These bits multiplied by 0x100000 gives the base offset for the background data
        const BG_SCREEN_BASE_MASK = 3 << 27;
        // These are mutually exclusive
        /// When [DISPLAY_VRAM] is set, display from this bank
        const VRAM_D = 3 << 18;
        /// When [DISPLAY_VRAM] is set, display from this bank
        const VRAM_C = 2 << 18;
        /// When [DISPLAY_VRAM] is set, display from this bank
        const VRAM_B = 1 << 18;
        /// When [DISPLAY_VRAM] is set, display from this bank
        const VRAM_A = 0 << 18;
        //
        // These are mutually exclusive
        const DISPLAY_RAM = 3 << 16;
        const DISPLAY_VRAM = 2 << 16;
        const DISPLAY_GRAPHICS = 1 << 16;
        const DISPLAY_ON = bit!(16);
        const DISPLAY_OFF = 0;
        //
        // Set to show objects
        const OBJ_DISPLAY = bit!(12);
        /// Set to show this background
        const BG3 = bit!(11);
        /// Set to show this background
        const BG2 = bit!(10);
        /// Set to show this background
        const BG1 = bit!(9);
        /// Set to show this background
        const BG0 = bit!(8);
        /// Set to use BG0 as the output of the 3D engine
        const ENABLE_3D = bit!(3);
        // These are mutually exclusive
        /// Sets engine in mode 6 (invalid for Sub)
        const MODE6 = 6;
        /// Sets engine in mode 5
        const MODE5 = 5;
        /// Sets engine in mode 4
        const MODE4 = 4;
        /// Sets engine in mode 3
        const MODE3 = 3;
        /// Sets engine in mode 2
        const MODE2 = 2;
        /// Sets engine in mode 1
        const MODE1 = 1;
        /// Sets engine in mode 0
        const MODE0 = 0;
    }
}
