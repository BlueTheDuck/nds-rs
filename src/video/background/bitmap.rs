use nds_sys::background::{BgSize, BgType};

use super::{AsBgInitArgs, BgInitArgs};

pub enum BitmapLayerSize {
    /// 128x128
    Small,
    /// 256x256
    Medium,
    /// 512x256
    Wide,
    /// 512x512
    Big,
}

/// A layer that uses a bitmap as its background.
/// Each pixel is 2 bytes and specifies a color directly.
/// Pixels use the Abgr1555 format.
pub struct DirectBitmapLayer {
    size: BitmapLayerSize,
    map_base: u8,
}

impl DirectBitmapLayer {
    /// Creates a new bitmap layer that uses direct color mode.
    ///
    /// `map_base` moves the start of the bitmap data in blocks of 16 KiB (`n * 0x4000`) from the start of VRAM, [BG_GFX](nds_sys::video::BG_GFX) or [BG_GFX_SUB](nds_sys::video::BG_GFX_SUB).
    ///
    /// Since the layer needs 32, 128, 256 or 512 KiB of VRAM depending on size, `map_base` can be at most 30, 24, 16 or 0 respectively.
    ///
    /// |         [`BitmapLayerSize`]         |  Size   | KiB | Max `map_base` |
    /// |-----------------------------------|---------|-----|----------------|
    /// | [Small](BitmapLayerSize::Small)   | 128x128 |  32 |             30 |
    /// | [Medium](BitmapLayerSize::Medium) | 256x256 | 128 |             24 |
    /// | [Wide](BitmapLayerSize::Wide)     | 512x256 | 256 |             16 |
    /// | [Big](BitmapLayerSize::Big)       | 512x512 | 512 |              0 |
    ///
    /// # Data overlapping
    ///
    /// No checks are done to prevent overlapping data, so it's possible to create a layer that shares data with another layer.
    ///
    pub fn new(size: BitmapLayerSize, map_base: u8) -> Option<Self> {
        match size {
            BitmapLayerSize::Small => {
                // 32 KiB
                if map_base > 30 {
                    return None;
                }
            }
            BitmapLayerSize::Medium => {
                // 128 KiB
                if map_base > 24 {
                    return None;
                }
            }
            BitmapLayerSize::Wide => {
                // 256 KiB
                if map_base > 16 {
                    return None;
                }
            }
            BitmapLayerSize::Big => {
                // 512 KiB
                if map_base > 0 {
                    return None;
                }
            }
        }
        Some(Self { size, map_base })
    }
}
impl AsBgInitArgs for DirectBitmapLayer {
    fn as_bg_init_args(self) -> BgInitArgs {
        let bg_size = match self.size {
            BitmapLayerSize::Small => BgSize::FullBitmapSmall,
            BitmapLayerSize::Medium => BgSize::FullBitmapMedium,
            BitmapLayerSize::Wide => BgSize::FullBitmapWide,
            BitmapLayerSize::Big => BgSize::FullBitmapBig,
        };
        let map_base = self.map_base as _;
        // Tile base is ignored for bitmap layers, but we need to pass something
        // libnds uses 0, so we do too
        let tile_base = 0;
        let bg_type = BgType::Bmp16;

        return (bg_type, bg_size, map_base, tile_base);
    }
}
