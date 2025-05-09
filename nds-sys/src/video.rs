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

pub enum DisplayMode {
    Off = 0b00_00_0000000000000000,
    GraphicsMode0 = 0b00_01_0000000000000000,
    GraphicsMode1 = 0b00_01_0000000000000001,
    GraphicsMode2 = 0b00_01_0000000000000010,
    GraphicsMode3 = 0b00_01_0000000000000011,
    GraphicsMode4 = 0b00_01_0000000000000100,
    GraphicsMode5 = 0b00_01_0000000000000101,
    GraphicsMode6 = 0b00_01_0000000000000110,
    VramA = 0b00_10_0000000000000000,
    VramB = 0b01_10_0000000000000000,
    VramC = 0b10_10_0000000000000000,
    VramD = 0b11_10_0000000000000000,
    Ram = 0b00_11_0000000000000000,
}

bitflags! {
    /// Display control flags
    #[derive(Copy, Clone)]
    pub struct DispCntFlags: u32 {
        /// Use extended palette
        const EXT_PALETTE = bit!(30);
        /// MAIN ONLY: 64kB offset for map data.
        const MAP_BASE_MASK = 0b111 << 27;
        /// MAIN ONLY: 64kB offset for tile data
        const TILE_BASE_MASK = 0b111 << 24;
        const DISPLAY_SRC_MASK = 0b11_11 << 16;
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
        const MODE_MASK = 0b111;

        // all bits have meaning
        const _ = 0xFFFF_FFFFu32;
    }
}
impl DispCntFlags {
    pub const fn with_display_mode(self, mode: DisplayMode) -> Self {
        const MASK: DispCntFlags = DispCntFlags::MODE_MASK.union(DispCntFlags::DISPLAY_SRC_MASK);
        let mode = mode as u32;
        self.difference(MASK)
            .union(DispCntFlags::from_bits_truncate(mode))
    }
    pub const fn with_map_base(self, base: u32) -> Self {
        let base = (base & 0b111) << 27;
        self.difference(DispCntFlags::MAP_BASE_MASK)
            .union(DispCntFlags::from_bits_truncate(base))
    }
    pub const fn with_tile_base(self, base: u32) -> Self {
        let base = (base & 0b111) << 24;
        self.difference(DispCntFlags::TILE_BASE_MASK)
            .union(DispCntFlags::from_bits_truncate(base))
    }
    pub const fn map_base(self) -> u32 {
        (self.intersection(DispCntFlags::MAP_BASE_MASK).bits()) >> 27
    }
    pub const fn tile_base(self) -> u32 {
        (self.intersection(DispCntFlags::TILE_BASE_MASK).bits()) >> 24
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
    Mode0_2d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode0)
        .bits(),
    Mode1_2d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode1)
        .bits(),
    Mode2_2d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode2)
        .bits(),
    Mode3_2d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode3)
        .bits(),
    Mode4_2d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode4)
        .bits(),
    Mode5_2d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode5)
        .bits(),
    Mode6_2d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode6)
        .bits(),

    Mode0_3d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode0)
        .union(DispCntFlags::BG0)
        .union(DispCntFlags::ENABLE_3D)
        .bits(),
    Mode1_3d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode1)
        .union(DispCntFlags::BG0)
        .union(DispCntFlags::ENABLE_3D)
        .bits(),
    Mode2_3d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode2)
        .union(DispCntFlags::BG0)
        .union(DispCntFlags::ENABLE_3D)
        .bits(),
    Mode3_3d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode3)
        .union(DispCntFlags::BG0)
        .union(DispCntFlags::ENABLE_3D)
        .bits(),
    Mode4_3d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode4)
        .union(DispCntFlags::BG0)
        .union(DispCntFlags::ENABLE_3D)
        .bits(),
    Mode5_3d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode5)
        .union(DispCntFlags::BG0)
        .union(DispCntFlags::ENABLE_3D)
        .bits(),
    Mode6_3d = DispCntFlags::empty()
        .with_display_mode(DisplayMode::GraphicsMode6)
        .union(DispCntFlags::BG0)
        .union(DispCntFlags::ENABLE_3D)
        .bits(),

    /// Display directly from RAM
    ModeFifo = DispCntFlags::empty()
        .with_display_mode(DisplayMode::Ram)
        .bits(),
    /// Displays directly from Bank A in LCD mode
    ModeFb0 = DispCntFlags::empty()
        .with_display_mode(DisplayMode::VramA)
        .bits(),
    /// Displays directly from Bank B in LCD mode
    ModeFb1 = DispCntFlags::empty()
        .with_display_mode(DisplayMode::VramB)
        .bits(),
    /// Displays directly from Bank C in LCD mode
    ModeFb2 = DispCntFlags::empty()
        .with_display_mode(DisplayMode::VramC)
        .bits(),
    /// Displays directly from Bank D in LCD mode
    ModeFb3 = DispCntFlags::empty()
        .with_display_mode(DisplayMode::VramD)
        .bits(),
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
