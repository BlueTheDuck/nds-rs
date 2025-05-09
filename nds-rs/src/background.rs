mod graphics_mode;

#[cfg(feature = "embedded-graphics-core")]
use embedded_graphics_core::prelude::*;

use core::convert::Infallible;

pub use graphics_mode::*;

extern "C" {
    fn CP15_CleanAndFlushDCacheRange(start: *mut core::ffi::c_void, size: usize);
}

pub struct RenderTargetBitmap {
    // TODO: change lifetime?
    framebuffer: &'static mut [u16],
    width: u32,
    height: u32,
}

impl RenderTargetBitmap {
    pub(crate) const fn new(framebuffer: &'static mut [u16], width: u32, height: u32) -> Self {
        Self {
            framebuffer,
            width,
            height,
        }
    }

    #[inline]
    pub fn put_pixel(&mut self, x: u32, y: u32, color: u16) -> Option<()> {
        let offset = self.width.checked_mul(y)?.checked_add(x)? as usize;
        let pixel_mut = self.framebuffer.get_mut(offset)?;
        *pixel_mut = color;

        Some(())
    }

    pub fn flush_cache(&mut self) {
        unsafe {
            CP15_CleanAndFlushDCacheRange(
                self.framebuffer.as_mut_ptr() as _,
                self.framebuffer.len(),
            );
        }
    }
}
#[cfg(feature = "embedded-graphics-core")]
impl OriginDimensions for RenderTargetBitmap {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}
#[cfg(feature = "embedded-graphics-core")]
impl DrawTarget for RenderTargetBitmap {
    type Color = embedded_graphics_core::pixelcolor::Bgr555;

    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics_core::Pixel<Self::Color>>,
    {
        for Pixel(pos, color) in pixels {
            if self.bounding_box().contains(pos) {
                let color = u16::from_le_bytes(color.to_le_bytes());
                self.put_pixel(pos.x as u32, pos.y as u32, color | 0x8000);
            }
        }

        Ok(())
    }
}
