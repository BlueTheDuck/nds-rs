#![allow(unused_parens)]
#![allow(clippy::unusual_byte_groupings)]

use bitfield_struct::{bitenum, bitfield};

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

#[bitenum]
#[repr(u8)]
pub enum DestinationAddressUpdate {
    #[fallback]
    Increment = 0,
    Decrement = 1,
    Fixed = 2,
    IncrementReload = 3,
}

#[bitenum]
#[repr(u8)]
pub enum SourceAddressUpdate {
    #[fallback]
    Increment = 0,
    Decrement = 1,
    Fixed = 2,
}

#[bitenum]
#[repr(u8)]
pub enum StartMode {
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
pub struct DmaControl {
    #[bits(21)]
    pub count: usize,
    #[bits(2)]
    pub dar: DestinationAddressUpdate,
    #[bits(2)]
    pub sar: SourceAddressUpdate,
    pub repeat: bool,
    pub width: bool,
    #[bits(3)]
    pub mode: StartMode,
    pub irq: bool,
    enable: bool,
}
impl DmaControl {
    pub const fn busy(&self) -> bool {
        self.enable()
    }
    pub const fn set_start(&mut self) {
        self.set_enable(true)
    }
    pub const fn with_start(self) -> Self {
        self.with_enable(true)
    }
}

unsafe extern "C" {
    /// Starts a DMA transfer safely from ITCM.
    #[link_name = "dmaSetParams"]
    pub unsafe fn set_params(ch: Channel, src: *const u8, dst: *mut u8, control: DmaControl);
}

/// Fills `dst` with `len` words of `src`.
/// This function operates in words (32 bits), so the amount of bytes copied will be `len*4`
/// # Safety
/// **It is not recommended to use this function**, it is only provided as a compat with the C version of libnds.
/// This function works by instructing the DMA to copy `len` words from `src` to `dst`, so the caller must make
/// sure that:
///  - only [`Copy`] values are being copied,
///  - [`dst`, `dst + len`) is valid **and** won't overwrite unrelated data,
///  - and the channel is free, since [`copy_words`] will overwrite it's settings
pub unsafe fn copy_words(ch: Channel, src: *const u32, dst: *mut u32, len: usize) {
    let control = DmaControl::new()
        .with_count(len)
        .with_dar(DestinationAddressUpdate::Increment)
        .with_sar(SourceAddressUpdate::Increment)
        .with_repeat(false)
        .with_width(true)
        .with_mode(StartMode::Immediate)
        .with_irq(false)
        .with_start();
    unsafe {
        set_params(ch, src as _, dst as _, control);
    }
}

/// Fills `dst` with `len` half words of `src`.
/// This function operates in half words (16 bits), so the amount of bytes copied will be `len*2`
/// # Safety
/// **It is not recommended to use this function**, it is only provided as a compat with the C version of libnds.
/// This function works by instructing the DMA to copy `len` half words from `src` to `dst`, so the caller must make
/// sure that:
///  - only [`Copy`] values are being copied,
///  - [`dst`, `dst + len`) is valid **and** won't overwrite unrelated data,
///  - and the channel is free, since [`copy_half_words`] will overwrite it's settings
pub unsafe fn copy_half_words(ch: Channel, src: *const u16, dst: *mut u16, len: usize) {
    let control = DmaControl::new()
        .with_count(len)
        .with_dar(DestinationAddressUpdate::Increment)
        .with_sar(SourceAddressUpdate::Increment)
        .with_repeat(false)
        .with_width(false)
        .with_mode(StartMode::Immediate)
        .with_irq(false)
        .with_start();
    unsafe {
        set_params(ch, src as _, dst as _, control);
    }
}


/// Fills `dst` with `len` copies of `value`.
/// This function operates in words (32 bits), so the amount of bytes written will be `len*4`
/// # Safety
/// **It is not recommended to use this function**, it is only provided as a compat with the C version of libnds.
/// This function works by instructing the DMA to copy `len` times `value` words over `dst`, so the caller must make
/// sure that:
///  - only [`Copy`] values are being copied,
///  - [`dst`, `dst + len`) is valid **and** won't overwrite unrelated data,
///  - and the channel is free, since [`fill_words`] will overwrite it's settings
pub unsafe fn fill_words(ch: Channel, value: u32, dst: *mut u32, len: usize) {
    let control = DmaControl::new()
        .with_count(len)
        .with_dar(DestinationAddressUpdate::Fixed)
        .with_sar(SourceAddressUpdate::Increment)
        .with_repeat(false)
        .with_width(true)
        .with_mode(StartMode::Immediate)
        .with_irq(false)
        .with_start();
    unsafe {
        calc_fill(ch).write_unaligned(value);
        set_params(ch, calc_fill(ch) as _, dst as _, control);
    }
}


/// Fills `dst` with `len` copies of `value`.
/// This function operates in half words (16 bits), so the amount of bytes written will be `len*2`
/// # Safety
/// **It is not recommended to use this function**, it is only provided as a compat with the C version of libnds.
/// This function works by instructing the DMA to copy `len` times `value` half words over `dst`, so the caller must make
/// sure that:
///  - only [`Copy`] values are being copied,
///  - [`dst`, `dst + len`) is valid **and** won't overwrite unrelated data,
///  - and the channel is free, since [`fill_half_words`] will overwrite it's settings
pub unsafe fn fill_half_words(ch: Channel, value: u16, dst: *mut u16, len: usize) {
    let control = DmaControl::new()
        .with_count(len)
        .with_dar(DestinationAddressUpdate::Fixed)
        .with_sar(SourceAddressUpdate::Increment)
        .with_repeat(false)
        .with_width(false)
        .with_mode(StartMode::Immediate)
        .with_irq(false)
        .with_start();
    unsafe {
        calc_fill(ch).write_unaligned(value as _);
        set_params(ch, calc_fill(ch) as _, dst as _, control);
    }
}
