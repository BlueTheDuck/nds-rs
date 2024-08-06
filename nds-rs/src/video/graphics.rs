use core::{cell::Cell, marker::PhantomData};
use nds_sys::video::{DispCntFlags, SCREEN_HEIGHT, SCREEN_WIDTH};

use super::{
    backgrounds::{DirectBitmapLayer, DirectBitmapSize},
    IntoRegisterValue, Layer2, Layer3,
};

/// A struct that represents the graphics hardware, either the main engine or the sub engine.
/// Having a mut ref allows you to control one of the engines.
///
/// See [`crate::Hw::take_video`] for information on how to get an instance of this struct.
///
/// Currently, only the main engine is supported.
/// Only modes [`Mode5`] and [`VramA`] are implemented.
///
/// # Example
/// ```rust,no_run
/// # #[entry]
/// # fn main(mut hw: nds::Hw) -> ! {
///   let mut video = hw.take_video().unwrap();
///   // Use the main engine to draw from Bank A
///   let mut graphics = video.new_graphics::<VramA>();
/// # }
/// ```
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
        let mut bgs_displayed = DispCntFlags::empty();
        bgs_displayed.set(DispCntFlags::BG0, v0);
        bgs_displayed.set(DispCntFlags::BG1, v1);
        bgs_displayed.set(DispCntFlags::BG2, v2);
        bgs_displayed.set(DispCntFlags::BG3, v3);
        (M::MODE | bgs_displayed).bits()
    }
}

/// Generic methods that are available in all modes, such as hiding/showing backgrounds.
///
impl<'v, M> Graphics<'v, M> {
    pub(crate) fn new() -> Self {
        Self {
            _mode: PhantomData::<M>,
            _video: PhantomData::<&'v ()>,
            bgs_displayed: Cell::new((false, false, false, false)),
        }
    }

    // TODO: Use enum, not u8
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

    // TODO: Use enum, not u8
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

/// This specialization has methods that are only available in mode 5,
/// such as constructing bitmap layers.
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

/// This specialization has methods that are only available when rendering directly from VRAM,
/// in particular the Bank A, such as getting the framebuffer.
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

/// Marker trait that limits which modes can be used with [`Graphics`].
#[doc(hidden)]
pub trait GraphicsMode: crate::private::Sealed {
    #[doc(hidden)]
    const MODE: DispCntFlags;
}

/// Marker struct that tells the compiler to construct a [`Graphics`] struct that uses the normal
/// graphics engine.
pub struct Mode5;
impl GraphicsMode for Mode5 {
    const MODE: DispCntFlags =
        DispCntFlags::union(DispCntFlags::MODE5, DispCntFlags::DISPLAY_GRAPHICS);
}

/// Marker struct that tells the compiler to construct a [`Graphics`] struct that display directly from VRAM
pub struct VramA;
impl GraphicsMode for VramA {
    const MODE: DispCntFlags =
        DispCntFlags::union(DispCntFlags::VRAM_A, DispCntFlags::DISPLAY_VRAM);
}
