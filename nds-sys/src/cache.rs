use core::ffi::c_void;

extern "C" {
    pub fn IC_InvalidateAll();
    pub fn IC_InvalidateRange(base: *const c_void, size: usize);
    pub fn DC_FlushAll();
    pub fn DC_FlushRange(base: *const c_void, size: usize);
    pub fn DC_InvalidateAll();
    pub fn DC_InvalidateRange(base: *const c_void, size: usize);
}
