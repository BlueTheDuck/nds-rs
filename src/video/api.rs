use crate::sys::background as bg;
use crate::sys::video;

pub fn create_main_mode0(
    map_base: u8,
    tiles_base: u8,
) -> Engine<TextBg, TextBg, TextBg, TextBg, true> {
    Engine::new(map_base, tiles_base)
}
/* pub fn create_sub_mode0(
    map_base: u8,
    tiles_base: u8,
) -> Engine<TextBg, TextBg, TextBg, TextBg, false> {
    Engine::new(map_base, tiles_base)
}
 */
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

/* #region Mode 0 */
impl<const MAIN: bool> Engine<TextBg, TextBg, TextBg, TextBg, MAIN> {
    fn new(map_base: u8, tiles_base: u8) -> Engine<TextBg, TextBg, TextBg, TextBg, MAIN> {
        Engine {
            layer0: TextBg::default(),
            layer1: TextBg::default(),
            layer2: TextBg::default(),
            layer3: TextBg::default(),
            display0: false,
            display1: false,
            display2: false,
            display3: false,
            map_base,
            tiles_base,
        }
    }

    pub fn as_bitflags(&self) -> u32 {
        debug_assert!(
            self.map_base <= 0b111,
            "self.screen_base must be a valid u3"
        );
        debug_assert!(
            self.tiles_base <= 0b111,
            "self.character_base must be a valid u3"
        );

        let mut flags = video::Flags::DISPLAY_ON | self.bg_mode();

        if self.display0 {
            flags |= video::Flags::BG0;
        }
        if self.display1 {
            flags |= video::Flags::BG1;
        }
        if self.display2 {
            flags |= video::Flags::BG2;
        }
        if self.display3 {
            flags |= video::Flags::BG3;
        }

        let screen_base = (self.map_base as u32) << video::SCREEN_BASE_OFFSET;
        let character_base = (self.tiles_base as u32) << video::CHARACTER_BASE_OFFSET;

        flags.bits() | screen_base | character_base
    }

    pub unsafe fn commit(&self) {
        if MAIN {
            video::REG_DISPCNT.write_volatile(self.as_bitflags());
        } else {
            video::REG_DISPCNT_SUB.write_volatile(self.as_bitflags());
        }
        let bgs = if MAIN {
            (
                bg::registers::BG0CNT,
                bg::registers::BG1CNT,
                bg::registers::BG2CNT,
                bg::registers::BG3CNT,
            )
        } else {
            (
                bg::registers::DB_BG0CNT,
                bg::registers::DB_BG1CNT,
                bg::registers::DB_BG2CNT,
                bg::registers::DB_BG3CNT,
            )
        };
        bgs.0.write_volatile(self.layer0.as_bitflags());
        bgs.1.write_volatile(self.layer1.as_bitflags());
        bgs.2.write_volatile(self.layer2.as_bitflags());
        bgs.3.write_volatile(self.layer3.as_bitflags());
    }

    /// Returns mode. **Only available on valid combinations of backgrounds**
    pub fn bg_mode(&self) -> video::Flags {
        video::Flags::MODE0
    }

    // Screen/map ptr
    pub fn get_map_ptr(&self, bg: u8) -> *mut u16 {
        let map_base_block = {
            let map_base = match bg {
                0 => self.layer0.map_base,
                1 => self.layer1.map_base,
                2 => self.layer2.map_base,
                3 => self.layer3.map_base,
                _ => unreachable!(),
            } as usize;
            map_base * 0x800 / 2
        };
        unsafe {
            if MAIN {
                return video::BG_GFX
                    .add(self.map_base as usize * 0x10000 / 2 + map_base_block);
            } else {
                return video::BG_GFX_SUB.add(map_base_block);
            }
        }
    }

    // Character/tiles ptr
    pub fn get_tiles_ptr(&self, bg: u8) -> *mut u16 {
        let tiles_base_block = {
            let tiles_base = match bg {
                0 => self.layer0.tiles_base,
                1 => self.layer1.tiles_base,
                2 => self.layer2.tiles_base,
                3 => self.layer3.tiles_base,
                _ => unreachable!(),
            } as usize;
            tiles_base * 0x4000 / 2
        };
        unsafe {
            if MAIN {
                return video::BG_GFX
                    .add(self.tiles_base as usize * 0x10000 / 2 + tiles_base_block);
            } else {
                return video::BG_GFX_SUB.add(tiles_base_block);
            }
        }
    }

    pub fn set_visibility(&mut self, bg: u8, show: bool) {
        match bg {
            0 => self.display0 = show,
            1 => self.display1 = show,
            2 => self.display2 = show,
            3 => self.display3 = show,
            _ => unreachable!(),
        };
    }

    pub fn set_bg_offsets(&mut self, bg: u8, map_offset: u8, tiles_offset: u8) {
        debug_assert!(map_offset <= 0b11111, "screen_base must be a valid u5");
        match bg {
            0 => self.layer0.map_base = map_offset,
            1 => self.layer1.map_base = map_offset,
            2 => self.layer2.map_base = map_offset,
            3 => self.layer3.map_base = map_offset,
            _ => unreachable!(),
        };
        debug_assert!(
            tiles_offset <= 0b1111,
            "character_base must be a valid u4"
        );
        match bg {
            0 => self.layer0.tiles_base = tiles_offset,
            1 => self.layer1.tiles_base = tiles_offset,
            2 => self.layer2.tiles_base = tiles_offset,
            3 => self.layer3.tiles_base = tiles_offset,
            _ => unreachable!(),
        };
    }
}

/* #endregion */
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
/* #endregion */
