use core::{marker::PhantomData, mem::size_of};

use crate::display::{
    graphics::{Layer2, Layer3},
    AffineTransform, IntoRegisterValue,
};

use super::AffineBackgroundMarker;

#[derive(Clone, Copy)]
pub enum DirectBitmapSize {
    /// 256x256
    Small = 0,
    /// 256x512
    Tall = 1,
    /// 512x256
    Wide = 2,
    /// 512x512
    Big = 3,
}
impl DirectBitmapSize {
    pub const fn pixel_count(self) -> usize {
        match self {
            Self::Small => 256 * 256,
            Self::Tall => 256 * 512,
            Self::Wide => 512 * 256,
            Self::Big => 512 * 512,
        }
    }
}

pub struct DirectBitmapLayer<'g, L> {
    _graphics: PhantomData<&'g ()>,
    _layer: PhantomData<L>,
    pub block: u8,
    pub size: DirectBitmapSize,
}
impl<'g, L> DirectBitmapLayer<'g, L> {
    const BLOCK_SIZE: usize = 0x4000;

    pub fn new(block: u8, size: DirectBitmapSize) -> Self {
        // TODO: Check `block`?
        Self {
            _graphics: PhantomData,
            _layer: PhantomData,
            block,
            size,
        }
    }

    
    /// Returns a mut ptr to the framebuffer of this layer.
    ///
    /// # Safety
    /// The returned value may point to memory that has no bank mapped to it,
    /// or that is being used by another layer
    pub unsafe fn get_mut_framebuffer_ptr(&mut self) -> *mut u16 {
        nds_sys::video::BG_GFX.add(Self::BLOCK_SIZE / size_of::<u16>() * self.block as usize)
    }

    /// Returns a mut slice to the framebuffer of this layer.
    ///
    /// # Safety
    /// The returned slice may span memory that has no bank mapped to it,
    /// or that is being used by another layer
    pub unsafe fn get_mut_framebuffer(&mut self) -> &mut [u16] {
        // This is still unsafe
        unsafe {
            core::slice::from_raw_parts_mut(self.get_mut_framebuffer_ptr(), self.size.pixel_count())
        }
    }
}

impl<'g> IntoRegisterValue for DirectBitmapLayer<'g, Layer2> {
    type SIZE = u16;

    const REGISTER: *mut Self::SIZE = nds_sys::background::registers::BG2CNT;

    fn into_value(&self) -> Self::SIZE {
        const TILE_BASE_BLOCK: u16 = 0b00000000_00000100;
        const COLOR_MODE: u16 = 0b00000000_10000000;

        let size = match self.size {
            DirectBitmapSize::Small => 0,
            DirectBitmapSize::Tall => 1,
            DirectBitmapSize::Wide => 2,
            DirectBitmapSize::Big => 3,
        };
        (size << 14) | COLOR_MODE | TILE_BASE_BLOCK | (self.block as u16) << 2
    }
}
impl<'g> IntoRegisterValue for DirectBitmapLayer<'g, Layer3> {
    type SIZE = u16;

    const REGISTER: *mut Self::SIZE = nds_sys::background::registers::BG3CNT;

    fn into_value(&self) -> Self::SIZE {
        const TILE_BASE_BLOCK: u16 = 0b00000000_00000100;
        const COLOR_MODE: u16 = 0b00000000_10000000;

        let size = match self.size {
            DirectBitmapSize::Small => 0,
            DirectBitmapSize::Tall => 1,
            DirectBitmapSize::Wide => 2,
            DirectBitmapSize::Big => 3,
        };
        (size << 14) | COLOR_MODE | TILE_BASE_BLOCK | (self.block as u16) << 2
    }
}

impl<'g> AffineBackgroundMarker for DirectBitmapLayer<'g, Layer2> {
    const AFFINE_MATRIX_REGISTER: *mut AffineTransform = 0x0400_0020 as _;
}
impl<'g> AffineBackgroundMarker for DirectBitmapLayer<'g, Layer3> {
    const AFFINE_MATRIX_REGISTER: *mut AffineTransform = 0x0400_0030 as _;
}
