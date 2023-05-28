/* use nds_sys::video::DispCntFlags;

use super::Color;



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

 */