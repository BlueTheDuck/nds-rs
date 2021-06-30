use nds_sys::video::Flags;

use super::Color;



/// Constants to be used for [`set_mode`] and [`set_mode_sub`].
/// The DS has 2 rendering engines, Main and Sub, that can be put in different modes (6 modes and 5 modes respectively).
/// Mode6_2d and the ones suffixed "3D" are only valid for Main
/// (The 3D engine renders on Background 0).
/// Modes FB0-FB3 ("LCD" mode) map the Banks A-D respectively to pixels on screen.
#[repr(u32)]
pub enum Mode {
    Mode0_2d = Flags::DISPLAY_ON.bits() | Flags::MODE0.bits(),
    Mode1_2d = Flags::DISPLAY_ON.bits() | Flags::MODE1.bits(),
    Mode2_2d = Flags::DISPLAY_ON.bits() | Flags::MODE2.bits(),
    Mode3_2d = Flags::DISPLAY_ON.bits() | Flags::MODE3.bits(),
    Mode4_2d = Flags::DISPLAY_ON.bits() | Flags::MODE4.bits(),
    Mode5_2d = Flags::DISPLAY_ON.bits() | Flags::MODE5.bits(),
    Mode6_2d = Flags::DISPLAY_ON.bits() | Flags::MODE6.bits(),

    Mode0_3d = Self::Mode0_2d as u32 | Flags::BG0.bits() | Flags::ENABLE_3D.bits(),
    Mode1_3d = Self::Mode1_2d as u32 | Flags::BG0.bits() | Flags::ENABLE_3D.bits(),
    Mode2_3d = Self::Mode2_2d as u32 | Flags::BG0.bits() | Flags::ENABLE_3D.bits(),
    Mode3_3d = Self::Mode3_2d as u32 | Flags::BG0.bits() | Flags::ENABLE_3D.bits(),
    Mode4_3d = Self::Mode4_2d as u32 | Flags::BG0.bits() | Flags::ENABLE_3D.bits(),
    Mode5_3d = Self::Mode5_2d as u32 | Flags::BG0.bits() | Flags::ENABLE_3D.bits(),
    Mode6_3d = Self::Mode6_2d as u32 | Flags::BG0.bits() | Flags::ENABLE_3D.bits(),

    /// Display directly from RAM
    ModeFifo = Flags::DISPLAY_RAM.bits(),
    /// Displays directly from Bank A in LCD mode
    ModeFb0 = Flags::DISPLAY_VRAM.bits() | Flags::VRAM_A.bits(),
    /// Displays directly from Bank B in LCD mode
    ModeFb1 = Flags::DISPLAY_VRAM.bits() | Flags::VRAM_B.bits(),
    /// Displays directly from Bank C in LCD mode
    ModeFb2 = Flags::DISPLAY_VRAM.bits() | Flags::VRAM_C.bits(),
    /// Displays directly from Bank D in LCD mode
    ModeFb3 = Flags::DISPLAY_VRAM.bits() | Flags::VRAM_D.bits(),
}

/// Sets video mode for Main
/// To control whether this renders to the top LCD or the bottom one, use [`main_engine_on`](crate::system::main_engine_on)
pub fn set_mode(mode: Mode) {
    unsafe {
        nds_sys::video::REG_DISPCNT.write_volatile(mode as u32);
    }
}

/// Sets video mode for Sub
/// Panics if an invalid mode is passed
/// To control whether this renders to the top LCD or the bottom one, use [`main_engine_on`](crate::system::main_engine_on)
pub fn set_mode_sub(mode: Mode) {
    match mode {
        Mode::Mode0_3d
        | Mode::Mode1_3d
        | Mode::Mode2_3d
        | Mode::Mode3_3d
        | Mode::Mode4_3d
        | Mode::Mode5_3d
        | Mode::Mode6_3d => {
            panic!("3D modes are not valid for Sub engine");
        }
        Mode::Mode6_2d => {
            panic!("Mode 6 is not valid for Sub engine");
        }
        Mode::ModeFb0 | Mode::ModeFb1 | Mode::ModeFb2 | Mode::ModeFb3 => {
            panic!("Modes Fb0-Fb3 are not valid for Sub engine");
        }
        _ => unsafe {
            nds_sys::video::REG_DISPCNT_SUB.write_volatile(mode as u32);
        },
    }
}


pub fn set_backdrop_color(color: Color) {
    unsafe {
        nds_sys::video::BG_PALETTE.write_volatile(color.0);
    }
}
pub fn set_backdrop_color_sub(color: Color) {
    unsafe {
        nds_sys::video::BG_PALETTE_SUB.write_volatile(color.0);
    }
}

