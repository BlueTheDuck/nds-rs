use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct Attr0: u16 {
        const SHAPE_SQUARE = 0b00_000000_00000000;
        const SHAPE_WIDE = 0b01_000000_00000000;
        const SHAPE_TALL = 0b10_000000_00000000;
        // 0b11_000000_00000000: Forbidden

        /// Set for 256 colors, unset for 16
        const COLOR_256 = bit!(13);

        const NORMAL = 0b00 << 10;
        const TRANSLUCENT = 0b01 << 10;
        const WINDOW = 0b10 << 10;
        const BITMAP = 0b11 << 10;

        /// Only when [AFFINE_ENABLE](Attr0::AFFINE_ENABLE) is not set, controls whether the sprite is displayed
        const HIDDEN = bit!(9);
        /// Double the canvas size for the sprite (To prevent clipping)
        const DOUBLE_SIZE = bit!(9);
        /// Set to enable affine tranformation
        const AFFINE_ENABLE = bit!(8);


    }
}
bitflags! {
    #[derive(Copy, Clone)]
    pub struct Attr1: u16 {
        /// Small axis: 8 pixels / Large axis: 16 pixels. Square: 8 pixels
        const SIZE_SMALL = 0b00_000000_00000000;
        /// Small axis: 8 pixels / Large axis: 32 pixels. Square 16 pixels
        const SIZE_MED = 0b01_000000_00000000;
        /// Small axis: 16 pixels / Large axis: 32 pixels. Square 32 pixels
        const SIZE_BIG = 0b10_000000_00000000;
        /// Small axis: 32 pixels / Large axis: 64 pixels. Square 64 pixels
        const SIZE_MAX = 0b11_000000_00000000;

        // TODO: The rest
    }
}

/// X pos field in [Attr1] is 9 bits long.
pub const X_COORD_MASK: u16 = 0b1_11111111;
/// Y pos field in [Attr0] is 8 bits long.
pub const Y_COORD_MASK: u16 = 0b11111111;

/// ID of the tile is in Attr2
pub const ID_MASK: u16 = 0b11_11111111;

/// Indexed mode: Sets the palette
/// BMP mode: Sets transparency
pub const COLOR: u16 = 0b1111_0000_00000000;
