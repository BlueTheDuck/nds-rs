#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transformation {
    pub a: i16,
    pub b: i16,
    pub c: i16,
    pub d: i16,
    pub x0: i32,
    pub y0: i32,
}
impl Transformation {
    pub const IDENTITY: Transformation = Transformation {
        a: 1 << 8,
        b: 0,
        c: 0,
        d: 1 << 8,
        x0: 0,
        y0: 0,
    };
}
