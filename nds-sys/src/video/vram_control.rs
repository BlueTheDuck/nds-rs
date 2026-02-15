use bitfield_struct::bitfield;

#[bitfield(u8, debug = false)]
pub struct VramControl {
    #[bits(3)]
    modeset: u32,
    #[bits(2)]
    slot: usize,
    #[bits(2)]
    __: (),
    #[bits(1, default = true)]
    enabled: bool,
}

const SLOT0: u8 = 0 << VramControl::SLOT_OFFSET;
const SLOT1: u8 = 1 << VramControl::SLOT_OFFSET;
const SLOT2: u8 = 2 << VramControl::SLOT_OFFSET;
const SLOT3: u8 = 3 << VramControl::SLOT_OFFSET;
const ENABLE: u8 = 1 << VramControl::ENABLED_OFFSET;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VramAType {
    Lcd = 0,
    MainBgSlot0 = 1 | SLOT0,
    MainBgSlot1 = 1 | SLOT1,
    MainBgSlot2 = 1 | SLOT2,
    MainBgSlot3 = 1 | SLOT3,
    MainSpriteSlot0 = 2 | SLOT0,
    MainSpriteSlot1 = 2 | SLOT1,
    TextureSlot0 = 3 | SLOT0,
    TextureSlot1 = 3 | SLOT1,
    TextureSlot2 = 3 | SLOT2,
    TextureSlot3 = 3 | SLOT3,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VramBType {
    Lcd = 0,
    MainBgSlot0 = 1 | SLOT0,
    MainBgSlot1 = 1 | SLOT1,
    MainBgSlot2 = 1 | SLOT2,
    MainBgSlot3 = 1 | SLOT3,
    MainSpriteSlot0 = 2 | SLOT0,
    MainSpriteSlot1 = 2 | SLOT1,
    TextureSlot0 = 3 | SLOT0,
    TextureSlot1 = 3 | SLOT1,
    TextureSlot2 = 3 | SLOT2,
    TextureSlot3 = 3 | SLOT3,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VramCType {
    Lcd = 0,
    MainBgSlot0 = 1 | SLOT0,
    MainBgSlot1 = 1 | SLOT1,
    MainBgSlot2 = 1 | SLOT2,
    MainBgSlot3 = 1 | SLOT3,
    Arm7Slot0 = 2 | SLOT0,
    Arm7Slot1 = 2 | SLOT1,
    TextureSlot0 = 3 | SLOT0,
    TextureSlot1 = 3 | SLOT1,
    TextureSlot2 = 3 | SLOT2,
    TextureSlot3 = 3 | SLOT3,
    SubBg = 4,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VramDType {
    Lcd = 0,
    MainBgSlot0 = 1 | SLOT0,
    MainBgSlot1 = 1 | SLOT1,
    MainBgSlot2 = 1 | SLOT2,
    MainBgSlot3 = 1 | SLOT3,
    Arm7Slot0 = 2 | SLOT0,
    Arm7Slot1 = 2 | SLOT1,
    TextureSlot0 = 3 | SLOT0,
    TextureSlot1 = 3 | SLOT1,
    TextureSlot2 = 3 | SLOT2,
    TextureSlot3 = 3 | SLOT3,
    SubSprite = 4,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VramEType {
    Lcd = 0,
    MainBg = 1,
    MainSprite = 2,
    TexPalette = 3,
    BgExtPalette = 4,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VramFType {
    Lcd = 0,
    MainBgSlot0 = 1 | SLOT0,
    MainBgSlot1 = 1 | SLOT1,
    MainBgSlot2 = 1 | SLOT2,
    MainBgSlot3 = 1 | SLOT3,
    MainSpriteSlot0 = 2 | SLOT0,
    MainSpriteSlot1 = 2 | SLOT1,
    MainSpriteSlot2 = 2 | SLOT2,
    MainSpriteSlot3 = 2 | SLOT3,
    TexPaletteSlot0 = 3 | SLOT0,
    TexPaletteSlot1 = 3 | SLOT1,
    BgExtPaletteSlot0 = 4 | SLOT0,
    BgExtPaletteSlot1 = 4 | SLOT1,
    BgExtPaletteSlot2 = 4 | SLOT2,
    BgExtPaletteSlot3 = 4 | SLOT3,
    SpriteExtPalette = 5,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VramGType {
    Lcd = 0,
    MainBgSlot0 = 1 | SLOT0,
    MainBgSlot1 = 1 | SLOT1,
    MainBgSlot2 = 1 | SLOT2,
    MainBgSlot3 = 1 | SLOT3,
    MainSpriteSlot0 = 2 | SLOT0,
    MainSpriteSlot1 = 2 | SLOT1,
    MainSpriteSlot2 = 2 | SLOT2,
    MainSpriteSlot3 = 2 | SLOT3,
    TexPaletteSlot0 = 3 | SLOT0,
    TexPaletteSlot1 = 3 | SLOT1,
    BgExtPaletteSlot0 = 4 | SLOT0,
    BgExtPaletteSlot1 = 4 | SLOT1,
    BgExtPaletteSlot2 = 4 | SLOT2,
    BgExtPaletteSlot3 = 4 | SLOT3,
    SpriteExtPalette = 5,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VramHType {
    Lcd = 0,
    SubBg = 1,
    SubBgExtPalette = 2,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VramIType {
    Lcd = 0,
    SubBg = 1,
    SubSprite = 2,
    SubSpriteExtPalette = 3,
}


unsafe extern "C" {
    #[link_name = "vramSetPrimaryBanks"]
    pub unsafe fn vram_set_primary_banks(a: VramAType, b: VramBType, c: VramCType, d: VramDType);
}
