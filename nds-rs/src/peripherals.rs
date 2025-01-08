use spin::Mutex;

use crate::{system::System, video::Video};

#[no_mangle]
pub static __HW: Mutex<Option<Hw>> = Mutex::new(Some(unsafe { Hw::new() }));

/// Represents the hardware of the console.
///
/// Having a mutable reference to this type allows you get access to the hardware,
/// such as the [`Video`](crate::display::Video).
///
/// The only way to get an object of this type is to call [`take`](Hw::take),
/// which will return `None` if the hardware is already taken.
/// If using the [`entry`](crate::entry) attribute, the hardware will be taken
/// automatically, and passed to the entry point.
///
/// # Example
/// ```rust,no_run
/// #[entry]
/// fn main(mut hw: nds::Hw) -> ! {
///   let mut video = hw.take_video().unwrap();
///   /* draw to the screen */
/// }
/// ```
///
#[non_exhaustive]
pub struct Hw {
    pub video: Video,
    pub system: System
}
impl Drop for Hw {
    fn drop(&mut self) {
        unsafe {
            *__HW.lock() = Some(Hw::new());
        }
    }
}
impl Hw {
    pub(crate) const unsafe fn new() -> Self {
        Self {
            video: Video::new(),
            system: System::new()
        }
    }

    #[inline]
    pub fn take() -> Option<Self> {
        let mut this = __HW.lock();
        this.take()
    }
}
/*
struct DrawingTarget<'g> {
    _layer: DirectBitmapLayer<'g>,
}

impl<'g> DrawingTarget<'g> {
    fn new(_layer: DirectBitmapLayer<'g>) -> Self { Self { _layer } }

    fn fill_white(&mut self) -> () {
        black_box(&self);
    }
}


fn example() {
    let hw = Hw {};
    let mut video = hw.take_video().unwrap();
    let mut graphics = video.new_graphics::<Mode5>();
    let layer_2 = graphics.new_layer_2(0, DirectBitmapSize::Size256x256);

    let mut target = DrawingTarget::new(layer_2);

    target.fill_white();
    // video.new_vram();
}
 */
