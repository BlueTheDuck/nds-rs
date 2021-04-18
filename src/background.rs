use nds_sys::background::{self, bgInit_call};

pub use background::{BgSize, BgType, Layer};

pub unsafe fn bg_init(layer: Layer, bg_type: BgType, bg_size: BgSize, map_base: i32, tile_base: i32) -> i32 {
    bgInit_call(layer as usize as i32, bg_type, bg_size, map_base, tile_base)
}

