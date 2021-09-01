use nds_sys::sprite;

pub enum Shape {
    ShapeSquare = sprite::Attr0::SHAPE_SQUARE.bits() as isize,
    ShapeWide = sprite::Attr0::SHAPE_WIDE.bits() as isize,
    ShapeTall = sprite::Attr0::SHAPE_TALL.bits() as isize,
}

pub enum Size {
    SizeSmall = sprite::Attr1::SIZE_SMALL.bits() as isize,
    SizeMed = sprite::Attr1::SIZE_MED.bits() as isize,
    SizeBig = sprite::Attr1::SIZE_BIG.bits() as isize,
    SizeMax = sprite::Attr1::SIZE_MAX.bits() as isize,
}

// TODO: Maybe the `attr`s should be each one their own struct?
#[repr(align(64))]
#[derive(Clone, Copy)]
pub struct Obj {
    /// Contains shape, color, mosaic, mode, double-size, affine and y-coord
    pub attr0: sprite::Attr0,
    pub attr1: sprite::Attr1,
    pub attr2: u16,
}
impl Obj {
    pub fn bits(&self) -> u64 {
        let mut bits = 0;
        bits |= self.attr0.bits() as u64;
        bits |= (self.attr1.bits() as u64) << 16;
        bits |= (self.attr2 as u64) << 32;

        bits
    }
    pub fn set_x(&mut self, x: u16) {
        debug_assert!(x <= sprite::X_COORD_MASK, "x coord out of range");
        let bits = self.attr1.bits() & !sprite::X_COORD_MASK;
        self.attr1 = unsafe { sprite::Attr1::from_bits_unchecked(bits | x) };
    }
    pub fn set_y(&mut self, y: u16) {
        debug_assert!(y <= sprite::Y_COORD_MASK, "y coord out of range");
        let bits = self.attr0.bits() & !sprite::Y_COORD_MASK;
        self.attr0 = unsafe { sprite::Attr0::from_bits_unchecked(bits | y) };
    }
    pub fn hide(&mut self) {
        self.attr0.remove(sprite::Attr0::AFFINE_ENABLE);
        self.attr0.insert(sprite::Attr0::DOUBLE_SIZE);
    }
}
impl Default for Obj {
    fn default() -> Self {
        let mut obj = Obj {
            attr0: sprite::Attr0::empty(),
            attr1: sprite::Attr1::empty(),
            attr2: 0,
        };
        obj.hide();
        return obj;
    }
}

