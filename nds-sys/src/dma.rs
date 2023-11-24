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

bitflags! {
    pub struct Flags: u32 {
        const ENABLED = 0b1_0_000_0_0_00_00_000000000000000000000;
        const ENABLE = 0b1_0_000_0_0_00_00_000000000000000000000;

        const INT_REQ = 0b0_1_000_0_0_00_00_000000000000000000000;

        const START_IMM         = 0b0_0_000_0_0_00_00_000000000000000000000;
        const START_AT_VBLANK   = 0b0_0_001_0_0_00_00_000000000000000000000;
        const START_AT_HBLANK   = 0b0_0_010_0_0_00_00_000000000000000000000;
        const SYNC_WITH_DISPLAY = 0b0_0_011_0_0_00_00_000000000000000000000;
        const MAIN_MEM          = 0b0_0_100_0_0_00_00_000000000000000000000;
        const GAMECARD          = 0b0_0_101_0_0_00_00_000000000000000000000;
        const DS_ACCS           = 0b0_0_110_0_0_00_00_000000000000000000000;
        const GEO_CMD_FIFO      = 0b0_0_111_0_0_00_00_000000000000000000000;
        const START_MASK        = 0b0_0_111_0_0_00_00_000000000000000000000;

        const WORDS     = 0b0_0_000_1_0_00_00_000000000000000000000;
        const HALFWORDS = 0b0_0_000_0_0_00_00_000000000000000000000;

        const REPEAT = 0b0_0_000_0_1_00_00_000000000000000000000;

        const INC_SRC  = 0b0_0_000_0_0_00_00_000000000000000000000;
        const DEC_SRC  = 0b0_0_000_0_0_01_00_000000000000000000000;
        const FIX_SRC  = 0b0_0_000_0_0_10_00_000000000000000000000;
        const SRC_MASK = 0b0_0_000_0_0_11_00_000000000000000000000;

        const INC_DST     = 0b0_0_000_0_0_00_00_000000000000000000000;
        const DEC_DST     = 0b0_0_000_0_0_00_01_000000000000000000000;
        const FIX_DST     = 0b0_0_000_0_0_00_10_000000000000000000000;
        /// Increment reload
        const INC_REL_DST = 0b0_0_000_0_0_00_11_000000000000000000000;
        const DST_MASK    = 0b0_0_000_0_0_00_11_000000000000000000000;

        const LEN_MASK = 0b111111111111111111111;
    }
}
impl core::fmt::Display for Flags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let enabled = self.contains(Flags::ENABLED);
        let int_req = self.contains(Flags::INT_REQ);
        let start_mode = match *self & Flags::START_MASK {
            Flags::START_IMM => "Immediate",
            Flags::START_AT_VBLANK => "At VBlank",
            Flags::START_AT_HBLANK => "At HBlank",
            Flags::SYNC_WITH_DISPLAY => "Sync with display",
            _ => unreachable!(),
        };
        let size = if self
            .contains(Flags::WORDS) { "Words" } else { "Halfwords" };
        let repeat = self.contains(Flags::REPEAT);
        let src_mode = match *self & Flags::SRC_MASK {
            Flags::INC_SRC => "Increment",
            Flags::DEC_SRC => "Decrement",
            Flags::FIX_SRC => "Fixed",
            _ => unreachable!(),
        };
        let dst_mode = match *self & Flags::DST_MASK {
            Flags::INC_DST => "Increment",
            Flags::DEC_DST => "Decrement",
            Flags::FIX_DST => "Fixed",
            Flags::INC_REL_DST => "Increment reload",
            _ => unreachable!(),
        };
        let len = self.bits & Flags::LEN_MASK.bits;

        f.debug_struct("DmaFlags")
            .field("enabled", &enabled)
            .field("IRQ", &int_req)
            .field("start", &start_mode)
            .field("size", &size)
            .field("repeat", &repeat)
            .field("source", &src_mode)
            .field("destination", &dst_mode)
            .field("len", &len)
            .finish()
    }
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
