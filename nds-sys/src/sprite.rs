use bitfield_struct::{bitenum, bitfield};
use static_assertions::assert_eq_size;

#[bitenum]
#[repr(i32)]
pub enum BlendMode {
    #[fallback]
    Normal = 0,
    Translucent = 1,
    Window = 2,
    Bitmap = 3,
}

#[bitenum]
#[repr(i32)]
pub enum ObjShape {
    #[fallback]
    Square = 0,
    Wide = 1,
    Tall = 2,
}

#[bitenum]
#[repr(i32)]
/// |   Shape    | Smallest | Small | Large | Largest |
/// |------------|----------|-------|-------|---------|
/// | Square     | 8x8      | 16x16 | 32x32 | 64x64   |
/// | Wide rect. | 16x8     | 32x8  | 32x16 | 64x32   |
/// | Tall rect. | 8x16     | 8x32  | 16x32 | 32x64   |
///  
pub enum ObjSize {
    #[fallback]
    /// Small axis: 8 pixels / Large axis: 16 pixels. Square: 8 pixels
    Smallest = 0,

    /// Small axis: 8 pixels / Large axis: 32 pixels. Square 16 pixels
    Small = 1,

    /// Small axis: 16 pixels / Large axis: 32 pixels. Square 32 pixels
    Large = 2,

    /// Small axis: 32 pixels / Large axis: 64 pixels. Square 64 pixels
    Largest = 3,
}

#[bitfield(u64, debug = false)]
pub struct SpriteEntry {
    #[bits(8)]
    pub y: u32,
    pub is_affine: bool,
    pub double_or_hidden: bool,
    #[bits(2)]
    pub blend_mode: BlendMode,
    pub mosaic: bool,
    pub color: bool,
    #[bits(2)]
    pub shape: ObjShape,
    #[bits(9)]
    pub x: u32,
    #[bits(5)]
    affine_index_or_flips: usize,
    #[bits(2)]
    pub size: ObjSize,
    #[bits(10)]
    pub name: u32,
    #[bits(2)]
    pub priority: usize,
    #[bits(4)]
    pub color_parameter: u32,
    __: u16,
}
impl SpriteEntry {
    pub const fn affine_index(&self) -> usize {
        self.affine_index_or_flips()
    }
    pub fn with_affine_index(mut self, index: usize) -> Self {
        self.set_affine_index(index);
        self
    }
    pub fn set_affine_index(&mut self, index: usize) {
        self.set_affine_index_or_flips(index);
    }

    pub const HFLIP_OFFSET: usize = Self::AFFINE_INDEX_OR_FLIPS_OFFSET + 3;
    const HFLIP_FLAG: u64 = 1 << Self::HFLIP_OFFSET;
    pub fn hflip(&self) -> bool {
        self.0 & Self::HFLIP_FLAG != 0
    }
    pub fn with_hflip(mut self, value: bool) -> Self {
        self.set_hflip(value);
        self
    }
    pub fn set_hflip(&mut self, value: bool) {
        self.set_affine_index_or_flips(if value { 0b01_000 } else { 0 });
    }

    pub const VFLIP_OFFSET: usize = Self::AFFINE_INDEX_OR_FLIPS_OFFSET + 4;
    const VFLIP_FLAG: u64 = 1 << Self::VFLIP_OFFSET;
    pub fn vflip(&self) -> bool {
        self.0 & Self::VFLIP_FLAG != 0
    }
    pub fn with_vflip(mut self, value: bool) -> Self {
        self.set_vflip(value);
        self
    }
    pub fn set_vflip(&mut self, value: bool) {
        self.set_affine_index_or_flips(if value { 0b10_000 } else { 0 });
    }

    pub fn hidden(&self) -> bool {
        debug_assert!(!self.is_affine(), "cannot get hide on a RotateScale sprite");
        self.double_or_hidden()
    }
    pub fn set_hidden(&mut self, value: bool) {
        debug_assert!(!self.is_affine(), "cannot set hide on a RotateScale sprite");
        self.set_double_or_hidden(value);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpriteRotation {
    _1: [u16; 3],
    hdx: u16,
    _2: [u16; 3],
    vdx: u16,
    _3: [u16; 3],
    hdy: u16,
    _4: [u16; 3],
    vdy: u16,
}

pub const SPRITE_COUNT: usize = 128;
pub const MATRIX_COUNT: usize = 32;

assert_eq_size!([SpriteEntry; SPRITE_COUNT], [SpriteRotation; MATRIX_COUNT]);

#[repr(C)]
pub union OamTable {
    sprites: *mut [SpriteEntry; SPRITE_COUNT],
    affines: *mut [SpriteRotation; MATRIX_COUNT],
}

// OAMTable

#[repr(i32)]
pub enum SpriteSize {
    SpriteSize8x8 = (ObjSize::Smallest.into_bits() << 14)
        | (ObjShape::Square.into_bits() << 12)
        | ((8 * 8) >> 5),
    SpriteSize16x16 = (ObjSize::Small.into_bits() << 14)
        | (ObjShape::Square.into_bits() << 12)
        | ((16 * 16) >> 5),
    SpriteSize32x32 = (ObjSize::Large.into_bits() << 14)
        | (ObjShape::Square.into_bits() << 12)
        | ((32 * 32) >> 5),
    SpriteSize64x64 = (ObjSize::Largest.into_bits() << 14)
        | (ObjShape::Square.into_bits() << 12)
        | ((64 * 64) >> 5),

    SpriteSize16x8 = (ObjSize::Smallest.into_bits() << 14)
        | (ObjShape::Wide.into_bits() << 12)
        | ((16 * 8) >> 5),
    SpriteSize32x8 =
        (ObjSize::Small.into_bits() << 14) | (ObjShape::Wide.into_bits() << 12) | ((32 * 8) >> 5),
    SpriteSize32x16 =
        (ObjSize::Large.into_bits() << 14) | (ObjShape::Wide.into_bits() << 12) | ((32 * 16) >> 5),
    SpriteSize64x32 = (ObjSize::Largest.into_bits() << 14)
        | (ObjShape::Wide.into_bits() << 12)
        | ((64 * 32) >> 5),

    SpriteSize8x16 = (ObjSize::Smallest.into_bits() << 14)
        | (ObjShape::Tall.into_bits() << 12)
        | ((8 * 16) >> 5),
    SpriteSize8x32 =
        (ObjSize::Small.into_bits() << 14) | (ObjShape::Tall.into_bits() << 12) | ((8 * 32) >> 5),
    SpriteSize16x32 =
        (ObjSize::Large.into_bits() << 14) | (ObjShape::Tall.into_bits() << 12) | ((16 * 32) >> 5),
    SpriteSize32x64 = (ObjSize::Largest.into_bits() << 14)
        | (ObjShape::Tall.into_bits() << 12)
        | ((32 * 64) >> 5),
}

#[repr(i32)]
pub enum SpriteMapping {
    Bmp1d256 = 7340115,
}

#[repr(i32)]
pub enum SpriteColorFormat {
    SpriteColorFormat16color = 0,
    SpriteColorFormat256color = 1,
    SpriteColorFormatBmp = 3,
}

#[repr(C)]
pub struct OamState {
    gfx_offset_step: i32,
    first_free: i16,
    alloc_buffer_size: i16,
    alloc_buffer: *mut core::ffi::c_void,
    table: OamTable,
    sprite_mapping: SpriteMapping,
}
assert_eq_size!(OamState, [u8; 20]);
impl OamState {
    pub unsafe fn is_initialized(this: *const Self) -> bool {
        let this = unsafe { this.read_volatile() };
        !this.alloc_buffer.is_null()
    }

    pub fn sprites_mut(&mut self) -> &mut [SpriteEntry; SPRITE_COUNT] {
        // # Safety
        // SpriteEntry and SpriteRotation have padding so their fields don't overlap
        let sprites = unsafe { self.table.sprites };
        unsafe { sprites.as_mut().unwrap_unchecked() }
    }
    pub fn affines_mut(&mut self) -> &mut [SpriteRotation; MATRIX_COUNT] {
        // # Safety
        // SpriteEntry and SpriteRotation have padding so they don't overlap
        let affines = unsafe { self.table.affines };
        unsafe { affines.as_mut().unwrap_unchecked() }
    }
}

unsafe extern "C" {
    pub unsafe static mut oamMain: OamState;
    pub unsafe static mut oamSub: OamState;
}

// oamGfxPtrToOffset

unsafe extern "C" {
    #[link_name = "oamInit"]
    pub unsafe fn oam_init(oam: *mut OamState, mapping: SpriteMapping, ext: bool);
    // oamDisable
    // oamEnable
    // oamGetGfxPtr
    #[link_name = "oamAllocateGfx"]
    pub unsafe fn oam_allocate_gfx(
        oam: *mut OamState,
        size: SpriteSize,
        color_format: SpriteColorFormat,
    ) -> *mut u16;
    #[link_name = "oamFreeGfx"]
    pub unsafe fn oam_free_gfx(oam: *mut OamState, ptr: *const u16);

    #[link_name = "oamSet"]
    pub unsafe fn oam_set(
        oam: *mut OamState,
        id: u32,
        x: u32,
        y: u32,
        prio: i32,
        pal: i32,
        size: SpriteSize,
        color_format: SpriteColorFormat,
        gfx: *const u16,
        affine: u32,
        double: bool,
        hide: bool,
        hflip: bool,
        vflip: bool,
        mosaic: bool,
    );

    #[link_name = "oamSetGfx"]
    pub unsafe fn oam_set_gfx(oam: *mut OamState, id: i32, size: SpriteSize, format: SpriteColorFormat,
               gfx: *const u16);

    #[link_name = "oamUpdate"]
    pub safe fn oam_update(oam: *mut OamState);
}
