use nds_sys::sprite::{self, OamState, SpriteMapping, oamMain};

use crate::interrupts::critical_section;

pub fn oam_init_main(mapping: SpriteMapping, extended: bool) -> &'static mut OamState {
    critical_section(|| {
        if unsafe { OamState::is_initialized(&raw const oamMain) } {
            panic!("OAM is already initialized");
        }
        let oam = &raw mut oamMain;

        unsafe {
            sprite::oam_init(oam, mapping, extended);
        }

        unsafe { oam.as_mut().unwrap_unchecked() }
    })
}
