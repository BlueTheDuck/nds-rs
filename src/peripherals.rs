use spin::Mutex;

use crate::display::Video;

#[no_mangle]
pub static __HW: Mutex<Option<Hw>> = Mutex::new(Some(Hw::new()));

/// Represents the hardware of the console.
/// 
/// Having a mutable reference to this type allows you get access to the hardware,
/// such as the [`Video`](crate::display::Video).
/// 
/// The only way to get an object of this type is to call [`take`](Hw::take),
/// which will return `None` if the hardware is already taken.
/// If using the [`entry`](crate::entry) attribute, the hardware will be taken
/// automatically, and passed to the entry point.
pub struct Hw {
    video: bool,
    _unused: (),
}

impl Drop for Hw {
    fn drop(&mut self) {
        *__HW.lock() = Some(Hw::new());
    }
}
impl Hw {
    pub(crate) const fn new() -> Self {
        Self {
            video: true,
            _unused: (),
        }
    }

    #[inline]
    pub fn take() -> Option<Self> {
        let mut this = __HW.lock();
        this.take()
    }

    pub fn take_video<'s>(&'s self) -> Option<Video> {
        if self.video {
            Some(Video {})
        } else {
            None
        }
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
