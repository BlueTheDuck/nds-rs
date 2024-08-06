pub use nds_sys::input::TouchPosition;
use nds_sys::input::*;

pub fn scan_keys() {
    unsafe {
        scanKeys();
    }
}
pub fn keys_down() -> u32 {
    unsafe { keysDown() }
}
pub fn keys_held() -> u32 {
    unsafe { keysHeld() }
}
pub fn touch_read(data: &mut TouchPosition) {
    let data_ptr: *mut TouchPosition = &mut *data;
    unsafe {
        touchRead(data_ptr);
    }
}

#[derive(PartialEq, Eq)]
pub enum KeyState {
    Up,
    Down,
    Held,
}

impl KeyState {
    /// Returns `true` if the key state is [`Up`].
    ///
    /// [`Up`]: KeyState::Up
    #[must_use]
    pub fn is_up(&self) -> bool {
        matches!(self, Self::Up)
    }

    /// Returns `true` if the key state is [`Down`].
    ///
    /// [`Down`]: KeyState::Down
    #[must_use]
    pub fn is_down(&self) -> bool {
        matches!(self, Self::Down)
    }

    /// Returns `true` if the key state is [`Held`].
    ///
    /// [`Held`]: KeyState::Held
    #[must_use]
    pub fn is_held(&self) -> bool {
        matches!(self, Self::Held)
    }

    /// Returns `true` if the key state is [`Down`] or [`Held`]
    ///
    /// [`Down`]: KeyState::Down
    /// [`Held`]: KeyState::Held
    #[must_use]
    pub fn is_pressed(&self) -> bool {
        self.is_down() || self.is_held()
    }
}
impl Default for KeyState {
    fn default() -> Self {
        Self::Up
    }
}

macro_rules! keypad {
    {$down: ident, $held: ident, $self: expr, $($key:ident=>$key_b:path),+} => {
        $(
            keypad!{$down, $held, $self.$key, $key_b}
        )+
    };
    {$down: ident, $held: ident, $self: expr, $key_b: path} => {
        if $down & ($key_b as u32) != 0 {
            $self = KeyState::Down;
        } else if $held & ($key_b as u32) != 0 {
            $self = KeyState::Held
        } else {
            $self = KeyState::Up;
        }
    };
    {$($key:ident),+} => {
        Keypad {
            $($key: KeyState::Up),+
        }
    }
}

#[derive(Default)]
pub struct Keypad {
    pub a: KeyState,
    pub b: KeyState,
    pub select: KeyState,
    pub start: KeyState,
    pub right: KeyState,
    pub left: KeyState,
    pub up: KeyState,
    pub down: KeyState,
    pub r: KeyState,
    pub l: KeyState,
    pub x: KeyState,
    pub y: KeyState,
    pub touch: KeyState,
    pub lid: KeyState,
}
impl Keypad {
    pub fn scan(&mut self) {
        scan_keys();
        let held = keys_held();
        let down = keys_down();
        // This macro sets `self.a`, `self.b`, etc... with values from `KeyState`
        keypad! {down, held, self,
            a => KeypadBits::A,
            b => KeypadBits::B,
            select => KeypadBits::Select,
            start => KeypadBits::Start,
            right => KeypadBits::Right,
            left => KeypadBits::Left,
            up => KeypadBits::Up,
            down => KeypadBits::Down,
            r => KeypadBits::R,
            l => KeypadBits::L,
            x => KeypadBits::X,
            y => KeypadBits::Y,
            touch => KeypadBits::Touch,
            lid => KeypadBits::Lid
        };
    }
}
