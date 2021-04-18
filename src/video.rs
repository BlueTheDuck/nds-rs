#![allow(dead_code)]

use nds_sys::video::Flags;

/// Width of the screens, in pixels
pub static WIDTH: usize = 256;
/// Height of the screens, in pixels
pub static HEIGHT: usize = 192;

/// Control registers related to video operations
/// It is "technically" unsafe to use them, but you can't break
/// Rust's safety with them
pub mod registers {
    pub use nds_sys::video::{VRAM_A, VRAM_B, VRAM_C, VRAM_D};
}

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

macro_rules! vramOffset {
    ($offset:literal) => {
        $offset << 3
    };
}

/// Bank A (128KB)
pub mod vram_a {
    use nds_sys::video::{VRAM_A_CR as VRAM_CR, VRAM_ENABLE};

    pub static MAIN_BG: BankMode = BankMode::MainBgSlot0;
    pub static MAIN_SPRITE: BankMode = BankMode::MainSpriteSlot0;
    pub static TEXTURE: BankMode = BankMode::TextureSlot0;

    /// Type of mapping that can be used with Bank A
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum BankMode {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background slot 0. (Address: 0x06000000)
        MainBgSlot0 = 1,
        /// Maps Bank to Main's background slot 1. (Address: 0x06020000)
        MainBgSlot1 = 1 | vramOffset!(1),
        /// Maps Bank to Main's background slot 2. (Address: 0x06040000)
        MainBgSlot2 = 1 | vramOffset!(2),
        /// Maps Bank to Main's background slot 3. (Address: 0x06060000)
        MainBgSlot3 = 1 | vramOffset!(3),

        /// Maps Bank to Main's sprites slot 0. (Address: 0x06400000)
        MainSpriteSlot0 = 2,
        /// Maps Bank to Main's sprites slot 1. (Address: 0x06420000)
        MainSpriteSlot1 = 2 | vramOffset!(1),

        /// Maps Bank to 3D texture slot 0.
        TextureSlot0 = 3,
        /// Maps Bank to 3D texture slot 1.
        TextureSlot1 = 3 | vramOffset!(1),
        /// Maps Bank to 3D texture slot 2.
        TextureSlot2 = 3 | vramOffset!(2),
        /// Maps Bank to 3D texture slot 3.
        TextureSlot3 = 3 | vramOffset!(3),
    }

    /// Sets the mapping for Bank A
    pub fn set_bank_mode(mode: BankMode) {
        unsafe {
            VRAM_CR.write_volatile(VRAM_ENABLE | (mode as u8));
        }
    }
}

/// Bank B (128KB)
pub mod vram_b {
    use nds_sys::video::{VRAM_B_CR as VRAM_CR, VRAM_ENABLE};

    pub static MAIN_BG: BankMode = BankMode::MainBgSlot1;
    pub static MAIN_SPRITE: BankMode = BankMode::MainSpriteSlot0;
    pub static TEXTURE: BankMode = BankMode::TextureSlot1;

    /// Type of mapping that can be used with Bank B
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum BankMode {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background slot 0. (Address: 0x06000000)
        MainBgSlot0 = 1,
        /// Maps Bank to Main's background slot 1. (Address: 0x06020000)
        MainBgSlot1 = 1 | vramOffset!(1),
        /// Maps Bank to Main's background slot 2. (Address: 0x06040000)
        MainBgSlot2 = 1 | vramOffset!(2),
        /// Maps Bank to Main's background slot 3. (Address: 0x06060000)
        MainBgSlot3 = 1 | vramOffset!(3),

        /// Maps Bank to Main's sprites slot 0. (Address: 0x06400000)
        MainSpriteSlot0 = 2,
        /// Maps Bank to Main's sprites slot 1. (Address: 0x06420000)
        MainSpriteSlot1 = 2 | vramOffset!(1),

        /// Maps Bank to 3D texture slot 0.
        TextureSlot0 = 3,
        /// Maps Bank to 3D texture slot 1.
        TextureSlot1 = 3 | vramOffset!(1),
        /// Maps Bank to 3D texture slot 2.
        TextureSlot2 = 3 | vramOffset!(2),
        /// Maps Bank to 3D texture slot 3.
        TextureSlot3 = 3 | vramOffset!(3),
    }

    /// Sets the mapping for Bank B
    pub fn set_bank_mode(mode: BankMode) {
        unsafe {
            VRAM_CR.write_volatile(VRAM_ENABLE | (mode as u8));
        }
    }
}

/// Bank C (128KB)
pub mod vram_c {
    use nds_sys::video::{VRAM_C_CR as VRAM_CR, VRAM_ENABLE};

    pub static MAIN_BG: BankMode = BankMode::MainBgSlot2;
    pub static ARM7: BankMode = BankMode::Arm7Slot0;
    pub static TEXTURE: BankMode = BankMode::TextureSlot2;

    /// Type of mapping that can be used with Bank C
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum BankMode {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background slot 0. (Address: 0x06000000)
        MainBgSlot0 = 1,
        /// Maps Bank to Main's background slot 1. (Address: 0x06020000)
        MainBgSlot1 = 1 | vramOffset!(1),
        /// Maps Bank to Main's background slot 2. (Address: 0x06040000)
        MainBgSlot2 = 1 | vramOffset!(2),
        /// Maps Bank to Main's background slot 3. (Address: 0x06060000)
        MainBgSlot3 = 1 | vramOffset!(3),

        /// Maps Bank to workram slot 0 of the ARM7. (Address: 0x06000000)
        Arm7Slot0 = 2,
        /// Maps Bank to workram slot 1 of the ARM7. (Address: 0x06020000)
        Arm7Slot1 = 2 | vramOffset!(1),

        /// Maps Bank to Sub's background slot 0. (Address: 0x06200000)
        SubBgSlot0 = 4,

        /// Maps Bank to 3d texture slot 0.
        TextureSlot0 = 3,
        /// Maps Bank to 3d texture slot 1.
        TextureSlot1 = 3 | vramOffset!(1),
        /// Maps Bank to 3d texture slot 2.
        TextureSlot2 = 3 | vramOffset!(2),
        /// Maps Bank to 3d texture slot 3.
        TextureSlot3 = 3 | vramOffset!(3),
    }

    /// Sets the mapping for Bank C
    pub fn set_bank_mode(mode: BankMode) {
        unsafe {
            VRAM_CR.write_volatile(VRAM_ENABLE | (mode as u8));
        }
    }
}

/// Bank D (128KB)
pub mod vram_d {
    use nds_sys::video::{VRAM_C_CR as VRAM_CR, VRAM_ENABLE};

    pub static MAIN_BG: BankMode = BankMode::MainBgSlot3;
    pub static ARM7: BankMode = BankMode::Arm7Slot1;
    pub static TEXTURE: BankMode = BankMode::TextureSlot3;

    /// Type of mapping that can be used with Bank D
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum BankMode {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background slot 0. (Address: 0x06000000)
        MainBgSlot0 = 1,
        /// Maps Bank to Main's background slot 1. (Address: 0x06020000)
        MainBgSlot1 = 1 | vramOffset!(1),
        /// Maps Bank to Main's background slot 2. (Address: 0x06040000)
        MainBgSlot2 = 1 | vramOffset!(2),
        /// Maps Bank to Main's background slot 3. (Address: 0x06060000)
        MainBgSlot3 = 1 | vramOffset!(3),

        /// Maps Bank to workram slot 0 of the ARM7. (Address: 0x06000000)
        Arm7Slot0 = 2,
        /// Maps Bank to workram slot 1 of the ARM7. (Address: 0x06020000)
        Arm7Slot1 = 2 | vramOffset!(1),

        /// Maps Bank to Sub's sprite slot 0. (Address: 0x06200000)
        SubSpriteSlot0 = 4,

        /// Maps Bank to 3D texture slot 0.
        TextureSlot0 = 3,
        /// Maps Bank to 3D texture slot 1.
        TextureSlot1 = 3 | vramOffset!(1),
        /// Maps Bank to 3D texture slot 2.
        TextureSlot2 = 3 | vramOffset!(2),
        /// Maps Bank to 3D texture slot 3.
        TextureSlot3 = 3 | vramOffset!(3),
    }

    /// Sets the mapping for Bank D
    pub fn set_bank_mode(mode: BankMode) {
        unsafe {
            VRAM_CR.write_volatile(VRAM_ENABLE | (mode as u8));
        }
    }
}

/// Bank E (64KB)
pub mod vram_e {
    use nds_sys::video::{VRAM_ENABLE, VRAM_E_CR as VRAM_CR};

    /// Type of mapping that can be used with Bank E
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum BankMode {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background, first half, slot 0
        MainBg = 1,

        /// Maps Bank to Main's sprites, first half, slot 0
        MainSprite = 2,

        /// Maps Bank to 3D texture palette slots 0-3
        TexturePalette = 3,

        /// Maps Bank to Main's background extended palette
        BgExtendedPalette = 4,
    }

    /// Sets the mapping for Bank E
    pub fn set_bank_mode(mode: BankMode) {
        unsafe {
            VRAM_CR.write_volatile(VRAM_ENABLE | (mode as u8));
        }
    }
}

/// Bank F (16KB)
pub mod vram_f {
    use nds_sys::video::{VRAM_ENABLE, VRAM_F_CR as VRAM_CR};

    pub static MAIN_BG: BankMode = BankMode::MainBgFirstPart;
    pub static MAIN_SPRITE: BankMode = BankMode::MainSpriteFirstPart;
    pub static TEXTURE_PALETTE: BankMode = BankMode::TexturePaletteSlot0;
    pub static BG_EXT_PALETTE: BankMode = BankMode::BgExtPaletteSlot01;

    /// Type of mapping that can be used with Bank F
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum BankMode {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background first part, first half, slot 0. (Address: 0x06000000)
        MainBgFirstPart = 1,
        /// Maps Bank to Main's background second part, first half, slot 0. (Address: 0x06004000)
        MainBgSecondPart = 1 | vramOffset!(1),
        /// Maps Bank to Main's background first part, second half, slot 0. (Address: 0x06010000)
        MainBgFirstPartSecondHalf = 1 | vramOffset!(2),
        /// Maps Bank to Main's background second part, second half, slot 0. (Address: 0x06014000)
        MainBgSecondPartSecondHalf = 1 | vramOffset!(3),

        /// Maps Bank to Main sprites first part of slot 0 (Address: 0x06400000)
        MainSpriteFirstPart = 2,
        /// Maps Bank to Main sprites second part of slot 0 (Address: 0x06404000)
        MainSpriteSecondPart = 2 | vramOffset!(1),
        /// Maps Bank to Main sprites first part, second half  (Address: 0x06410000)
        MainSpriteFirstPartSecondHalf = 2 | vramOffset!(2),
        /// Maps Bank to Main sprites second part, second half (Address: 0x06414000)
        MainSpriteSecondPartSecondHalf = 2 | vramOffset!(3),

        /// Maps Bank to 3D texture palette slot 0
        TexturePaletteSlot0 = 3,
        /// Maps Bank to 3D texture palette slot 1
        TexturePaletteSlot1 = 3 | vramOffset!(1),
        /// Maps Bank to 3D texture palette slot 4
        TexturePaletteSlot4 = 3 | vramOffset!(2),
        /// Maps Bank to 3D texture palette slot 5
        TexturePaletteSlot5 = 3 | vramOffset!(3),

        /// Maps Bank to Main background extended palette, slots 0 and 1
        BgExtPaletteSlot01 = 4,
        /// Maps Bank to Main background extended palette, slots 2 and 3
        BgExtPaletteSlot23 = 4 | vramOffset!(1),

        /// Maps Bank to Main sprites extended palette
        SpriteExtPalette = 5,
    }

    /// Sets the mapping for Bank F
    pub fn set_bank_mode(mode: BankMode) {
        unsafe {
            VRAM_CR.write_volatile(VRAM_ENABLE | (mode as u8));
        }
    }
}

/// Bank G (16KB)
pub mod vram_g {
    use nds_sys::video::{VRAM_ENABLE, VRAM_G_CR as VRAM_CR};

    pub static MAIN_BG: BankMode = BankMode::MainBgFirstPart;
    pub static MAIN_SPRITE: BankMode = BankMode::MainSpriteFirstPart;
    pub static TEXTURE_PALETTE: BankMode = BankMode::TexturePaletteSlot0;
    pub static BG_EXT_PALETTE: BankMode = BankMode::BgExtPaletteSlot01;

    /// Type of mapping that can be used with Bank G
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum BankMode {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Main's background first part, first half, slot 0. (Address: 0x06000000)
        MainBgFirstPart = 1,
        /// Maps Bank to Main's background second part, first half, slot 0. (Address: 0x06004000)
        MainBgSecondPart = 1 | vramOffset!(1),
        /// Maps Bank to Main's background first part, second half, slot 0. (Address: 0x06010000)
        MainBgFirstPartSecondHalf = 1 | vramOffset!(2),
        /// Maps Bank to Main's background second part, second half, slot 0. (Address: 0x06014000)
        MainBgSecondPartSecondHalf = 1 | vramOffset!(3),

        /// Maps Bank to Main sprites first part of slot 0 (Address: 0x06400000)
        MainSpriteFirstPart = 2,
        /// Maps Bank to Main sprites second part of slot 0 (Address: 0x06404000)
        MainSpriteSecondPart = 2 | vramOffset!(1),
        /// Maps Bank to Main sprites first part, second half  (Address: 0x06410000)
        MainSpriteFirstPartSecondHalf = 2 | vramOffset!(2),
        /// Maps Bank to Main sprites second part, second half (Address: 0x06414000)
        MainSpriteSecondPartSecondHalf = 2 | vramOffset!(3),

        /// Maps Bank to 3D texture palette slot 0
        TexturePaletteSlot0 = 3,
        /// Maps Bank to 3D texture palette slot 1
        TexturePaletteSlot1 = 3 | vramOffset!(1),
        /// Maps Bank to 3D texture palette slot 4
        TexturePaletteSlot4 = 3 | vramOffset!(2),
        /// Maps Bank to 3D texture palette slot 5
        TexturePaletteSlot5 = 3 | vramOffset!(3),

        /// Maps Bank to Main background extended palette, slots 0 and 1
        BgExtPaletteSlot01 = 4,
        /// Maps Bank to Main background extended palette, slots 2 and 3
        BgExtPaletteSlot23 = 4 | vramOffset!(1),

        /// Maps Bank to Main sprites extended palette
        SpriteExtPalette = 5,
    }

    /// Sets the mapping for Bank G
    pub fn set_bank_mode(mode: BankMode) {
        unsafe {
            VRAM_CR.write_volatile(VRAM_ENABLE | (mode as u8));
        }
    }
}

/// Bank H (32KB)
pub mod vram_h {
    use nds_sys::video::{VRAM_ENABLE, VRAM_H_CR as VRAM_CR};

    /// Type of mapping that can be used with Bank H
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum BankMode {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Sub's background first 2 parts of slot 0
        SubBg = 1,

        /// Maps Bank to Sub's background extended palette
        SubBgExtPalette = 2,
    }

    /// Sets the mapping for Bank H
    pub fn set_bank_mode(mode: BankMode) {
        unsafe {
            VRAM_CR.write_volatile(VRAM_ENABLE | (mode as u8));
        }
    }
}

/// Bank I (16KB)
pub mod vram_i {
    use nds_sys::video::{VRAM_ENABLE, VRAM_I_CR as VRAM_CR};

    /// Type of mapping that can be used with Bank I
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum BankMode {
        /// Maps Bank to LCD.
        Lcd = 0,

        /// Maps Bank to Sub's background thirth part of slot 0. (Address: 0x06208000)
        SubBg0 = 1,

        /// Maps Bank to Sub's sprites
        SubSprite = 2,

        /// Maps Bank to Sub's sprites extended palette
        SubSpriteExtPalette = 3,
    }

    /// Sets the mapping for Bank I
    pub fn set_bank_mode(mode: BankMode) {
        unsafe {
            VRAM_CR.write_volatile(VRAM_ENABLE | (mode as u8));
        }
    }
}

pub mod colors {
    #![allow(clippy::unusual_byte_groupings)]

    use super::Color;

    pub static BLACK: Color = Color(0b0_00000_00000_00000);
    pub static RED: Color = Color(0b0_00000_00000_11111);
    pub static GREEN: Color = Color(0b0_00000_11111_00000);
    pub static BLUE: Color = Color(0b0_11111_00000_00000);
    pub static TRANSPARENT: Color = Color(0b1_00000_00000_00000);
}

/// ABGR1555 color
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
