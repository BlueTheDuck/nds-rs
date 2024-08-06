use embedded_graphics_core::{
    pixelcolor::IntoStorage,
    prelude::{DrawTarget, OriginDimensions, Pixel, Point, Size},
};
use nds_sys::{
    dma::Channel,
    video::{SCREEN_HEIGHT, SCREEN_WIDTH},
};

use crate::{cache::dc_flush_array, dma};

use super::Framebuffer;

/// A struct that implements [`DrawTarget`] and [`OriginDimensions`] from
/// [`embedded-graphics`](https://docs.rs/embedded-graphics/latest/embedded_graphics/index.html).
///
/// It can be used to draw to either a layer (Running in modes 3, 4 or 5)
pub struct GraphicsTarget<'t, 'b> {
    target: &'t mut Framebuffer,
    buffer: &'b mut Framebuffer,
}
impl<'t, 'b> GraphicsTarget<'t, 'b> {
    pub fn new(target: &'t mut Framebuffer, buffer: &'b mut Framebuffer) -> Self {
        Self { target, buffer }
    }

    pub fn flush(&mut self) {
        unsafe {
            dc_flush_array(self.buffer);
        }
        dma::copy(self.buffer, self.target)
    }
}
impl OriginDimensions for GraphicsTarget<'_, '_> {
    fn size(&self) -> embedded_graphics_core::prelude::Size {
        Size::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}
impl DrawTarget for GraphicsTarget<'_, '_> {
    type Color = embedded_graphics_core::pixelcolor::Bgr555;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics_core::Pixel<Self::Color>>,
    {
        let pixels = pixels
            .into_iter()
            .filter(|&Pixel(p, _)| {
                0 <= p.x && (p.x as u32) < SCREEN_WIDTH && 0 <= p.y && (p.y as u32) < SCREEN_HEIGHT
            })
            .map(|Pixel(Point { x, y }, color)| (SCREEN_WIDTH * y as u32 + x as u32, color));
        for (index, color) in pixels {
            self.buffer[index as usize] = color.into_storage();
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        dma::fill(color.into_storage(), self.buffer);
        dma::wait_for(Channel::Ch3);
        unsafe {
            dc_flush_array(self.buffer);
        }
        Ok(())
    }
}
