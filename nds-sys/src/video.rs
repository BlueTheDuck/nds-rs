//! Video related functions and constants.
//! Mostly "borrowed" from libnds's video.h

mod vram_control;
pub use vram_control::*;

use bitfield_struct::bitfield;

pub const BG_GFX: *mut u16 = 0x06000000 as _;
pub const BG_GFX_SUB: *mut u16 = 0x06200000 as _;

/// Display control register (main). Used to choose the mode for the Main Engine
pub const REG_DISPCNT: *mut DisplayControl = 0x04000000 as _;
/// Display control register (sub). Used to choose the mode for the Sub Engine
pub const REG_DISPCNT_SUB: *mut DisplayControl = 0x04001000 as _;

pub const SCREEN_HEIGHT: u32 = 192;
pub const SCREEN_WIDTH: u32 = 256;

pub const VRAM_ENABLE: u8 = 1 << 7;

/// Control register for VRAM A
pub const VRAM_A_CR: *mut VramAType = 0x04000240 as _;
/// Control register for VRAM B
pub const VRAM_B_CR: *mut VramBType = 0x04000241 as _;
/// Control register for VRAM C
pub const VRAM_C_CR: *mut VramCType = 0x04000242 as _;
/// Control register for VRAM D
pub const VRAM_D_CR: *mut VramDType = 0x04000243 as _;
/// Control register for VRAM E
pub const VRAM_E_CR: *mut VramEType = 0x04000244 as _;
/// Control register for VRAM F
pub const VRAM_F_CR: *mut VramFType = 0x04000245 as _;
/// Control register for VRAM G
pub const VRAM_G_CR: *mut VramGType = 0x04000246 as _;
/// Control register for VRAM H
pub const VRAM_H_CR: *mut VramHType = 0x04000248 as _;
/// Control register for VRAM I
pub const VRAM_I_CR: *mut VramIType = 0x04000249 as _;

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

#[bitfield(u32)]
pub struct DisplayControl {
    #[bits(3)]
    pub bg_mode: u32,
    pub display_3d: bool,
    pub tile_obj_mapping: bool,
    pub bitmap_obj_2d_dimension: bool,
    pub bitmap_obj_mapping: bool,
    pub forced_blank: bool,
    pub display_bg0: bool,
    pub display_bg1: bool,
    pub display_bg2: bool,
    pub display_bg3: bool,
    pub display_obj: bool,
    pub window_0_display: bool,
    pub window_1_display: bool,
    pub obj_window_display: bool,
    #[bits(2)]
    pub display_mode: u32,
    #[bits(2)]
    pub display_vram: u32,
    #[bits(2)]
    pub tile_obj_1d_boundary: u32,
    pub bitmap_obj_1d_boundary: bool,
    pub obj_processing_during_h_blank: bool,
    #[bits(3)]
    pub char_base: u32,
    #[bits(3)]
    pub screen_base: u32,
    pub bg_ext_pal: bool,
    pub obj_ext_pal: bool,
}

impl DisplayControl {
    pub const fn with_mode(self, mode: DisplayMode) -> Self {
        let other = Self::from_bits(mode as u32);
        self.with_display_mode(other.display_mode())
            .with_display_vram(other.display_vram())
            .with_bg_mode(other.bg_mode())
    }

    #[inline]
    pub const fn with_3d_enabled(self) -> Self {
        self.with_display_3d(true)
    }

    #[inline]
    pub const fn with_display_bg0_enabled(self) -> Self {
        self.with_display_bg0(true)
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
pub mod modes {
    use super::*;

    pub const MODE0_2D: DisplayControl =
        DisplayControl::new().with_mode(DisplayMode::GraphicsMode0);
    pub const MODE1_2D: DisplayControl =
        DisplayControl::new().with_mode(DisplayMode::GraphicsMode1);
    pub const MODE2_2D: DisplayControl =
        DisplayControl::new().with_mode(DisplayMode::GraphicsMode2);
    pub const MODE3_2D: DisplayControl =
        DisplayControl::new().with_mode(DisplayMode::GraphicsMode3);
    pub const MODE4_2D: DisplayControl =
        DisplayControl::new().with_mode(DisplayMode::GraphicsMode4);
    pub const MODE5_2D: DisplayControl =
        DisplayControl::new().with_mode(DisplayMode::GraphicsMode5);
    pub const MODE6_2D: DisplayControl =
        DisplayControl::new().with_mode(DisplayMode::GraphicsMode6);

    pub const MODE0_3D: DisplayControl = DisplayControl::new()
        .with_mode(DisplayMode::GraphicsMode0)
        .with_3d_enabled()
        .with_display_bg0_enabled();
    pub const MODE1_3D: DisplayControl = DisplayControl::new()
        .with_mode(DisplayMode::GraphicsMode1)
        .with_3d_enabled()
        .with_display_bg0_enabled();
    pub const MODE2_3D: DisplayControl = DisplayControl::new()
        .with_mode(DisplayMode::GraphicsMode2)
        .with_3d_enabled()
        .with_display_bg0_enabled();
    pub const MODE3_3D: DisplayControl = DisplayControl::new()
        .with_mode(DisplayMode::GraphicsMode3)
        .with_3d_enabled()
        .with_display_bg0_enabled();
    pub const MODE4_3D: DisplayControl = DisplayControl::new()
        .with_mode(DisplayMode::GraphicsMode4)
        .with_3d_enabled()
        .with_display_bg0_enabled();
    pub const MODE5_3D: DisplayControl = DisplayControl::new()
        .with_mode(DisplayMode::GraphicsMode5)
        .with_3d_enabled()
        .with_display_bg0_enabled();
    pub const MODE6_3D: DisplayControl = DisplayControl::new()
        .with_mode(DisplayMode::GraphicsMode6)
        .with_3d_enabled()
        .with_display_bg0_enabled();

    /// Display directly from RAM
    pub const MODE_FIFO: DisplayControl = DisplayControl::new().with_mode(DisplayMode::Ram);
    /// Displays directly from Bank A in LCD mode
    pub const MODE_FB0: DisplayControl = DisplayControl::new().with_mode(DisplayMode::VramA);
    /// Displays directly from Bank B in LCD mode
    pub const MODE_FB1: DisplayControl = DisplayControl::new().with_mode(DisplayMode::VramB);
    /// Displays directly from Bank C in LCD mode
    pub const MODE_FB2: DisplayControl = DisplayControl::new().with_mode(DisplayMode::VramC);
    /// Displays directly from Bank D in LCD mode
    pub const MODE_FB3: DisplayControl = DisplayControl::new().with_mode(DisplayMode::VramD);
}

/// Sets the video mode for the main engine.
///
/// NOTE:
/// While any mode is valid for the main engine,
/// changing it on the fly can cause graphical glitches
/// and may crash the system
#[inline]
pub fn set_video_mode(flags: DisplayControl) {
    unsafe {
        REG_DISPCNT.write_volatile(flags);
    }
}

/// Sets the video mode for the sub engine.
/// Mode 6 and 3D are invalid for the sub engine.
///
/// # Safety
///  - The sub engine can't use any mode that uses 3D
///  - Only modes 0-5 can be used for the sub engine
///  - changing it on the fly can cause graphical glitches and may crash the system
#[inline]
pub unsafe fn set_video_mode_sub(flags: DisplayControl) {
    unsafe { REG_DISPCNT_SUB.write_volatile(flags) };
}
