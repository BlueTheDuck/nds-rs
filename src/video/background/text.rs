use nds_sys::background::{BgSize, BgType};

use super::{AsBgInitArgs, BgInitArgs};

pub enum TextLayerSize {
    Small,
    Wide,
    Tall,
    Big,
}

pub struct TextLayer {
    size: TextLayerSize,
    map_base: u8,
    tile_base: u8,
    // TODO: change this to an enum?
    /// `true` for 8bpp, `false` for 4bpp
    bits_per_pixel: bool,
}

impl AsBgInitArgs for TextLayer {
    fn as_bg_init_args(self) -> BgInitArgs {
        let bg_size = match self.size {
            TextLayerSize::Small => BgSize::TextSmall,
            TextLayerSize::Wide => BgSize::TextWide,
            TextLayerSize::Tall => BgSize::TextTall,
            TextLayerSize::Big => BgSize::TextBig,
        };
        let map_base = self.map_base as _;
        let tile_base = self.tile_base as _;
        let bg_type = if self.bits_per_pixel {
            BgType::Text8
        } else {
            BgType::Text4
        };

        return (bg_type, bg_size, map_base, tile_base);
    }
}
