#![allow(unused_parens)]
#![allow(clippy::unusual_byte_groupings)]

const CH0_SRC: *mut (*const usize) = 0x040000B0 as *mut (*const usize);
const CH1_SRC: *mut (*const usize) = 0x040000BC as *mut (*const usize);
const CH2_SRC: *mut (*const usize) = 0x040000C8 as *mut (*const usize);
const CH3_SRC: *mut (*const usize) = 0x040000D4 as *mut (*const usize);

const CH0_DST: *mut (*mut usize) = 0x040000B4 as *mut (*mut usize);
const CH1_DST: *mut (*mut usize) = 0x040000C0 as *mut (*mut usize);
const CH2_DST: *mut (*mut usize) = 0x040000CC as *mut (*mut usize);
const CH3_DST: *mut (*mut usize) = 0x040000D8 as *mut (*mut usize);

const CH0_CR: *mut u32 = 0x040000B8 as *mut u32;
const CH1_CR: *mut u32 = 0x040000C4 as *mut u32;
const CH2_CR: *mut u32 = 0x040000D0 as *mut u32;
const CH3_CR: *mut u32 = 0x040000DC as *mut u32;

const CH0_FILL: *mut u32 = 0x040000E0 as *mut u32;
const CH1_FILL: *mut u32 = 0x040000E4 as *mut u32;
const CH2_FILL: *mut u32 = 0x040000E8 as *mut u32;
const CH3_FILL: *mut u32 = 0x040000EC as *mut u32;

#[bitenum]
#[repr(u8)]
enum DestinationAddressUpdate {
    #[fallback]
    Increment = 0,
    Decrement = 1,
    Fixed = 2,
    IncrementReload = 3,
}

#[bitenum]
#[repr(u8)]
enum SourceAddressUpdate {
    #[fallback]
    Increment = 0,
    Decrement = 1,
    Fixed = 2,
}

#[bitenum]
#[repr(u8)]
enum StartMode {
    #[fallback]
    Immediate = 0,
    Vblank = 1,
    Hblank = 2,
    Sync = 3,
    MainMem = 4,
    Cart = 5,
    Accessory = 6,
    Geometry = 7,
}

#[bitfield(u32, debug = false)]
struct DmaControl {
    #[bits(21)]
    count: u32,
    #[bits(2)]
    dar: DestinationAddressUpdate,
    #[bits(2)]
    sar: SourceAddressUpdate,
    repeat: bool,
    width: bool,
    #[bits(3)]
    mode: StartMode,
    irq: bool,
    enable: bool,
}

/// The DMA has 4 "channels" to operate.
/// Channels set the priority of operation being processed,
/// so when multiple operations are scheduled, the one with the
/// highest priority (= lowest channel) is resolved first. If another
/// operation with a lower priority is being processed, then it is stopped
/// and later resumed
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Channel {
    /// Highest priority
    Ch0 = 0,
    /// Second highest priority
    Ch1 = 1,
    /// Third highest priority
    Ch2 = 2,
    /// Lowest priority
    Ch3 = 3,
}

/// Gets the SRC reg. addr for the specified channel
pub const fn calc_src(ch: Channel) -> *mut (*const usize) {
    match ch {
        Channel::Ch0 => CH0_SRC,
        Channel::Ch1 => CH1_SRC,
        Channel::Ch2 => CH2_SRC,
        Channel::Ch3 => CH3_SRC,
    }
}

/// Gets the DST reg. addr for the specified channel
pub const fn calc_dst(ch: Channel) -> *mut (*mut usize) {
    match ch {
        Channel::Ch0 => CH0_DST,
        Channel::Ch1 => CH1_DST,
        Channel::Ch2 => CH2_DST,
        Channel::Ch3 => CH3_DST,
    }
}

/// Gets the CR reg. addr for the specified channel
pub const fn calc_cr(ch: Channel) -> *mut u32 {
    match ch {
        Channel::Ch0 => CH0_CR,
        Channel::Ch1 => CH1_CR,
        Channel::Ch2 => CH2_CR,
        Channel::Ch3 => CH3_CR,
    }
}

/// Gets the FILL reg. addr for the specified channel
pub const fn calc_fill(ch: Channel) -> *mut u32 {
    match ch {
        Channel::Ch0 => CH0_FILL,
        Channel::Ch1 => CH1_FILL,
        Channel::Ch2 => CH2_FILL,
        Channel::Ch3 => CH3_FILL,
    }
}

/// Returns the addresses for the 4 registers
pub const fn calc_registers(
    ch: Channel,
) -> (*mut (*const usize), *mut (*mut usize), *mut u32, *mut u32) {
    let src_cr = calc_src(ch);
    let dst_cr = calc_dst(ch);
    let cr = calc_cr(ch);
    let fill_cr = calc_fill(ch);
    (src_cr, dst_cr, cr, fill_cr)
}
