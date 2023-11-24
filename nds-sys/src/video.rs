//! Video related functions and constants.
//! Mostly "borrowed" from libnds's video.h

use crate::background::Layer;

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
        /// Display from main RAM.
        /// Requires configuring DMA
        const DISPLAY_RAM = 3 << 16;
        /// Display the bitmap stored in VRAM
        const DISPLAY_VRAM = 2 << 16;
        /// Generate images using the regular 2D/3D main and sub engines
        const DISPLAY_GRAPHICS = 1 << 16;
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

/// Constants to be used for [`set_video_mode`] and [`set_video_mode_sub`].
/// The DS has 2 rendering engines, Main and Sub, that can be put in different modes (6 modes and 5 modes respectively).
/// `Mode6_2d` and the ones suffixed "3D" are only valid for Main
/// (The 3D engine renders on Background 0).
/// Modes FB0-FB3 ("LCD" mode) map the Banks A-D respectively to pixels on screen.
#[repr(u32)]
pub enum VideoMode {
    Mode0_2d = DispCntFlags::DISPLAY_GRAPHICS.bits() | DispCntFlags::MODE0.bits(),
    Mode1_2d = DispCntFlags::DISPLAY_GRAPHICS.bits() | DispCntFlags::MODE1.bits(),
    Mode2_2d = DispCntFlags::DISPLAY_GRAPHICS.bits() | DispCntFlags::MODE2.bits(),
    Mode3_2d = DispCntFlags::DISPLAY_GRAPHICS.bits() | DispCntFlags::MODE3.bits(),
    Mode4_2d = DispCntFlags::DISPLAY_GRAPHICS.bits() | DispCntFlags::MODE4.bits(),
    Mode5_2d = DispCntFlags::DISPLAY_GRAPHICS.bits() | DispCntFlags::MODE5.bits(),
    Mode6_2d = DispCntFlags::DISPLAY_GRAPHICS.bits() | DispCntFlags::MODE6.bits(),

    Mode0_3d = Self::Mode0_2d as u32 | DispCntFlags::BG0.bits() | DispCntFlags::ENABLE_3D.bits(),
    Mode1_3d = Self::Mode1_2d as u32 | DispCntFlags::BG0.bits() | DispCntFlags::ENABLE_3D.bits(),
    Mode2_3d = Self::Mode2_2d as u32 | DispCntFlags::BG0.bits() | DispCntFlags::ENABLE_3D.bits(),
    Mode3_3d = Self::Mode3_2d as u32 | DispCntFlags::BG0.bits() | DispCntFlags::ENABLE_3D.bits(),
    Mode4_3d = Self::Mode4_2d as u32 | DispCntFlags::BG0.bits() | DispCntFlags::ENABLE_3D.bits(),
    Mode5_3d = Self::Mode5_2d as u32 | DispCntFlags::BG0.bits() | DispCntFlags::ENABLE_3D.bits(),
    Mode6_3d = Self::Mode6_2d as u32 | DispCntFlags::BG0.bits() | DispCntFlags::ENABLE_3D.bits(),

    /// Display directly from RAM
    ModeFifo = DispCntFlags::DISPLAY_RAM.bits(),
    /// Displays directly from Bank A in LCD mode
    ModeFb0 = DispCntFlags::DISPLAY_VRAM.bits() | DispCntFlags::VRAM_A.bits(),
    /// Displays directly from Bank B in LCD mode
    ModeFb1 = DispCntFlags::DISPLAY_VRAM.bits() | DispCntFlags::VRAM_B.bits(),
    /// Displays directly from Bank C in LCD mode
    ModeFb2 = DispCntFlags::DISPLAY_VRAM.bits() | DispCntFlags::VRAM_C.bits(),
    /// Displays directly from Bank D in LCD mode
    ModeFb3 = DispCntFlags::DISPLAY_VRAM.bits() | DispCntFlags::VRAM_D.bits(),
}

/// Sets the video mode for the main engine.
///
/// # Safety
/// While any mode is valid for the main engine,
/// changing it on the fly can cause graphical glitches
/// and may crash the system
#[inline]
pub unsafe fn set_video_mode(flags: VideoMode) {
    REG_DISPCNT.write_volatile(flags as u32);
}

/// Sets the video mode for the sub engine.
/// Mode 6 and 3D are invalid for the sub engine.
///
/// # Safety
///  - The sub engine can't use any mode that uses 3D
///  - Only modes 0-5 can be used for the sub engine
///  - changing it on the fly can cause graphical glitches and may crash the system
#[inline]
pub unsafe fn set_video_mode_sub(flags: VideoMode) {
    REG_DISPCNT_SUB.write_volatile(flags as u32);
}

#[inline]
pub fn video_3d_enabled() -> bool {
    let control = DispCntFlags::from_bits_retain(unsafe { REG_DISPCNT.read_volatile() });
    control.contains(DispCntFlags::ENABLE_3D)
}

pub fn enable_main_background(layer: Layer, enable: bool) {
    let flag = match layer {
        Layer::Layer0 => DispCntFlags::BG0,
        Layer::Layer1 => DispCntFlags::BG1,
        Layer::Layer2 => DispCntFlags::BG2,
        Layer::Layer3 => DispCntFlags::BG3,
    };
    let mut current_flags = DispCntFlags::from_bits_retain(unsafe { REG_DISPCNT.read_volatile() });
    current_flags.set(flag, enable);
    unsafe { REG_DISPCNT.write_volatile(current_flags.bits()) };
}
