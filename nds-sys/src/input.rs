extern "C" {
    pub fn scanKeys();
    pub fn keysDown() -> u32;
    pub fn keysHeld() -> u32;
    pub fn touchRead(data: *mut TouchPosition);
}

#[repr(u32)]
pub enum KeypadBits {
    A      = bit!(0),
    B      = bit!(1),
    Select = bit!(2),
    Start  = bit!(3),
    Right  = bit!(4),
    Left   = bit!(5),
    Up     = bit!(6),
    Down   = bit!(7),
    R      = bit!(8),
    L      = bit!(9),
    X      = bit!(10),
    Y      = bit!(11),
    Touch  = bit!(12),
    Lid    = bit!(13),
}
#[repr(C)]
#[derive(Default)]
pub struct TouchPosition {
    rawx: u16,
    rawy: u16,
    pub px: u16,
    pub py: u16,
    z1: u16,
    z2: u16
}

