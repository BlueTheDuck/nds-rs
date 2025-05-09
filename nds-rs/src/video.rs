use crate::background::{MainGraphicsMode, SubGraphicsMode, ValidGraphicsMode};

pub struct Video {}
impl Video {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn set_graphics_mode<L2, L3>(&mut self, mode: MainGraphicsMode<L2, L3>)
    where
        MainGraphicsMode<L2, L3>: ValidGraphicsMode,
    {
        mode.apply(self);
    }

    pub fn set_sub_graphics_mode<L2, L3>(&mut self, mode: SubGraphicsMode<L2, L3>)
    where
        SubGraphicsMode<L2, L3>: ValidGraphicsMode,
    {
        mode.apply(self);
    }
}
