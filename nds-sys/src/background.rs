use crate::{
    bindings::{bgControl, bgInitSub_call, bgInit_call},
    video::{BG_GFX, BG_GFX_SUB},
};

pub mod registers {
    /// Control register for background 0 of Main Engine
    pub const BG0CNT: *mut u16 = 0x04000008 as _;
    /// Control register for background 1 of Main Engine
    pub const BG1CNT: *mut u16 = 0x0400000A as _;
    /// Control register for background 2 of Main Engine
    pub const BG2CNT: *mut u16 = 0x0400000C as _;
    /// Control register for background 3 of Main Engine
    pub const BG3CNT: *mut u16 = 0x0400000E as _;
    /// Control register for background 0 of Sub Engine
    pub const DB_BG0CNT: *mut u16 = 0x04001008 as _;
    /// Control register for background 1 of Sub Engine
    pub const DB_BG1CNT: *mut u16 = 0x0400100A as _;
    /// Control register for background 2 of Sub Engine
    pub const DB_BG2CNT: *mut u16 = 0x0400100C as _;
    /// Control register for background 3 of Sub Engine
    pub const DB_BG3CNT: *mut u16 = 0x0400100E as _;

    /// Affine transformation only. Register for background 2 of Main Engine. Controls x0 (Displacement)
    pub const BG2X: *mut u32 = 0x04000028 as _;
    /// Affine transformation only. Register for background 2 of Main Engine. Controls y0 (Displacement)
    pub const BG2Y: *mut u32 = 0x0400002C as _;

    pub const BG3PA: *mut i16 = 0x04000030 as _;
    pub const BG3PB: *mut i16 = 0x04000032 as _;
    pub const BG3PC: *mut i16 = 0x04000034 as _;
    pub const BG3PD: *mut i16 = 0x04000036 as _;
    /// Affine transformation only. Register for background 3 of Main Engine. Controls x0 (Displacement)
    pub const BG3X: *mut u32 = 0x04000038 as _;
    /// Affine transformation only. Register for background 3 of Main Engine. Controls y0 (Displacement)
    pub const BG3Y: *mut u32 = 0x0400003C as _;

    /// Affine transformation only. Register for background 2 of Sub Engine. Controls x0 (Displacement)
    pub const DB_BG2X: *mut u32 = 0x04001028 as _;
    /// Affine transformation only. Register for background 2 of Sub Engine. Controls y0 (Displacement)
    pub const DB_BG2Y: *mut u32 = 0x0400102C as _;
    /// Affine transformation only. Register for background 3 of Sub Engine. Controls x0 (Displacement)
    pub const DB_BG3X: *mut u32 = 0x04001038 as _;
    /// Affine transformation only. Register for background 3 of Sub Engine. Controls y0 (Displacement)
    pub const DB_BG3Y: *mut u32 = 0x0400103C as _;
}

// TODO: SCREENSIZE_* are wrongly named, not every type of BG interprets these bits the same
bitflags! {
    pub struct Flags: u16 {
        const SCREENSIZE_512_512 = 3 << 14;
        const SCREENSIZE_256_512 = 2 << 14;
        const SCREENSIZE_512_256 = 1 << 14;
        const SCREENSIZE_256_256 = 0 << 14;
        const ALTERNATIVE_EXT_PALETTE = bit!(13);
        /// Set to use 256-color mode, unset for 16-color mode
        const FULLCOLOR = bit!(7);
        /// Set to enable mosaic processing. See [DB_MOSAIC]
        const MOSAIC = bit!(6);
        const LOWEST_PRIORITY = 3;
        const LOW_PRIORITY = 2;
        const HIGH_PRIORITY = 1;
        const HIGHEST_PRIORITY = 0;
    }
}

/// These bits multiplied by 0x800 and added [BG_SCREEN_BASE_MASK](super::video::Flags::BG_SCREEN_BASE_MASK) give the address where screen info is stored for this layer
pub const SCREEN_OFFSET_MASK: u16 = 0b11111 << 8;
pub const SCREEN_BASE_OFFSET: u16 = 8;
pub const CHARACTER_BASE_OFFSET: u16 = 2;

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

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum Layer {
    Layer0 = 0,
    Layer1 = 1,
    Layer2 = 2,
    Layer3 = 3,
}
impl From<usize> for Layer {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Layer0,
            1 => Self::Layer1,
            2 => Self::Layer2,
            3 => Self::Layer3,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
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
    pub const fn is_main(&self) -> bool {
        match self {
            Self::MainBg0 | Self::MainBg1 | Self::MainBg2 | Self::MainBg3 => true,
            _ => false,
        }
    }
    pub fn get_layer(self) -> Layer {
        match self {
            BackgroundId::MainBg0 | BackgroundId::SubBg0 => Layer::Layer0,
            BackgroundId::MainBg1 | BackgroundId::SubBg1 => Layer::Layer1,
            BackgroundId::MainBg2 | BackgroundId::SubBg2 => Layer::Layer2,
            BackgroundId::MainBg3 | BackgroundId::SubBg3 => Layer::Layer3,
        }
    }
}
impl TryFrom<usize> for BackgroundId {
    type Error = &'static str;

    fn try_from(id: usize) -> Result<Self, Self::Error> {
        match id {
            0 => Ok(BackgroundId::MainBg0),
            1 => Ok(BackgroundId::MainBg1),
            2 => Ok(BackgroundId::MainBg2),
            3 => Ok(BackgroundId::MainBg3),
            4 => Ok(BackgroundId::SubBg0),
            5 => Ok(BackgroundId::SubBg1),
            6 => Ok(BackgroundId::SubBg2),
            7 => Ok(BackgroundId::SubBg3),
            _ => Err("Tried to convert unknown value to background ID"),
        }
    }
}
impl core::marker::ConstParamTy for BackgroundId {
    
}

pub unsafe fn bg_get_tile_base(id: usize) -> usize {
    let cnt = bgControl[id].read_volatile() >> 2;
    return (cnt & 15) as usize;
}

pub unsafe fn bg_get_map_base(id: usize) -> usize {
    let cnt = bgControl[id].read_volatile() >> 8;
    return (cnt & 0xFF) as usize;
}

pub unsafe fn bg_get_gfx_ptr(id: usize) -> *mut u16 {
    use super::bindings::bgState;
    match bgState[id].type_ {
        0 | 1 | 2 | 3 => {
            if id < 4 {
                BG_GFX.add(bg_get_tile_base(id) * 0x4000) as *mut u16
            } else {
                //((u16*)BG_TILE_RAM_SUB(bgGetTileBase(id)))
                unimplemented!("Graphics for SUB are not yet implemented")
            }
        }
        4 | 5 => {
            if id < 4 {
                BG_GFX.add(0x2000 * bg_get_map_base(id))
            } else {
                BG_GFX_SUB.add(0x2000 * bg_get_map_base(id))
            }
        }
        _ => {
            unreachable!("{}", "Background type is not any of: enum BgType {Text8, Text4, Rotation, ExRotation, Bmp8, Bmp16}");
        }
    }
}
// u16* bgGetGfxPtr(int id)
// {
// 	if(bgState[id].type < BgType_Bmp8)
// 		return (id < 4) ? (u16*)(BG_TILE_RAM(bgGetTileBase(id))) : ((u16*)BG_TILE_RAM_SUB(bgGetTileBase(id)));
// 	else
// 		return (id < 4) ? (u16*)(BG_GFX + 0x2000 * (bgGetMapBase(id))) : (u16*)(BG_GFX_SUB + 0x2000 * (bgGetMapBase(id)));
// }

pub unsafe fn bg_init(
    layer: Layer,
    bg_type: BgType,
    bg_size: BgSize,
    map_base: u8,
    tile_base: u8,
) -> usize {
    debug_assert!(tile_base <= 15);
    debug_assert!(map_base <= 31);
    if layer == Layer::Layer0 {
        debug_assert!(
            crate::video::video_3d_enabled() == false,
            "Background 0 is currently in use by the 3D engine"
        );
    }

    // For backgrounds 0 and 1 only Text8bpp and Text4bpp are valid types.
    if layer == Layer::Layer0 || layer == Layer::Layer1 {
        debug_assert!(
            bg_type.is_text(),
            "Background 0 and 1 can only be Text8 or Text4"
        );
    }

    if bg_type.is_bitmap() {
        debug_assert_eq!(tile_base, 0, "Tile base is unused for bitmaps");
    }

    bgInit_call(
        layer as i32,
        bg_type as u32,
        bg_size as u32,
        map_base as i32,
        tile_base as i32,
    ) as usize
}

pub unsafe fn bg_init_sub(
    layer: Layer,
    bg_type: BgType,
    bg_size: BgSize,
    map_base: usize,
    tile_base: usize,
) -> usize {
    bgInitSub_call(
        layer as i32,
        bg_type as u32,
        bg_size as u32,
        map_base as i32,
        tile_base as i32,
    ) as usize
}
