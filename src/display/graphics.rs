use core::{cell::Cell, marker::PhantomData};
use nds_sys::video::{DispCntFlags, SCREEN_HEIGHT, SCREEN_WIDTH};

use super::{
    backgrounds::{DirectBitmapLayer, DirectBitmapSize},
    IntoRegisterValue,
};

pub struct Graphics<'v, M> {
    _mode: PhantomData<M>,
    _video: PhantomData<&'v ()>,
    bgs_displayed: Cell<(bool, bool, bool, bool)>,
}

impl<'v, M: GraphicsMode> IntoRegisterValue for Graphics<'v, M> {
    type SIZE = u32;

    const REGISTER: *mut Self::SIZE = nds_sys::video::REG_DISPCNT;

    fn as_value(&self) -> Self::SIZE {
        let (v0, v1, v2, v3) = self.bgs_displayed.get();
        let bgs_displayed = v0
            .then_some(DispCntFlags::BG0)
            .unwrap_or(DispCntFlags::empty())
            | v1.then_some(DispCntFlags::BG1)
                .unwrap_or(DispCntFlags::empty())
            | v2.then_some(DispCntFlags::BG2)
                .unwrap_or(DispCntFlags::empty())
            | v3.then_some(DispCntFlags::BG3)
                .unwrap_or(DispCntFlags::empty());
        (M::MODE | bgs_displayed).bits()
    }
}

impl<'v, M> Graphics<'v, M> {
    pub fn new() -> Self {
        Self {
            _mode: PhantomData::<M>,
            _video: PhantomData::<&'v ()>,
            bgs_displayed: Cell::new((false, false, false, false)),
        }
    }

    pub fn hide_background(&self, bg: u8) {
        let (v0, v1, v2, v3) = self.bgs_displayed.get();
        let new = match bg {
            0 => (false, v1, v2, v3),
            1 => (v0, false, v2, v3),
            2 => (v0, v1, false, v3),
            3 => (v0, v1, v2, false),
            _ => panic!("Invalid background index"),
        };
        self.bgs_displayed.set(new);
    }

    pub fn show_background(&self, bg: u8) {
        let (v0, v1, v2, v3) = self.bgs_displayed.get();
        let new = match bg {
            0 => (true, v1, v2, v3),
            1 => (v0, true, v2, v3),
            2 => (v0, v1, true, v3),
            3 => (v0, v1, v2, true),
            _ => panic!("Invalid background index"),
        };
        self.bgs_displayed.set(new);
    }
}

impl Graphics<'_, Mode5> {
    pub fn mode() -> DispCntFlags {
        DispCntFlags::MODE5
    }

    pub fn new_layer_2(&self, block: u8, size: DirectBitmapSize) -> DirectBitmapLayer<'_, Layer2> {
        DirectBitmapLayer::new(block, size)
    }

    pub fn new_layer_3(&self, block: u8, size: DirectBitmapSize) -> DirectBitmapLayer<'_, Layer3> {
        DirectBitmapLayer::new(block, size)
    }
}

impl<'g> Graphics<'g, VramA> {
    pub fn mode() -> DispCntFlags {
        VramA::MODE
    }

    // TODO: This is unsafe
    pub fn framebuffer(&self) -> &'g mut [u16; (SCREEN_WIDTH * SCREEN_HEIGHT) as _] {
        unsafe {
            core::slice::from_raw_parts_mut(
                nds_sys::video::VRAM_A as *mut _,
                (SCREEN_WIDTH * SCREEN_HEIGHT) as _,
            )
            .try_into()
            .unwrap_unchecked()
        }
    }
}

pub trait GraphicsMode {
    const MODE: DispCntFlags;
}

pub struct Mode5;

impl GraphicsMode for Mode5 {
    const MODE: DispCntFlags =
        DispCntFlags::union(DispCntFlags::MODE5, DispCntFlags::DISPLAY_GRAPHICS);
}

pub struct VramA;
impl GraphicsMode for VramA {
    const MODE: DispCntFlags =
        DispCntFlags::union(DispCntFlags::VRAM_A, DispCntFlags::DISPLAY_VRAM);
}

pub struct Layer0(());
impl LayerMarker for Layer0 {
    const LAYER_INDEX: usize = 0;
}

pub struct Layer1(());
impl LayerMarker for Layer1 {
    const LAYER_INDEX: usize = 1;
}

pub struct Layer2(());
impl LayerMarker for Layer2 {
    const LAYER_INDEX: usize = 2;
}

pub struct Layer3(());
impl LayerMarker for Layer3 {
    const LAYER_INDEX: usize = 3;
}

pub trait LayerMarker: crate::private::Sealed {
    const LAYER_INDEX: usize;
}
