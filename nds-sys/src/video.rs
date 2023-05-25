//! Video related functions and constants.
//! Mostly "borrowed" from libnds's video.h

pub const BG_GFX: *mut u16 = 0x06000000 as _;
pub const BG_GFX_SUB: *mut u16 = 0x06200000 as _;

/// Display control register (main). Used to choose the mode for the Main Engine
pub const REG_DISPCNT: *mut u32 = 0x04000000 as _;
/// Display control register (sub). Used to choose the mode for the Sub Engine
pub const REG_DISPCNT_SUB: *mut u32 = 0x04001000 as _;

pub const SCREEN_HEIGHT: u32 = 192;
pub const SCREEN_WIDTH: u32 = 256;

pub const VRAM_ENABLE: u8 = 1 << 7;

/// Control register for VRAM A
pub const VRAM_A_CR: *mut u8 = 0x04000240 as _;
/// Control register for VRAM B
pub const VRAM_B_CR: *mut u8 = 0x04000241 as _;
/// Control register for VRAM C
pub const VRAM_C_CR: *mut u8 = 0x04000242 as _;
/// Control register for VRAM D
pub const VRAM_D_CR: *mut u8 = 0x04000243 as _;
/// Control register for VRAM E
pub const VRAM_E_CR: *mut u8 = 0x04000244 as _;
/// Control register for VRAM F
pub const VRAM_F_CR: *mut u8 = 0x04000245 as _;
/// Control register for VRAM G
pub const VRAM_G_CR: *mut u8 = 0x04000246 as _;
/// Control register for VRAM H
pub const VRAM_H_CR: *mut u8 = 0x04000248 as _;
/// Control register for VRAM I
pub const VRAM_I_CR: *mut u8 = 0x04000249 as _;

/// When VRAM A is in LCD mode, this points to the first pixel (top left most)
pub const VRAM_A: *mut u16 = 0x6800000 as _;
/// When VRAM B is in LCD mode, this points to the first pixel (top left most)
pub const VRAM_B: *mut u16 = 0x6820000 as _;
/// When VRAM C is in LCD mode, this points to the first pixel (top left most)
pub const VRAM_C: *mut u16 = 0x6840000 as _;
/// When VRAM D is in LCD mode, this points to the first pixel (top left most)
pub const VRAM_D: *mut u16 = 0x6860000 as _;

/// Background palette (Main)
pub const BG_PALETTE: *mut u16 = 0x05000000 as _;
/// Background palette (Sub)
pub const BG_PALETTE_SUB: *mut u16 = 0x05000400 as _;

bitflags! {
    /// Display control flags
    pub struct DispCntFlags: u32 {
        /// Use extended palette
        const EXT_PALETTE = bit!(30);
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
        /// Set to show objects
        const OBJECTS = bit!(12);
        /// Set to show background 3
        const BG3 = bit!(11);
        /// Set to show background 2
        const BG2 = bit!(10);
        /// Set to show background 1
        const BG1 = bit!(9);
        /// Set to show background 0
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

pub const MAP_BASE_OFFSET: u32 = 27;
pub const TILES_BASE_OFFSET: u32 = 24;
pub const MAP_BASE_MASK: u32 = 3 << 27;
pub const TILES_BASE_MASK: u32 = 3 << 24;

pub unsafe fn set_video_mode(flags: DispCntFlags) {
    REG_DISPCNT.write_volatile(flags.bits());
}

pub unsafe fn set_video_mode_sub(flags: DispCntFlags) {
    REG_DISPCNT_SUB.write_volatile(flags.bits());
}

pub fn video_3d_enabled() -> bool {
    let control = unsafe { DispCntFlags::from_bits_unchecked(REG_DISPCNT.read_volatile()) };
    control.contains(DispCntFlags::ENABLE_3D)
}
