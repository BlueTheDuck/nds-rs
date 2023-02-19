use core::{convert::Infallible, slice::from_raw_parts_mut};

use embedded_graphics_core::{
    pixelcolor::Rgb555,
    prelude::{DrawTarget, OriginDimensions, Point, Size},
    Pixel,
};
use nds_sys::{
    dma::Channel,
    video::{REG_DISPCNT, SCREEN_HEIGHT, SCREEN_WIDTH},
};

use crate::{
    cache::{dc_flush_array, dc_invalidate_array},
    dma,
};

const PIXELS: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as _;
pub type Framebuffer = [Rgb555; PIXELS];

pub struct GraphicsTarget<'b> {
    target: &'static mut Framebuffer,
    buffer: &'b mut Framebuffer,
}
impl<'b> GraphicsTarget<'b> {
    ///
    /// Initializes main engine to be used with embedded-graphics
    ///
    /// # Safety
    /// This function should be the first thing called when using graphics.
    /// Other graphics functions (backgrounds, objects, etc.) should not be used
    ///
    pub fn new(buffer: &'b mut Framebuffer) -> Self {
        // Display from VRAM bank A
        const CONTROL: u32 = 0b00_10_00000000_00000000;
        unsafe {
            REG_DISPCNT.write_volatile(CONTROL);
        }
        let target: &'static mut Framebuffer = unsafe {
            from_raw_parts_mut(nds_sys::video::VRAM_A as *mut Rgb555, PIXELS)
                .try_into()
                .unwrap_unchecked()
        };

        Self { target, buffer }
    }

    pub fn flush(&mut self) {
        unsafe {
            dc_flush_array(self.buffer);
        }
        dma::copy(self.buffer, self.target)
    }
}
impl OriginDimensions for GraphicsTarget<'_> {
    fn size(&self) -> embedded_graphics_core::prelude::Size {
        Size::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}
impl DrawTarget for GraphicsTarget<'_> {
    type Color = Rgb555;

    type Error = Infallible;

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
            self.buffer[index as usize] = color;
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        unsafe {
            dc_flush_array(self.buffer);
        }
        dma::fill(color, self.buffer);
        dma::wait_for(Channel::Ch3);
        unsafe {
            dc_invalidate_array(self.buffer);
        }
        Ok(())
    }
}
