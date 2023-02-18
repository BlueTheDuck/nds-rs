pub use background::{BgSize, BgType, Layer};
use nds_sys::{
    background::{self, bg_get_map_base, bg_get_tile_base, bg_init, bg_init_sub, BackgroundId},
    bindings::bgState,
    video::{BG_GFX, BG_GFX_SUB},
};

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
            unsafe {
                bg_init(ID.get_layer(), type_, size, 0, 0)
                    .try_into()
                    .unwrap_unchecked()
            }
        } else {
            unsafe {
                bg_init_sub(ID.get_layer(), type_, size, 0, 0)
                    .try_into()
                    .unwrap_unchecked()
            }
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
