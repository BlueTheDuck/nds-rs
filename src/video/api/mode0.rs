use core::mem::size_of;

use crate::sys::background as bg;
use crate::sys::video;

use super::Engine;
use super::TextBg;
use super::TextTileData;

///
/// Creates a config object for the Main Engine. Uses Mode 0 2D¹.
///
/// ¹ https://libnds.devkitpro.org/video_8h.html#a1e46218ee302fcc8c77e4ea0968ea149
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

/* #region Mode 0 */
impl<const MAIN: bool> Engine<TextBg, TextBg, TextBg, TextBg, MAIN> {
    fn new(map_base: u8, tiles_base: u8) -> Engine<TextBg, TextBg, TextBg, TextBg, MAIN> {
        debug_assert!(map_base <= 0b111, "screen_base must be a valid u3");
        debug_assert!(tiles_base <= 0b111, "character_base must be a valid u3");
        Engine {
            layer0: TextBg::default(),
            layer1: TextBg::default(),
            layer2: TextBg::default(),
            layer3: TextBg::default(),
            display0: false,
            display1: false,
            display2: false,
            display3: false,
            display_obj: false,
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

        if self.display_obj {
            flags |= video::Flags::OBJECTS;
        }
        if self.display3 {
            flags |= video::Flags::BG3;
        }
        if self.display2 {
            flags |= video::Flags::BG2;
        }
        if self.display1 {
            flags |= video::Flags::BG1;
        }
        if self.display0 {
            flags |= video::Flags::BG0;
        }

        let map_base = (self.map_base as u32) << video::MAP_BASE_OFFSET;
        let tiles_base = (self.tiles_base as u32) << video::TILES_BASE_OFFSET;

        flags.bits() | map_base | tiles_base
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
    pub fn get_map(&self, bg: u8) -> &mut [TextTileData] {
        let map_base_block = {
            let map_base = match bg {
                0 => self.layer0.map_base,
                1 => self.layer1.map_base,
                2 => self.layer2.map_base,
                3 => self.layer3.map_base,
                _ => unreachable!()
            } as usize;
            map_base * 0x800 / 2
        };
        let map_size = match bg {
            0 => self.layer0.get_map_size(),
            1 => self.layer1.get_map_size(),
            2 => self.layer2.get_map_size(),
            3 => self.layer3.get_map_size(),
            _ => unreachable!()
        };
        let ptr;
        unsafe {
            if MAIN {
                // 0x06000000 + map_base * 0x10000 + map_block * 0x800
                ptr = video::BG_GFX.add(self.map_base as usize * 0x10000 / 2 + map_base_block);
            } else {
                // 0x06200000 + map_block * 0x800
                ptr = video::BG_GFX_SUB.add(map_base_block);
            }
        }
        unsafe {
            core::slice::from_raw_parts_mut(ptr as *mut _, map_size / size_of::<TextTileData>())
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
                _ => unreachable!()
            } as usize;
            tiles_base * 0x4000 / 2
        };
        unsafe {
            if MAIN {
                // 0x06000000 + tiles_base * 0x10000 + tiles_block * 0x4000
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
            _ => unreachable!()
        };
    }

    /// `map_offset` must be less than 32
    /// `tiles_offset` must be less than 16
    pub fn set_bg_offsets(&mut self, bg: u8, map_offset: u8, tiles_offset: u8) {
        debug_assert!(map_offset <= 0b11111, "screen_base must be a valid u5");
        match bg {
            0 => self.layer0.map_base = map_offset,
            1 => self.layer1.map_base = map_offset,
            2 => self.layer2.map_base = map_offset,
            3 => self.layer3.map_base = map_offset,
            _ => unreachable!()
        };
        debug_assert!(tiles_offset <= 0b1111, "character_base must be a valid u4");
        match bg {
            0 => self.layer0.tiles_base = tiles_offset,
            1 => self.layer1.tiles_base = tiles_offset,
            2 => self.layer2.tiles_base = tiles_offset,
            3 => self.layer3.tiles_base = tiles_offset,
            _ => unreachable!()
        };
    }
}

/* #endregion */
