pub mod registers;
pub mod affine;

///  Register overlay for scroll registers
#[repr(C)]
pub struct BgScroll {
    /// < X scroll
    pub x: u16,
    /// < Y scroll
    pub y: u16,
}

///  Register overlay for affine matrix registers
#[repr(C)]
pub struct BgTransform {
    /// < The change in x per horizontal pixel
    pub hdx: i16,
    /// < The change in x per vertical pixel
    pub vdx: i16,
    /// < The change in y per horizontal pixel
    pub hdy: i16,
    /// < The change in x per vertical pixel
    pub vdy: i16,
    /// < Map x value which corresponds to the screen origin
    pub dx: i32,
    /// < Map y value which corresponds to the screen origin
    pub dy: i32,
}

///  Register overlay for background attribute registers.
#[repr(C)]
pub struct BgAttribute {
    /// < Background control registers
    pub control: [u16; 4usize],
    /// < Background scroll registers
    pub scroll: [BgScroll; 4usize],
    /// < Background 2 affine matrix
    pub bg2_rotation: BgTransform,
    /// < Background 3 affine matrix
    pub bg3_rotation: BgTransform,
}

/// Overlay for 8-bit tile map entries
#[repr(C)]
pub struct TileMapEntry8 {
    pub index: u8,
}

/// Overlay for 16-bit tile map entries
///
/// `PPPPVHII_IIIIIIII`
#[repr(C, packed)]
#[derive(Default, Clone, Copy)]
pub struct TileMapEntry16 {
    data: u16,
}
impl TileMapEntry16 {
    const INDEX_MASK: u16   = 0b00000011_11111111;
    const HFLIP: u16        = 0b00000100_00000000;
    const VFLIP: u16        = 0b00001000_00000000;
    const PALETTE_MASK: u16 = 0b11110000_00000000;

    pub const fn new() -> Self {
        Self { data: 0 }
    }

    pub fn set_index(&mut self, index: u16) {
        self.data = (self.data & !Self::INDEX_MASK) | (index & Self::INDEX_MASK);
    }
    pub fn index(&self) -> u16 {
        self.data & Self::INDEX_MASK
    }

    pub fn set_palette(&mut self, palette: u16) {
        self.data = (self.data & !Self::PALETTE_MASK) | (palette & Self::PALETTE_MASK);
    }
    pub fn palette(&self) -> u16 {
        self.data & Self::PALETTE_MASK
    }

    pub fn set_hflip(&mut self, hflip: bool) {
        if hflip {
            self.data |= Self::HFLIP;
        } else {
            self.data &= !Self::HFLIP;
        }
    }
    pub fn hflip(&self) -> bool {
        self.data & Self::HFLIP != 0
    }
    
    pub fn set_vflip(&mut self, vflip: bool) {
        if vflip {
            self.data |= Self::VFLIP;
        } else {
            self.data &= !Self::VFLIP;
        }
    }
    pub fn vflip(&self) -> bool {
        self.data & Self::VFLIP != 0
    }
}

/// Bit defines for the background control registers
pub mod BackgroundControlConstants {
    #![allow(non_upper_case_globals)]
    #![allow(non_snake_case)]

    use super::BackgroundControl;

    /// < 32 x 32 tile text background
    pub const BG_32x32: BackgroundControl = BackgroundControl::from_bits_retain(0);
    /// < 64 x 32 tile text background
    pub const BG_64x32: BackgroundControl = BackgroundControl::from_bits_retain(16384);
    /// < 32 x 64 tile text background
    pub const BG_32x64: BackgroundControl = BackgroundControl::from_bits_retain(32768);
    /// < 64 x 64 tile text background
    pub const BG_64x64: BackgroundControl = BackgroundControl::from_bits_retain(49152);

    /// < 16 x 16 tile affine (rotation & scale) background
    pub const BG_RS_16x16: BackgroundControl = BackgroundControl::from_bits_retain(0);
    /// < 32 x 32 tile affine (rotation & scale) background
    pub const BG_RS_32x32: BackgroundControl = BackgroundControl::from_bits_retain(16384);
    /// < 64 x 64 tile affine (rotation & scale) background
    pub const BG_RS_64x64: BackgroundControl = BackgroundControl::from_bits_retain(32768);
    /// < 128 x 128 tile affine (rotation & scale) background
    pub const BG_RS_128x128: BackgroundControl = BackgroundControl::from_bits_retain(49152);

    /// < 128x128 pixel 8-bit bitmap
    pub const BG_BMP8_128x128: BackgroundControl = BackgroundControl::from_bits_retain(128);
    /// < 256x256 pixel 8-bit bitmap
    pub const BG_BMP8_256x256: BackgroundControl = BackgroundControl::from_bits_retain(16512);
    /// < 512x256 pixel 8-bit bitmap
    pub const BG_BMP8_512x256: BackgroundControl = BackgroundControl::from_bits_retain(32896);
    /// < 512 pixel 8-bit bitma
    pub const BG_BMP8_512x512: BackgroundControl = BackgroundControl::from_bits_retain(49280);
    /// < 1024x512 pixel 8-bit Large bitmap (Mode 6, main engine)
    pub const BG_BMP8_1024x512: BackgroundControl = BackgroundControl::from_bits_retain(16384);
    /// < 512x1024 pixel 8-bit Large bitmap (Mode 6, main engine)
    pub const BG_BMP8_512x1024: BackgroundControl = BackgroundControl::from_bits_retain(0);

    /// < 128x128 pixel 16-bit bitmap
    pub const BG_BMP16_128x128: BackgroundControl = BackgroundControl::from_bits_retain(132);
    /// < 256x256 pixel 16-bit bitmap
    pub const BG_BMP16_256x256: BackgroundControl = BackgroundControl::from_bits_retain(16516);
    /// < 512x256 pixel 16-bit bitmap
    pub const BG_BMP16_512x256: BackgroundControl = BackgroundControl::from_bits_retain(32900);
    /// < 512x512 pixel 16-bit bitmap
    pub const BG_BMP16_512x512: BackgroundControl = BackgroundControl::from_bits_retain(49284);

    /// < Mosaic enable
    pub const BG_MOSAIC_ON: BackgroundControl = BackgroundControl::from_bits_retain(64);
    /// < Mosaic disable
    pub const BG_MOSAIC_OFF: BackgroundControl = BackgroundControl::from_bits_retain(0);

    /// < Lower priority will be rendered on top
    pub const BG_PRIORITY_0: BackgroundControl = BackgroundControl::from_bits_retain(0);
    /// < Lower priority will be rendered on top
    pub const BG_PRIORITY_1: BackgroundControl = BackgroundControl::from_bits_retain(1);
    /// < Lower priority will be rendered on top
    pub const BG_PRIORITY_2: BackgroundControl = BackgroundControl::from_bits_retain(2);
    /// < Lower priority will be rendered on top
    pub const BG_PRIORITY_3: BackgroundControl = BackgroundControl::from_bits_retain(3);

    /// < Disable wrapping (text backgrounds always wrap)
    pub const BG_WRAP_OFF: BackgroundControl = BackgroundControl::from_bits_retain(0);
    /// < Enable wrapping (text backgrounds always wrap)
    pub const BG_WRAP_ON: BackgroundControl = BackgroundControl::from_bits_retain(8192);

    /// < Use slot 0 of extended palettes
    pub const BG_PALETTE_SLOT0: BackgroundControl = BackgroundControl::from_bits_retain(0);
    /// < Use slot 1 of extended palettes
    pub const BG_PALETTE_SLOT1: BackgroundControl = BackgroundControl::from_bits_retain(0);
    /// < Use slot 2 of extended palettes
    pub const BG_PALETTE_SLOT2: BackgroundControl = BackgroundControl::from_bits_retain(8192);
    /// < Use slot 3 of extended palettes
    pub const BG_PALETTE_SLOT3: BackgroundControl = BackgroundControl::from_bits_retain(8192);

    /// < 256 color text background
    pub const BG_COLOR_256: BackgroundControl = BackgroundControl::from_bits_retain(128);
    /// < 16x16 color text background
    pub const BG_COLOR_16: BackgroundControl = BackgroundControl::from_bits_retain(0);
}

/// libnds' internal representation of a background.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BgState {
    pub angle: core::ffi::c_int,
    pub center_x: i32,
    pub center_y: i32,
    pub scale_x: i32,
    pub scale_y: i32,
    pub scroll_x: i32,
    pub scroll_y: i32,
    /// Unused
    pub size: BgSize,
    pub r#type: BgType,
    pub dirty: bool,
}
impl BgState {
    #[inline]
    pub fn copy_from(&mut self, other: &Self) {
        *self = *other;
        self.dirty = true;
    }
}
impl Default for BgState {
    fn default() -> Self {
        // not really safe tho
        unsafe { core::mem::zeroed() }
    }
}

extern "C" {
    pub static mut bgControl: [*mut u16; 8];
    pub static mut bgScrollTable: [*mut BgScroll; 8];
    pub static mut bgTransform: [*mut BgTransform; 8];
    pub static mut bgState: [BgState; 8];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum BgType {
    /// 8 bit per pixel tiled background with 16 bit tile indexes. No rotation nor scaling
    Text8 = 0,
    /// 4 bit per pixel tiled background with 16 bit tile indexes. No rotation nor scaling
    Text4 = 1,
    /// Tiled background with 8 bit tile indexes. Rotable and scalable
    Rotation = 2,
    /// Tiled background with 16 bit tile indexes. Rotable and scalable
    ExRotation = 3,
    /// Bitmap background with 8 bit indexes for a 256 color palette
    Bmp8 = 4,
    /// Bitmap background with 16 bit ABGR1555 colors
    Bmp16 = 5,
}
impl BgType {
    #[inline]
    pub fn is_text(self) -> bool {
        matches!(self, Self::Text4 | Self::Text8)
    }

    #[inline]
    pub fn is_bitmap(self) -> bool {
        matches!(self, Self::Bmp8 | Self::Bmp16)
    }
}
impl<N: Into<usize>> From<N> for BgType {
    fn from(ty: N) -> Self {
        match ty.into() {
            0 => Self::Text8,
            1 => Self::Text4,
            2 => Self::Rotation,
            3 => Self::ExRotation,
            4 => Self::Bmp8,
            5 => Self::Bmp16,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum BgSize {
    /// 128 x 128 pixel rotation background
    RotationSmall = (0 << 14),
    /// 256 x 256 pixel rotation background
    RotationMedium = (1 << 14),
    /// 512 x 512 pixel rotation background
    RotationBig = (2 << 14),
    /// 1024 x 1024 pixel rotation background
    RotationLarge = (3 << 14),

    /// 256 x 256 pixel text background
    TextSmall = (0 << 14) | (1 << 16),
    /// 512 x 256 pixel text background
    TextWide = (1 << 14) | (1 << 16),
    /// 256 x 512 pixel text background
    TextTall = (2 << 14) | (1 << 16),
    /// 512 x 512 pixel text background
    TextBig = (3 << 14) | (1 << 16),

    /// 128 x 128 pixel extended rotation background
    ExRotSmall = (0 << 14) | (2 << 16),
    /// 256 x 256 pixel extended rotation background
    ExRotMedium = (1 << 14) | (2 << 16),
    /// 512 x 512 pixel extended rotation background
    ExRotBig = (2 << 14) | (2 << 16),
    /// 1024 x 1024 extended pixel rotation background
    ExRotLarge = (3 << 14) | (2 << 16),

    // 256 color bitmap
    /// 128 x 128 pixel 8 bit bitmap background
    BitmapSmall = ((0 << 14) | bit!(7) | (3 << 16)),
    /// 256 x 256 pixel 8 bit bitmap background
    BitmapMedium = ((1 << 14) | bit!(7) | (3 << 16)),
    /// 512 x 256 pixel 8 bit bitmap background
    BitmapWide = ((2 << 14) | bit!(7) | (3 << 16)),
    /// 512 x 512 pixel 8 bit bitmap background
    BitmapBig = ((3 << 14) | bit!(7) | (3 << 16)),

    /// 1024 x 512 pixel 8 bit bitmap background
    LargeBitmapWide = (1 << 14) | (3 << 16),
    /// 512 x 1024 pixel 8 bit bitmap background
    LargeBitmapTall = (0) | (3 << 16),

    // Direct color bitmap bg
    /// 128 x 128 pixel 16 bit bitmap background
    FullBitmapSmall = ((0 << 14) | bit!(7) | bit!(2) | (4 << 16)),
    /// 256 x 256 pixel 16 bit bitmap background
    FullBitmapMedium = ((1 << 14) | bit!(7) | bit!(2) | (4 << 16)),
    /// 512 x 256 pixel 16 bit bitmap background
    FullBitmapWide = ((2 << 14) | bit!(7) | bit!(2) | (4 << 16)),
    /// 512 x 512 pixel 16 bit bitmap background
    FullBitmapBig = ((3 << 14) | bit!(7) | bit!(2) | (4 << 16)),
}

extern "C" {
    #[link_name = "bgIsText"]
    fn is_text(id: ::core::ffi::c_int) -> bool;

    #[link_name = "bgInit_call"]
    fn init_call(
        layer: ::core::ffi::c_int,
        type_: BgType,
        size: BgSize,
        mapBase: ::core::ffi::c_int,
        tileBase: ::core::ffi::c_int,
    ) -> BackgroundId;

    #[link_name = "bgInitSub_call"]
    fn init_sub_call(
        layer: ::core::ffi::c_int,
        type_: BgType,
        size: BgSize,
        mapBase: ::core::ffi::c_int,
        tileBase: ::core::ffi::c_int,
    ) -> BackgroundId;

    ///  Must be called once per frame to update scroll/scale/and rotation of backgrounds.
    #[link_name = "bgUpdate"]
    pub fn update();
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct BackgroundControl: u16 {
        const SCREENSIZE_MASK         = 0b11000000_00000000;
        const ALTERNATIVE_EXT_PALETTE = 0b00100000_00000000;
        const MAP_MASK                = 0b00011111_00000000;
        const BITMAP                  = 0b00000000_10000100;
        /// Set to use 256-color mode, unset for 16-color mode
        const FULLCOLOR               = 0b00000000_10000000;
        /// Set to enable mosaic processing. See [DB_MOSAIC]
        const MOSAIC                  = 0b00000000_01000000;
        const TILES_MASK              = 0b00000000_00111100;
        const PRIORITY_MASK           = 0b00000000_00000011;
    }
}
impl BackgroundControl {
    // TODO: Find a better way to generate getters/setters

    pub const fn with_priority(self, priority: u16) -> Self {
        let this = self.bits() & !Self::PRIORITY_MASK.bits();
        let priority = priority & Self::PRIORITY_MASK.bits();
        Self::from_bits_retain(this | priority)
    }
    pub const fn priority(&self) -> u16 {
        self.bits() & Self::PRIORITY_MASK.bits()
    }

    pub const fn with_size(self, size: BgSize) -> Self {
        let size = size as u16 & Self::SCREENSIZE_MASK.bits();
        self.difference(Self::SCREENSIZE_MASK)
            .union(Self::from_bits_retain(size))
    }
    pub const fn size_value(self) -> u16 {
        self.intersection(Self::SCREENSIZE_MASK).bits() >> 14
    }

    pub const fn set_tile_base(&mut self, tile_base: u16) {
        let this = self.bits() & !Self::TILES_MASK.bits();
        let tile_base = (tile_base << 2) & Self::TILES_MASK.bits();
        *self = Self::from_bits_retain(this | tile_base);
    }
    pub const fn tile_base(self) -> u16 {
        (self.bits() & Self::TILES_MASK.bits()) >> 2
    }

    pub const fn set_map_base(&mut self, map_base: u16) {
        let this = self.bits() & !Self::MAP_MASK.bits();
        let map_base = (map_base << 8) & Self::MAP_MASK.bits();
        *self = Self::from_bits_retain(this | map_base);
    }
    pub const fn map_base(&self) -> u16 {
        (self.bits() & Self::MAP_MASK.bits()) >> 8
    }

    pub const fn with_screen_base_block(self, block: u16) -> Self {
        todo!()
    }
}

/// These bits multiplied by 0x800 and added [BG_SCREEN_BASE_MASK](super::video::Flags::BG_SCREEN_BASE_MASK) give the address where screen info is stored for this layer
pub const SCREEN_OFFSET_MASK: u16 = 0b11111 << 8;
pub const SCREEN_BASE_OFFSET: u16 = 8;
pub const CHARACTER_BASE_OFFSET: u16 = 2;

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum Layer {
    Layer0 = 0,
    Layer1 = 1,
    Layer2 = 2,
    Layer3 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
#[repr(i32)]
pub enum BackgroundId {
    MainBg0 = 0,
    MainBg1 = 1,
    MainBg2 = 2,
    MainBg3 = 3,
    SubBg0 = 4,
    SubBg1 = 5,
    SubBg2 = 6,
    SubBg3 = 7,
}
impl BackgroundId {
    pub const fn is_main(self) -> bool {
        matches!(
            self,
            Self::MainBg0 | Self::MainBg1 | Self::MainBg2 | Self::MainBg3
        )
    }

    pub const fn get_layer(self) -> Layer {
        match self {
            BackgroundId::MainBg0 | BackgroundId::SubBg0 => Layer::Layer0,
            BackgroundId::MainBg1 | BackgroundId::SubBg1 => Layer::Layer1,
            BackgroundId::MainBg2 | BackgroundId::SubBg2 => Layer::Layer2,
            BackgroundId::MainBg3 | BackgroundId::SubBg3 => Layer::Layer3,
        }
    }
}

/// Returns the base address of the tile data for the given background.
/// This value is garbage for bitmap backgrounds.
pub unsafe fn tile_base(_: BackgroundId) -> usize {
    todo!()
}

/// Returns the base address of the map data for the given background.
/// For bitmap backgrounds, this value is the base address of the bitmap data.
pub unsafe fn map_base(_: BackgroundId) -> usize {
    todo!()
}

pub unsafe fn gfx_ptr(_: BackgroundId) -> *mut u16 {
    todo!()
}

pub fn init(
    layer: Layer,
    type_: BgType,
    size: BgSize,
    map_base: u8,
    tile_base: u8,
) -> BackgroundId {
    debug_assert!(tile_base <= 15);
    debug_assert!(map_base <= 31);
    if layer == Layer::Layer0 {
        debug_assert!(
            !crate::video::video_3d_enabled(),
            "Background 0 is currently in use by the 3D engine"
        );
    }

    // For backgrounds 0 and 1 only Text8bpp and Text4bpp are valid types.
    if layer == Layer::Layer0 || layer == Layer::Layer1 {
        debug_assert!(
            type_.is_text(),
            "Background 0 and 1 can only be Text8 or Text4"
        );
    }

    if type_.is_bitmap() {
        debug_assert_eq!(
            tile_base, 0,
            "Tile base is unused for bitmaps. Please set it to 0"
        );
    }

    unsafe { init_call(layer as i32, type_, size, map_base as i32, tile_base as i32) }
}

#[doc(hidden)]
#[allow(warnings)]
pub unsafe fn init_sub(
    layer: Layer,
    type_: BgType,
    size: BgSize,
    map_base: u8,
    tile_base: u8,
) -> BackgroundId {
    debug_assert!(tile_base <= 15);
    debug_assert!(map_base <= 31);

    // For backgrounds 0 and 1 only Text8bpp and Text4bpp are valid types.
    if layer == Layer::Layer0 || layer == Layer::Layer1 {
        debug_assert!(
            type_.is_text(),
            "Background 0 and 1 can only be Text8 or Text4"
        );
    }

    if type_.is_bitmap() {
        debug_assert_eq!(
            tile_base, 0,
            "Tile base is unused for bitmaps. Please set it to 0"
        );
        debug_assert!(
            size == BgSize::LargeBitmapWide || size == BgSize::LargeBitmapTall,
            "Sub Display can not use large bitmaps"
        );
    }

    init_sub_call(layer as i32, type_, size, map_base as i32, tile_base as i32)
}
