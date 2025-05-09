use super::affine::Transformation;

/// Control register for background 0 of Main Engine
pub const BG0CNT: *mut u16 = 0x04000008 as _;
/// Control register for background 1 of Main Engine
pub const BG1CNT: *mut u16 = 0x0400000A as _;
/// Control register for background 2 of Main Engine
pub const BG2CNT: *mut u16 = 0x0400000C as _;
/// Control register for background 3 of Main Engine
pub const BG3CNT: *mut u16 = 0x0400000E as _;
/// Control register for background 0 of Sub Engine
pub const DB_BG0CNT: *mut u16 = 0x04001008 as _;
/// Control register for background 1 of Sub Engine
pub const DB_BG1CNT: *mut u16 = 0x0400100A as _;
/// Control register for background 2 of Sub Engine
pub const DB_BG2CNT: *mut u16 = 0x0400100C as _;
/// Control register for background 3 of Sub Engine
pub const DB_BG3CNT: *mut u16 = 0x0400100E as _;

/// Affine transformation only. Register for background 2 of Main Engine. Controls x0 (Displacement)
pub const BG2X: *mut u32 = 0x04000028 as _;
/// Affine transformation only. Register for background 2 of Main Engine. Controls y0 (Displacement)
pub const BG2Y: *mut u32 = 0x0400002C as _;

pub const BG3PA: *mut i16 = 0x04000030 as _;
pub const BG3PB: *mut i16 = 0x04000032 as _;
pub const BG3PC: *mut i16 = 0x04000034 as _;
pub const BG3PD: *mut i16 = 0x04000036 as _;
/// Affine transformation only. Register for background 3 of Main Engine. Controls x0 (Displacement)
pub const BG3X: *mut u32 = 0x04000038 as _;
/// Affine transformation only. Register for background 3 of Main Engine. Controls y0 (Displacement)
pub const BG3Y: *mut u32 = 0x0400003C as _;

/// Affine transformation only. Register for background 2 of Sub Engine. Controls x0 (Displacement)
pub const DB_BG2X: *mut u32 = 0x04001028 as _;
/// Affine transformation only. Register for background 2 of Sub Engine. Controls y0 (Displacement)
pub const DB_BG2Y: *mut u32 = 0x0400102C as _;
/// Affine transformation only. Register for background 3 of Sub Engine. Controls x0 (Displacement)
pub const DB_BG3X: *mut u32 = 0x04001038 as _;
/// Affine transformation only. Register for background 3 of Sub Engine. Controls y0 (Displacement)
pub const DB_BG3Y: *mut u32 = 0x0400103C as _;

pub const BG2_TRANSFORMATION: *mut Transformation = 0x04000020 as _;
pub const BG3_TRANSFORMATION: *mut Transformation = 0x04000030 as _;
pub const DB_BG2_TRANSFORMATION: *mut Transformation = 0x04001020 as _;
pub const DB_BG3_TRANSFORMATION: *mut Transformation = 0x04001030 as _;

