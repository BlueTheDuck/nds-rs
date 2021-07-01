pub use background::{BgSize, BgType, Layer};
use nds_sys::{
    background::{self, bg_get_gfx_ptr, bg_get_map_base, bg_get_tile_base, bg_init, bg_init_sub},
    bindings::bgState,
    video::{BG_GFX, BG_GFX_SUB},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum BackgroundId {
    MainBg0 = 0,
    MainBg1 = 1,
    MainBg2 = 2,
    MainBg3 = 3,
    SubBg0 = 4,
    SubBg1 = 5,
    SubBg2 = 6,
    SubBg3 = 7,
}
impl BackgroundId {
    pub const fn is_main(&self) -> bool {
        match self {
            Self::MainBg0 | Self::MainBg1 | Self::MainBg2 | Self::MainBg3 => true,
            _ => false,
        }
    }
    pub fn get_layer(self) -> Layer {
        match self {
            BackgroundId::MainBg0 | BackgroundId::SubBg0 => Layer::Layer0,
            BackgroundId::MainBg1 | BackgroundId::SubBg1 => Layer::Layer1,
            BackgroundId::MainBg2 | BackgroundId::SubBg2 => Layer::Layer2,
            BackgroundId::MainBg3 | BackgroundId::SubBg3 => Layer::Layer3,
        }
    }
}
impl From<usize> for BackgroundId {
    fn from(n: usize) -> Self {
        match n {
            0 => BackgroundId::MainBg0,
            1 => BackgroundId::MainBg1,
            2 => BackgroundId::MainBg2,
            3 => BackgroundId::MainBg3,
            4 => BackgroundId::SubBg0,
            5 => BackgroundId::SubBg1,
            6 => BackgroundId::SubBg2,
            7 => BackgroundId::SubBg3,
            _ => unreachable!(),
        }
    }
}

pub struct Background<const ID: BackgroundId>;
impl<const ID: BackgroundId> Background<ID> {
    pub fn graphics_ptr(&self) -> *mut u16 {
        let bg_type: BgType = unsafe { bgState[ID as usize].type_ as usize }.into();
        match bg_type {
            BgType::Text8 | BgType::Text4 | BgType::Rotation | BgType::ExRotation => {
                if ID.is_main() {
                    unsafe { BG_GFX.add(0x4000 * bg_get_tile_base(ID as usize)) }
                } else {
                    unsafe { BG_GFX_SUB.add(0x4000 * bg_get_tile_base(ID as usize)) }
                }
            }
            BgType::Bmp8 | BgType::Bmp16 => {
                if ID.is_main() {
                    unsafe { BG_GFX.add(0x2000 * bg_get_map_base(ID as usize)) }
                } else {
                    unsafe { BG_GFX_SUB.add(0x2000 * bg_get_map_base(ID as usize)) }
                }
            }
        }
    }
    pub fn init(&self, type_: BgType, size: BgSize) -> BackgroundId {
        // TODO: Check if 3D is being used
        
        if ID.is_main() {
            unsafe { bg_init(ID.get_layer(), type_, size, 0, 0).into() }
        } else {
            unsafe { bg_init_sub(ID.get_layer(), type_, size, 0, 0).into() }
        }
    }
}

macro_rules! gen_backgrounds {
    (
        $($name:ident => $id:expr;)*
    ) => {
        $(
            pub static $name: Background<{$id}> = Background::<{$id}>;
        )*
    };
}

gen_backgrounds! {
    MAIN_BG0 => BackgroundId::MainBg0;
    MAIN_BG1 => BackgroundId::MainBg1;
    MAIN_BG2 => BackgroundId::MainBg2;
    MAIN_BG3 => BackgroundId::MainBg3;
    SUB_BG0 => BackgroundId::SubBg0;
    SUB_BG1 => BackgroundId::SubBg1;
    SUB_BG2 => BackgroundId::SubBg2;
    SUB_BG3 => BackgroundId::SubBg3;
}
