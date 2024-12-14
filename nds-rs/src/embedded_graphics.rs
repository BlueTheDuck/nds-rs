use nds_sys::video::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub use target::GraphicsTarget;

mod target;

const PIXELS: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as _;
pub type Framebuffer = [u16; PIXELS];
