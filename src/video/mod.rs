pub mod engines;
pub mod banks;
pub mod api;

pub mod colors {
    #![allow(clippy::unusual_byte_groupings)]

    use super::Color;

    pub static BLACK: Color = Color(0b0_00000_00000_00000);
    pub static RED: Color = Color(0b0_00000_00000_11111);
    pub static GREEN: Color = Color(0b0_00000_11111_00000);
    pub static BLUE: Color = Color(0b0_11111_00000_00000);
    pub static WHITE: Color = Color(0b0_11111_11111_11111);
    pub static TRANSPARENT: Color = Color(0b1_00000_00000_00000);
}

/// ABGR1555 color
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u16);
impl Color {
    pub fn get(&self) -> u16 {
        self.0
    }
    pub fn red(self) -> u16 {
        self.0 & colors::RED.0
    }
    pub fn green(self) -> u16 {
        (self.0 & colors::GREEN.0) >> 5
    }
    pub fn blue(self) -> u16 {
        (self.0 & colors::BLUE.0) >> 10
    }
    pub fn set_red(&mut self, amount: u16) {
        self.0 &= !colors::RED.0;
        self.0 |= amount;
    }
    pub fn set_green(&mut self, amount: u16) {
        self.0 &= !colors::GREEN.0;
        self.0 |= amount << 5;
    }
    pub fn set_blue(&mut self, amount: u16) {
        self.0 &= !colors::BLUE.0;
        self.0 |= amount << 10;
    }
}
impl From<Color> for u16 {
    fn from(color: Color) -> Self {
        color.0
    }
}
