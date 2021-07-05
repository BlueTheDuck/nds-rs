use crate::sys::background as bg;
use crate::sys::video;

pub struct Engine<L1, L2, L3, L4, const MAIN: bool> {
    layer0: L1,
    layer1: L2,
    layer2: L3,
    layer3: L4,
    display0: bool,
    display1: bool,
    display2: bool,
    display3: bool,
    /// Actually a "`u3`", also called "Screen"
    map_base: u8,
    /// Actually a "`u3`", also called "Character"
    tiles_base: u8,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum BackgroundId {
    Bg0,
    Bg1,
    Bg2,
    Bg3,
}

/* #region Text BG */
enum TextScreenSize {
    /// 256x256
    Small,
    /// 512x256
    Horizonal,
    /// 256x512
    Vertical,
    /// 512x512
    Big,
}
impl From<&TextScreenSize> for bg::Flags {
    fn from(v: &TextScreenSize) -> Self {
        match v {
            TextScreenSize::Small => bg::Flags::SCREENSIZE_256_256,
            TextScreenSize::Horizonal => bg::Flags::SCREENSIZE_512_256,
            TextScreenSize::Vertical => bg::Flags::SCREENSIZE_256_512,
            TextScreenSize::Big => bg::Flags::SCREENSIZE_512_512,
        }
    }
}

pub struct TextBg {
    screen_size: TextScreenSize,
    /// Actually a "`u5`", also called "Map"
    map_base: u8,
    /// Actually a "`u4`", also called "Tiles"
    tiles_base: u8,
}
impl TextBg {
    pub fn as_bitflags(&self) -> u16 {
        let mut flags: bg::Flags = (&self.screen_size).into();
        flags |= bg::Flags::FULLCOLOR;
        // TODO: The rest of flags

        let screen_base = (self.map_base as u16) << bg::SCREEN_BASE_OFFSET;
        let character_base = (self.tiles_base as u16) << bg::CHARACTER_BASE_OFFSET;

        flags.bits() | screen_base | character_base
    }

    pub fn get_map_size(&self) -> usize {
        match self.screen_size {
            TextScreenSize::Small => 2048,
            TextScreenSize::Horizonal => 4096,
            TextScreenSize::Vertical => 4096,
            TextScreenSize::Big => 8192,
        }
    }
}
impl Default for TextBg {
    fn default() -> Self {
        Self {
            screen_size: TextScreenSize::Small,
            map_base: 0,
            tiles_base: 0,
        }
    }
}

const TILE_ID_MASK: u16 = 0b11_11111111;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct TextBGMap(u16);
impl TextBGMap {
    pub fn get_palette_idx(&self) -> u8 {
        let pal = self.0 >> 12;
        return (pal & 0b1111) as u8;
    }
    pub fn get_tile_id(&self) -> u16 {
        self.0 & TILE_ID_MASK
    }
    pub fn set_tile_id(&mut self, id: u16) {
        assert!(id <= TILE_ID_MASK);
        self.0 = (self.0 & !TILE_ID_MASK) | id;
    }
    pub fn get_flip(&self) -> (bool, bool) {
        let bits = (self.0 >> 10) & 0b11;
        let flip: (bool, bool) = (bits & 0b10 != 0, bits & 0b01 != 0);
        return flip;
    }
    pub fn set_flip(&mut self, flips: (bool, bool)) {
        if flips.0 {
            self.0 |= 0b00001000_00000000;
        }
        if flips.1 {
            self.0 |= 0b00000100_00000000;
        }
    }
}

/* #endregion */

mod mode0;

pub use mode0::create_main_mode0;
