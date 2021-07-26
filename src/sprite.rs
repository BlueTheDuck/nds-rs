use bitflags::bitflags;

#[repr(C)]
pub struct ObjState {
    attr2: u16,
    attr1: u16,
    attr0: u16,
}

bitflags! {
    pub struct Attr0: u16 {
        const SHAPE_SQUARE = 0b00_000000_00000000;
        const SHAPE_WIDE = 0b01_000000_00000000;
        const SHAPE_TALL = 0b10_000000_00000000;
        // 0b11_000000_00000000: Forbidden
        
        const NORMAL = 0b0000_00_00_00000;
        const TRANSLUCENT = 0b0000_01_00_00000;
        const WINDOW = 0b0000_10_00_00000;
        const BITMAP = 0b0000_11_00_00000;

        // TODO: The rest
        /// Set for 256 colors, unset for 16
        const COLOR_256 = bit!(13);
    }
}
bitflags! {
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

/// X Coord is in Attr1
pub const X_COORD_MASK: u16 = 0b1_11111111;
/// Y Coord is in Attr0
pub const Y_COORD_MASK: u16 = 0b11111111;

/// ID of the tile is in Attr2
pub const ID_MASK: u16 = 0b11_11111111;

/// Indexed mode: Sets the palette
/// BMP mode: Sets transparency
pub const COLOR: u16 = 0b1111_0000_00000000;

