use crate::{bindings::{bgControl, bgInitSub_call, bgInit_call}, video::{BG_GFX, BG_GFX_SUB}};

pub mod registers {
    /// Control register for background 0 of Main Engine
    pub const BG0CNT: *mut u16 = 0x04000008 as _;
    /// Control register for background 1 of Main Engine
    pub const BG1CNT: *mut u16 = 0x0400000A as _;
    /// Control register for background 2 of Main Engine
    pub const BG2CNT: *mut u16 = 0 as _;
    /// Control register for background 3 of Main Engine
    pub const BG3CNT: *mut u16 = 0 as _;
    /// Control register for background 0 of Sub Engine
    pub const DB_BG0CNT: *mut u16 = 0 as _;
    /// Control register for background 1 of Sub Engine
    pub const DB_BG1CNT: *mut u16 = 0 as _;
    /// Control register for background 2 of Sub Engine
    pub const DB_BG2CNT: *mut u16 = 0 as _;
    /// Control register for background 3 of Sub Engine
    pub const DB_BG3CNT: *mut u16 = 0 as _;
    
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

bitflags! {
    pub struct Flags: u16 {
        const SCREENSIZE_512_512 = 3 << 14;
        const SCREENSIZE_256_512 = 2 << 14;
        const SCREENSIZE_512_256 = 1 << 14;
        const SCREENSIZE_256_256 = 0 << 14;
        const ALTERNATIVE_EXT_PALETTE = bit!(13);
        /// These bits multiplied by 0x800 and added [BG_SCREEN_BASE_MASK](super::video::Flags::BG_SCREEN_BASE_MASK) give the address where screen info is stored for this layer
        const SCREEN_OFFSET_MASK = 0b11111 << 8;
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

#[repr(C)]
pub enum BgSize {
    /// 128 x 128 pixel rotation background
    BgSize_R_128x128 = (0 << 14),
    /// 256 x 256 pixel rotation background
    BgSize_R_256x256 = (1 << 14),
    /// 512 x 512 pixel rotation background
    BgSize_R_512x512 = (2 << 14),
    /// 1024 x 1024 pixel rotation background
    BgSize_R_1024x1024 = (3 << 14),

    /// 256 x 256 pixel text background
    BgSize_T_256x256 = (0 << 14) | (1 << 16),
    /// 512 x 256 pixel text background
    BgSize_T_512x256 = (1 << 14) | (1 << 16),
    /// 256 x 512 pixel text background
    BgSize_T_256x512 = (2 << 14) | (1 << 16),
    /// 512 x 512 pixel text background
    BgSize_T_512x512 = (3 << 14) | (1 << 16),

    /// 128 x 128 pixel extended rotation background
    BgSize_ER_128x128 = (0 << 14) | (2 << 16),
    /// 256 x 256 pixel extended rotation background
    BgSize_ER_256x256 = (1 << 14) | (2 << 16),
    /// 512 x 512 pixel extended rotation background
    BgSize_ER_512x512 = (2 << 14) | (2 << 16),
    /// 1024 x 1024 extended pixel rotation background
    BgSize_ER_1024x1024 = (3 << 14) | (2 << 16),

    /// 128 x 128 pixel 8 bit bitmap background
    BgSize_B8_128x128 = ((0 << 14) | bit!(7) | (3 << 16)),
    /// 256 x 256 pixel 8 bit bitmap background
    BgSize_B8_256x256 = ((1 << 14) | bit!(7) | (3 << 16)),
    /// 512 x 256 pixel 8 bit bitmap background
    BgSize_B8_512x256 = ((2 << 14) | bit!(7) | (3 << 16)),
    /// 512 x 512 pixel 8 bit bitmap background
    BgSize_B8_512x512 = ((3 << 14) | bit!(7) | (3 << 16)),
    /// 1024 x 512 pixel 8 bit bitmap background
    BgSize_B8_1024x512 = (1 << 14) | (3 << 16),
    /// 512 x 1024 pixel 8 bit bitmap background
    BgSize_B8_512x1024 = (0) | (3 << 16),

    /// 128 x 128 pixel 16 bit bitmap background
    BgSize_B16_128x128 = ((0 << 14) | bit!(7) | bit!(2) | (4 << 16)),
    /// 256 x 256 pixel 16 bit bitmap background
    BgSize_B16_256x256 = ((1 << 14) | bit!(7) | bit!(2) | (4 << 16)),
    /// 512 x 512 pixel 16 bit bitmap background
    BgSize_B16_512x256 = ((2 << 14) | bit!(7) | bit!(2) | (4 << 16)),
    /// 1024 x 1024 pixel 16 bit bitmap background
    BgSize_B16_512x512 = ((3 << 14) | bit!(7) | bit!(2) | (4 << 16)),
}

#[repr(C)]
pub enum Layer {
    Layer0 = 0,
    Layer1 = 1,
    Layer2 = 2,
    Layer3 = 3,
}

pub unsafe fn bg_get_tile_base(id: usize) -> usize {
    let cnt = bgControl[id].read_volatile() >> 2;
    return (cnt & 15) as usize;
}

pub unsafe fn bg_get_map_base(id: usize) -> usize {
    let cnt = bgControl[id].read_volatile() >> 8;
    return (cnt & 0xFF) as usize;
}

pub unsafe fn bg_tile_ram(base: usize) -> *mut u16 {
    BG_GFX.add(base * 0x4000) as *mut u16
}

pub unsafe fn bg_get_gfx_ptr(id: usize) -> *mut u16 {
    use super::bindings::bgState;
    match bgState[id].type_ {
        1 | 2 | 3 => {
            if id < 4 {
                bg_tile_ram(bg_get_tile_base(id))
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
            unreachable!("Background type is not any of: enum BgType {Text8, Text4, Rotation, ExRotation, Bmp8, Bmp16}");
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
    map_base: usize,
    tile_base: usize,
) -> usize {
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
        layer as usize as i32,
        bg_type as usize as u32,
        bg_size as usize as u32,
        map_base as i32,
        tile_base as i32,
    ) as usize
}
