use nds_sys::video::{*, self};

/// Width of the screens, in pixels
pub static WIDTH: usize = 256;
/// Height of the screens, in pixels
pub static HEIGHT: usize = 192;

pub mod registers {
    pub use nds_sys::video::VRAM_A;
}

const MODE_ENABLE_3D: u32 = 1 << 3;

/// Used to select what backgrounds to draw
#[repr(u32)]
enum DisplayBg {
    Bg0 = bit!(8),
    Bg1 = bit!(9),
    Bg2 = bit!(10),
    Bg3 = bit!(11),
}

/// Constants to be used for REG_DISPCNT and REG_DISPCNT_SUB.
/// The DS has 2 rendering engines, Main and Sub, that can be put in different modes (6 or 5 respectively).
/// Mode6_2d and the ones suffixed "3D" are only valid for Main
/// (The 3D engine renders on Background 0).
/// Modes FB0-FB3 ("LCD" mode) map the Banks A-D respectively to pixels on screen.
#[repr(u32)]
pub enum Mode {
    Mode0_2d = 0x10000,
    Mode1_2d = 0x10001,
    Mode2_2d = 0x10002,
    Mode3_2d = 0x10003,
    Mode4_2d = 0x10004,
    Mode5_2d = 0x10005,
    Mode6_2d = 0x10006,

    Mode0_3d = (Self::Mode0_2d as u32 | DisplayBg::Bg0 as u32 | MODE_ENABLE_3D),
    Mode1_3d = (Self::Mode1_2d as u32 | DisplayBg::Bg0 as u32 | MODE_ENABLE_3D),
    Mode2_3d = (Self::Mode2_2d as u32 | DisplayBg::Bg0 as u32 | MODE_ENABLE_3D),
    Mode3_3d = (Self::Mode3_2d as u32 | DisplayBg::Bg0 as u32 | MODE_ENABLE_3D),
    Mode4_3d = (Self::Mode4_2d as u32 | DisplayBg::Bg0 as u32 | MODE_ENABLE_3D),
    Mode5_3d = (Self::Mode5_2d as u32 | DisplayBg::Bg0 as u32 | MODE_ENABLE_3D),
    Mode6_3d = (Self::Mode6_2d as u32 | DisplayBg::Bg0 as u32 | MODE_ENABLE_3D),

    ModeFifo = (3 << 16),
    /// Displays directly from Bank A in LCD mode
    ModeFb0 = 0x00020000,
    /// Displays directly from Bank B in LCD mode
    ModeFb1 = 0x00060000,
    /// Displays directly from Bank C in LCD mode
    ModeFb2 = 0x000A0000,
    /// Displays directly from Bank D in LCD mode
    ModeFb3 = 0x000E0000,
}

pub fn set_mode(mode: Mode) -> () {
    unsafe {
        nds_sys::video::REG_DISPCNT.write_volatile(mode as u32);
    }
}

pub fn set_mode_sub(mode: Mode) -> () {
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
        _ => unsafe {
            nds_sys::video::REG_DISPCNT_SUB.write_volatile(mode as u32);
        },
    }
}

macro_rules! vramOffset {
    ($offset:literal) => {
        $offset << 3
    };
}

pub mod VramA {
    use nds_sys::video::{VRAM_A_CR, VRAM_ENABLE};

    /// Type of mapping that can be used with Bank A
    #[repr(u8)]
    pub enum BankMode {
        /// Maps Bank A to lcd.
        Lcd = 0,
        /// Maps Bank A to main engine background slot 0.
        MainBg0x06000000 = 1 | vramOffset!(0),
        /// Maps Bank A to main engine background slot 1.
        MainBg0x06020000 = 1 | vramOffset!(1),
        /// Maps Bank A to main engine background slot 2.
        MainBg0x06040000 = 1 | vramOffset!(2),
        /// Maps Bank A to main engine background slot 3.
        MainBg0x06060000 = 1 | vramOffset!(3),
        /// Maps Bank A to main engine sprites slot 0.
        MainSprite0x06400000 = 2 | vramOffset!(0),
        /// Maps Bank A to main engine sprites slot 1.
        MainSprite0x06420000 = 2 | vramOffset!(1),
        /// Maps Bank A to 3d texture slot 0.
        TextureSlot0 = 3 | vramOffset!(0),
        /// Maps Bank A to 3d texture slot 1.
        TextureSlot1 = 3 | vramOffset!(1),
        /// Maps Bank A to 3d texture slot 2.
        TextureSlot2 = 3 | vramOffset!(2),
        /// Maps Bank A to 3d texture slot 3.
        TextureSlot3 = 3 | vramOffset!(3),
    }

    /// Sets the mapping for Bank A
    pub fn set_bank_mode(mode: BankMode) {
        unsafe {
            VRAM_A_CR.write_volatile(VRAM_ENABLE | (mode as u8));
        }
    }
}

pub mod colors {
    use super::Color;

    pub static BLACK: Color = Color(0b0_00000_00000_00000);
    pub static RED: Color = Color(0b0_00000_00000_11111);
    pub static GREEN: Color = Color(0b0_00000_11111_00000);
    pub static BLUE: Color = Color(0b0_11111_00000_00000);
    pub static TRANSPARENT: Color = Color(0b1_00000_00000_00000);
}

/// ABGR1555 color
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Color(pub u16);
impl Color {
    pub fn get(&self) -> u16 {
        self.0
    }
    pub fn red(self) -> u16 {
        self.0 & colors::RED.0
    }
    pub fn green(self) -> u16 {
        (self.0 & colors::GREEN.0) >> 5
    }
    pub fn blue(self) -> u16 {
        (self.0 & colors::BLUE.0) >> 10
    }
    pub fn set_red(&mut self, amount: u16) {
        self.0 &= !colors::RED.0;
        self.0 |= amount;
    }
    pub fn set_green(&mut self, amount: u16) {
        self.0 &= !colors::GREEN.0;
        self.0 |= amount << 5;
    }
    pub fn set_blue(&mut self, amount: u16) {
        self.0 &= !colors::BLUE.0;
        self.0 |= amount << 10;
    }
}
impl From<Color> for u16 {
    fn from(color: Color) -> Self {
        color.0
    }
}
