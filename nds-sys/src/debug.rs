#![allow(unused_parens)]

pub mod registers {
    // Reads the emulation ID. 16 bytes long
    pub const EMU_ID_PTR: *const u8 = 0x04FFFA00 as _;
    // Write a string without parameters
    pub const STRING_OUT_RAW: *mut (*const u8) = 0x4FFFA10 as _;
    pub const STRING_OUT_PARAM: *mut (*const u8) = 0x4FFFA14 as _;
    pub const STRING_OUT_PARAM_LF: *mut (*const u8) = 0x4FFFA18 as _;
    pub const CHAR_OUT: *mut char = 0x04FFFA1C as _;
}
