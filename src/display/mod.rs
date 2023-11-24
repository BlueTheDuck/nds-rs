use self::graphics::{Graphics, GraphicsMode};

pub mod backgrounds;
pub mod graphics;

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
    pub fn new_graphics<'v, M: GraphicsMode>(&'v mut self) -> Graphics<'v, M> {
        Graphics::new()
    }
    pub fn new_vram(&mut self) -> () {
        todo!("new_vram")
    }
}

pub trait IntoRegisterValue {
    type SIZE;
    const REGISTER: *mut Self::SIZE;

    fn into_value(&self) -> Self::SIZE;

    #[inline]
    unsafe fn commit(&self) {
        Self::REGISTER.write_volatile(self.into_value());
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
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
