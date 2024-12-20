use self::graphics::{Graphics, GraphicsMode};

pub mod backgrounds;
pub mod banks;
pub mod graphics;
mod layers;
pub use layers::*;

/// Represent and controls access to the video hardware.
///
/// Having a mutable reference to this type allows you change the video settings.
///
/// Check [`Hw`](crate::Hw) to learn how to get an object of this type.
///
/// # Example
/// ```rust,no_run
/// # let mut video = hw.take_video().unwrap();
/// // Display from VRAM bank A. Bgr555, 256x192, only one layer
/// let mut graphics = video.new_graphics::<VramA>();
/// // Commit the changes to the hardware
/// unsafe { graphics.commit(); }
/// // Use DMA to fill the screen with black
/// dma::fill(0x8000, graphics.framebuffer());
/// ```
pub struct Video {}
impl Video {
    pub fn new_graphics<M: GraphicsMode>(&mut self) -> Graphics<'_, M> {
        Graphics::new()
    }
    pub fn new_vram(&mut self) -> ! {
        todo!("new_vram")
    }
}

pub trait IntoRegisterValue {
    type SIZE;
    const REGISTER: *mut Self::SIZE;

    fn as_value(&self) -> Self::SIZE;

    /// Commit the changes to the hardware.
    ///
    /// # Safety
    /// Check the documentation of the register you are writing to,
    /// or hardware that may be affected by the change.
    #[inline]
    unsafe fn commit(&self) {
        Self::REGISTER.write_volatile(self.as_value());
    }
}

// TODO: Move this to a more appropriate place
// TODO: This is a barebones implementation, it should be improved
#[repr(C)]
#[derive(Clone, Copy)]
/// Represents an affine transformation.
/// Currently only the identity transformation is supported.
/// Use it with backgrounds that have the [`AffineBackgroundMarker`] trait.
pub struct AffineTransform {
    rot_scale: [u16; 4],
    disp: [u32; 2],
}
impl AffineTransform {
    pub const fn identity() -> Self {
        Self {
            rot_scale: [0x0100, 0x0000, 0x0000, 0x0100],
            disp: [0; 2],
        }
    }
}
