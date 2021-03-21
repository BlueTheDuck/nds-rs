/// Base address that used to calculate the src. control reg. address for each channel
static mut BASE_SRC: *mut (*const usize) = 0x040000B0 as *mut (*const usize);
/// Base address that used to calculate the dst. control reg. address for each channel
static mut BASE_DST: *mut (*mut usize) = 0x040000B4 as *mut (*mut usize);
/// Base address that used to calculate the control reg. address for each channel
static mut BASE_CR: *mut u32 = 0x040000B8 as *mut u32;
/// Base address that used to calculate the fill control reg. address for each channel
static mut BASE_FILL: *mut u32 = 0x040000E0 as *mut u32;


bitflags! {
    pub struct Flags: u32 {
        const ENABLED = 0b1_0_000_0_0_00_00_000000000000000000000;
        const ENABLE = 0b1_0_000_0_0_00_00_000000000000000000000;
        
        const INT_REQ = 0b0_1_000_0_0_00_00_000000000000000000000;
        
        const START_IMM = 0b0_0_000_0_0_00_00_000000000000000000000;
        const START_AT_VBLANK = 0b0_0_001_0_0_00_00_000000000000000000000;
        const START_AT_HBLANK = 0b0_0_010_0_0_00_00_000000000000000000000;
        const SYNC_WITH_DISPLAY = 0b0_0_011_0_0_00_00_000000000000000000000;
        const MAIN_MEM = 0b0_0_100_0_0_00_00_000000000000000000000;
        const GAMECARD = 0b0_0_101_0_0_00_00_000000000000000000000;
        const DS_ACCS = 0b0_0_110_0_0_00_00_000000000000000000000;
        const GEO_CMD_FIFO = 0b0_0_111_0_0_00_00_000000000000000000000;

        const WORDS = 0b0_0_000_1_0_00_00_000000000000000000000;
        const HALFWORDS = 0b0_0_000_0_0_00_00_000000000000000000000;

        const REPEAT = 0b0_0_000_0_1_00_00_000000000000000000000;

        const INC_SRC = 0b0_0_000_0_0_00_00_000000000000000000000;
        const DEC_SRC = 0b0_0_000_0_0_01_00_000000000000000000000;
        const FIX_SRC = 0b0_0_000_0_0_10_00_000000000000000000000;

        const INC_DST = 0b0_0_000_0_0_00_00_000000000000000000000;
        const DEC_DST = 0b0_0_000_0_0_00_00_000000000000000000000;
        const FIX_DST = 0b0_0_000_0_0_00_00_000000000000000000000;
        const INC_REL_DST = 0b0_0_000_0_0_00_00_000000000000000000000;

    }
}


/// The DMA has 4 channels to operate
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Channel {
    Ch0 = 0,
    Ch1 = 1,
    Ch2 = 2,
    Ch3 = 3,
}

/// Gets the SRC reg. addr for the specified channel
pub fn calc_src(ch: Channel) -> *mut (*const usize) {
    unsafe { BASE_SRC.add((ch as u8 * 12 / 4).into()) }
}

/// Gets the DST reg. addr for the specified channel
pub fn calc_dst(ch: Channel) -> *mut (*mut usize) {
    unsafe { BASE_DST.add((ch as u8 * 12 / 4).into()) }
}

/// Gets the CR reg. addr for the specified channel
pub fn calc_cr(ch: Channel) -> *mut u32 {
    unsafe { BASE_CR.add((ch as u8 * 12 / 4).into()) }
}

/// Gets the FILL reg. addr for the specified channel
pub fn calc_fill(ch: Channel) -> *mut u32 {
    unsafe { BASE_FILL.add((ch as u8 * 4 / 4).into()) }
}

/// Returns the addresses for the 4 registers
pub fn calc_registers(ch: Channel) -> (*mut (*const usize), *mut (*mut usize), *mut u32, *mut u32) {
    let src_cr = calc_src(ch);
    let dst_cr = calc_dst(ch);
    let cr = calc_cr(ch);
    let fill_cr = calc_fill(ch);
    return (src_cr, dst_cr, cr, fill_cr);
}
