use core::ffi::CStr;

pub const HEADER_START: *const u8 = 0x027FFE00 as _;

#[repr(C)]
pub struct NdsHeader {
    game_title: [u8; 12],
    game_code: [u8; 4],
    maker_code: [u8; 2],
    unit_code: u8,
    // TODO
}
impl NdsHeader {
    /// Returns the header of the currently running ROM.
    pub fn running() -> &'static Self {
        unsafe { *HEADER_START.cast() }
    }

    pub fn title(&self) -> Option<&str> {
        let title = CStr::from_bytes_until_nul(&self.game_title);
        title.ok().and_then(|t| t.to_str().ok())
    }
}
