use crate::sys::background as bg;

pub struct Engine<L1, L2, L3, L4, const MAIN: bool> {
    layer3: L4,
    layer2: L3,
    layer1: L2,
    layer0: L1,
    display3: bool,
    display2: bool,
    display1: bool,
    display0: bool,
    pub display_obj: bool,
    /// Actually a "`u3`", also called "Screen"
    map_base: u8,
    /// Actually a "`u3`", also called "Character"
    tiles_base: u8,
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

/// Represents one tile on the screen for text 
/// backgrounds or affine extended backgrounds.
#[derive(Debug, Copy, Clone, Default)]
#[repr(transparent)]
pub struct TextTileData(u16);
impl TextTileData {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn palette(&self) -> u8 {
        (self.0 >> 12) as u8
    }
    pub fn set_palette(&mut self, palette: u8) {
        self.0 = (self.0 & 0x0FFF) | ((palette as u16) << 12);
    }
    pub fn hflip(&self) -> bool {
        (self.0 & bit!(10)) != 0
    }
    pub fn set_hflip(&mut self, hflip: bool) {
        if hflip {
            self.0 |= bit!(10);
        } else {
            self.0 &= !bit!(10);
        }
    }
    pub fn vflip(&self) -> bool {
        (self.0 & bit!(11)) != 0
    }
    pub fn set_vflip(&mut self, vflip: bool) {
        if vflip {
            self.0 |= bit!(11);
        } else {
            self.0 &= !bit!(11);
        }
    }
    pub fn tile_index(&self) -> u16 {
        self.0 & 0x03FF
    }
    pub fn set_tile_index(&mut self, tile_index: u16) {
        self.0 = (self.0 & 0xFC00) | (tile_index & 0x03FF);
    }
}


#[derive(Clone, Copy)]
pub struct SixteenTileData(pub u16);
impl SixteenTileData {

}

/* #endregion */

mod mode0;

pub use mode0::create_main_mode0;
