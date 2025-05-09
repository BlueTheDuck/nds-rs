use nds_sys::{
    background::{affine::Transformation, registers::*, BackgroundControl, BgSize},
    video::{DispCntFlags, DisplayMode, BG_GFX, BG_GFX_SUB, REG_DISPCNT, REG_DISPCNT_SUB},
};

use crate::{private::Sealed, video::Video};

use super::RenderTargetBitmap;

pub trait GraphicsModeSettings: Sealed {
    unsafe fn map_base(&self) -> *mut u16;
    unsafe fn tile_base(&self) -> *mut u16;
    unsafe fn graphics_base(&self) -> *mut u16;
}

pub struct MainGraphicsModeSettings(DispCntFlags);
impl MainGraphicsModeSettings {
    pub fn new(enabled: (bool, bool, bool, bool), _: u32, _: u32) -> Self {
        let mut flags = DispCntFlags::empty();
        flags.set(DispCntFlags::BG0, enabled.0);
        flags.set(DispCntFlags::BG1, enabled.1);
        flags.set(DispCntFlags::BG2, enabled.2);
        flags.set(DispCntFlags::BG3, enabled.3);

        Self(flags)
    }
}
impl GraphicsModeSettings for MainGraphicsModeSettings {
    unsafe fn map_base(&self) -> *mut u16 {
        const BLOCK_SIZE: usize = 0x10000 / size_of::<u16>();
        let bytes = self.0.map_base() as usize * BLOCK_SIZE;
        BG_GFX.add(bytes as _)
    }

    unsafe fn tile_base(&self) -> *mut u16 {
        const BLOCK_SIZE: usize = 0x10000 / size_of::<u16>();
        let bytes = self.0.tile_base() as usize * BLOCK_SIZE;
        BG_GFX.add(bytes as _)
    }

    unsafe fn graphics_base(&self) -> *mut u16 {
        BG_GFX
    }
}

pub struct SubGraphicsModeSettings(DispCntFlags);
impl SubGraphicsModeSettings {
    pub fn new(enabled: (bool, bool, bool, bool)) -> Self {
        let mut flags = DispCntFlags::empty();
        flags.set(DispCntFlags::BG0, enabled.0);
        flags.set(DispCntFlags::BG1, enabled.1);
        flags.set(DispCntFlags::BG2, enabled.2);
        flags.set(DispCntFlags::BG3, enabled.3);

        Self(flags)
    }
}
impl GraphicsModeSettings for SubGraphicsModeSettings {
    unsafe fn map_base(&self) -> *mut u16 {
        BG_GFX_SUB
    }

    unsafe fn tile_base(&self) -> *mut u16 {
        BG_GFX_SUB
    }

    unsafe fn graphics_base(&self) -> *mut u16 {
        BG_GFX_SUB
    }
}

pub type MainGraphicsMode<L2, L3> = GraphicsMode<L2, L3, MainGraphicsModeSettings>;
pub type SubGraphicsMode<L2, L3> = GraphicsMode<L2, L3, SubGraphicsModeSettings>;

pub struct GraphicsMode<L2, L3, R> {
    pub mode_settings: R,
    pub layer2: L2,
    pub layer3: L3,
}

impl<L2, L3> MainGraphicsMode<L2, L3> {
    const fn gfx_base_ptr() -> *mut u16 {
        0x06000000 as _
    }
}
impl<L2, L3> SubGraphicsMode<L2, L3> {
    const fn gfx_base_ptr() -> *mut u16 {
        0x06200000 as _
    }
}

/// Implement the `apply` method for the `MainGraphicsMode` type
impl<L2, L3> MainGraphicsMode<L2, L3>
where
    Self: ValidGraphicsMode,
{
    pub(crate) fn apply(&self, _: &mut Video) {
        let display_mode = match Self::MODE {
            5 => DisplayMode::GraphicsMode5,
            0..=6 => todo!(),
            _ => unreachable!(),
        };
        let control_flags = self
            .mode_settings
            .0
            .with_display_mode(display_mode)
            .union(DispCntFlags::from_bits_retain(Self::MODE));

        let bg0 = /* self.flags0() */ BackgroundControl::empty();
        let bg1 = /* self.flags1() */ BackgroundControl::empty();
        let bg2 = self.flags2();
        let bg3 = self.flags3();
        unsafe {
            REG_DISPCNT.write_volatile(control_flags.bits());
            BG0CNT.write_volatile(bg0.bits());
            BG1CNT.write_volatile(bg1.bits());
            BG2CNT.write_volatile(bg2.bits());
            BG3CNT.write_volatile(bg3.bits());
        }
        if let Some(transformation) = self.transformation2() {
            unsafe {
                BG2_TRANSFORMATION.write_volatile(transformation);
            }
        }
        if let Some(transformation) = self.transformation3() {
            unsafe {
                BG3_TRANSFORMATION.write_volatile(transformation);
            }
        }
    }
}

/// Implement the `apply` method for the `SubGraphicsMode` type
impl<L2, L3> SubGraphicsMode<L2, L3>
where
    Self: ValidGraphicsMode,
{
    pub(crate) fn apply(&self, _: &mut Video) {
        let display_mode = match Self::MODE {
            5 => DisplayMode::GraphicsMode5,
            0..=5 => todo!(),
            6 => panic!("Invalid display mode: GraphicsMode6"),
            _ => unreachable!(),
        };
        let control_flags = self
            .mode_settings
            .0
            .with_display_mode(display_mode)
            .union(DispCntFlags::from_bits_retain(Self::MODE));

        let bg0 = /* self.flags0() */ BackgroundControl::empty();
        let bg1 = /* self.flags1() */ BackgroundControl::empty();
        let bg2 = self.flags2();
        let bg3 = self.flags3();
        unsafe {
            REG_DISPCNT_SUB.write_volatile(control_flags.bits());
            DB_BG0CNT.write_volatile(bg0.bits());
            DB_BG1CNT.write_volatile(bg1.bits());
            DB_BG2CNT.write_volatile(bg2.bits());
            DB_BG3CNT.write_volatile(bg3.bits());
        }
        if let Some(transformation) = self.transformation2() {
            unsafe {
                DB_BG2_TRANSFORMATION.write_volatile(transformation);
            }
        }
        if let Some(transformation) = self.transformation3() {
            unsafe {
                DB_BG3_TRANSFORMATION.write_volatile(transformation);
            }
        }
    }
}

/// Methods available when the 3rd layer is a [`BitmapLayer`]
impl<L2, M> GraphicsMode<L2, BitmapLayer, M>
where
    M: GraphicsModeSettings,
{
    pub fn layer3_framebuffer(&self) -> RenderTargetBitmap {
        let (width, height) = self.layer3.size();
        let len = width * height;
        let framebuffer = unsafe {
            let data = self
                .mode_settings
                .graphics_base()
                .add(self.layer3.gfx_block());
            println!("{data:p}");
            core::slice::from_raw_parts_mut(data, len as usize)
        };

        RenderTargetBitmap {
            framebuffer,
            width,
            height,
        }
    }
}

pub struct BitmapLayer {
    flags: BackgroundControl,
    transformation: Transformation,
}
impl BitmapLayer {
    /// Creates a new direct color bitmap layer with a size of 256x256 pixels
    pub const fn new_fullscreen() -> Self {
        Self {
            flags: BackgroundControl::BITMAP.with_size(BgSize::FullBitmapMedium),
            transformation: Transformation::IDENTITY,
        }
    }
    /// Creates a new direct color bitmap layer with a size of 512x512 pixels
    pub const fn new_big() -> Self {
        Self {
            flags: BackgroundControl::BITMAP.with_size(BgSize::FullBitmapBig),
            transformation: Transformation::IDENTITY,
        }
    }
    pub fn size(&self) -> (u32, u32) {
        match self.flags.size_value() {
            0 => (128, 128),
            1 => (256, 256),
            2 => (512, 256),
            3 => (512, 512),
            _ => unreachable!(),
        }
    }
    const fn gfx_block(&self) -> usize {
        // Block size in ARGB1555 units
        const BLOCK_SIZE: usize = 0x4000 / size_of::<u16>();
        BLOCK_SIZE * (self.flags.map_base() as usize)
    }
}

pub trait ValidGraphicsMode: Sealed {
    const MODE: u32;
    fn flags2(&self) -> BackgroundControl;
    fn flags3(&self) -> BackgroundControl;
    fn transformation2(&self) -> Option<Transformation> {
        None
    }
    fn transformation3(&self) -> Option<Transformation> {
        None
    }
}
macro_rules! impl_valid_graphics_mode {
    {
        impl MODE = 5 for $ty:ty
    } => {
        impl ValidGraphicsMode for $ty {
            const MODE: u32 = 5;

            impl_valid_graphics_mode!(@flags_getters);

            fn transformation2(&self) -> Option<Transformation> {
                Some(self.layer2.transformation)
            }

            fn transformation3(&self) -> Option<Transformation> {
                Some(self.layer3.transformation)
            }
        }
    };

    (@flags_getters) => {
        fn flags2(&self) -> BackgroundControl { self.layer2.flags }
        fn flags3(&self) -> BackgroundControl { self.layer3.flags }
    }
}
impl_valid_graphics_mode! { impl MODE = 5 for MainGraphicsMode<BitmapLayer, BitmapLayer> }
impl_valid_graphics_mode! { impl MODE = 5 for SubGraphicsMode<BitmapLayer, BitmapLayer> }
