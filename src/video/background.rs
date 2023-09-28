mod bitmap;
mod text;

use nds_sys::{
    background::{BgSize, BgType, Layer},
    video::{set_video_mode, VideoMode},
};

pub use self::{
    bitmap::{BitmapLayerSize, DirectBitmapLayer},
    text::{TextLayer, TextLayerSize},
};

/// Helper type, Libnds' `bgInit` takes 4 arguments and it's easy to get them wrong.
///
/// This type is a tuple of the 4 arguments. Layers implement [`AsBgInitArgs`], that
/// converts them into this type.
/// The two `u8` are the map and tile base
type BgInitArgs = (BgType, BgSize, u8, u8);

trait AsBgInitArgs {
    fn as_bg_init_args(self) -> BgInitArgs;
}

pub enum Mode {
    Mode0 {
        layer0: Option<TextLayer>,
        layer1: Option<TextLayer>,
        layer2: Option<TextLayer>,
        layer3: Option<TextLayer>,
    },
    // TODO: Mode1
    // TODO: Mode2
    Mode3 {
        layer0: Option<TextLayer>,
        layer1: Option<TextLayer>,
        layer2: Option<TextLayer>,
        layer3: Option<DirectBitmapLayer>,
    },
    // TODO: Mode4 {}
    Mode5 {
        layer0: Option<TextLayer>,
        layer1: Option<TextLayer>,
        layer2: Option<DirectBitmapLayer>,
        layer3: Option<DirectBitmapLayer>,
    },
    // TODO: Mode6 {}
}
impl Mode {
    fn as_bg_init_args(self) -> [Option<BgInitArgs>; 4] {
        match self {
            Mode::Mode0 {
                layer0,
                layer1,
                layer2,
                layer3,
            } => [
                layer0.map(AsBgInitArgs::as_bg_init_args),
                layer1.map(AsBgInitArgs::as_bg_init_args),
                layer2.map(AsBgInitArgs::as_bg_init_args),
                layer3.map(AsBgInitArgs::as_bg_init_args),
            ],
            Mode::Mode3 {
                layer0,
                layer1,
                layer2,
                layer3,
            } => [
                layer0.map(AsBgInitArgs::as_bg_init_args),
                layer1.map(AsBgInitArgs::as_bg_init_args),
                layer2.map(AsBgInitArgs::as_bg_init_args),
                layer3.map(AsBgInitArgs::as_bg_init_args),
            ],
            Mode::Mode5 {
                layer0,
                layer1,
                layer2,
                layer3,
            } => [
                layer0.map(AsBgInitArgs::as_bg_init_args),
                layer1.map(AsBgInitArgs::as_bg_init_args),
                layer2.map(AsBgInitArgs::as_bg_init_args),
                layer3.map(AsBgInitArgs::as_bg_init_args),
            ],
        }
    }

    // pub fn bmp_data(&self) -> Option<&[u16]> {}
}

/// Initializes the main display engine with the given [`Mode`].
pub fn background_init(mode: Mode) {
    let video_mode = match mode {
        Mode::Mode0 { .. } => VideoMode::Mode0_2d,
        Mode::Mode3 { .. } => VideoMode::Mode3_2d,
        Mode::Mode5 { .. } => VideoMode::Mode5_2d,
        // TODO: The rest of the modes
        #[allow(unreachable_patterns)]
        _ => todo!(),
    };

    unsafe {
        set_video_mode(video_mode);
    }

    let bg_init_args = mode
        .as_bg_init_args()
        .into_iter()
        .zip([Layer::Layer0, Layer::Layer1, Layer::Layer2, Layer::Layer3])
        .filter_map(|(options, index)| options.map(|(a, b, c, d)| (index, a, b, c, d)));
    for (layer, bg_type, bg_size, map, tile) in bg_init_args {
        unsafe {
            nds_sys::background::bg_init(layer, bg_type, bg_size, map, tile);
        }
    }
}
