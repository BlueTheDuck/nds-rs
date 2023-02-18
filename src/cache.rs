use core::{ffi::c_void, mem::size_of};

pub unsafe fn dc_flush_slice<'a, T>(slice: &'a [T])
where
    T: Sized,
{
    let base = slice.as_ptr() as *const c_void;
    let size = slice.len() * size_of::<T>();
    nds_sys::cache::DC_FlushRange(base, size)
}
pub unsafe fn dc_flush_array<'a, T, const N: usize>(array: &'a [T; N]) {
    let base = array.as_ptr() as *const c_void;
    let size = array.len() * size_of::<T>();
    nds_sys::cache::DC_FlushRange(base, size);
}

pub unsafe fn dc_invalidate_slice<'a, T>(slice: &'a [T])
where
    T: Sized,
{
    let base = slice.as_ptr() as *const c_void;
    let size = slice.len() * size_of::<T>();
    nds_sys::cache::DC_InvalidateRange(base, size)
}

pub unsafe fn dc_invalidate_array<'a, T, const N: usize>(array: &'a [T; N])
where
    T: Sized,
{
    let base = array.as_ptr() as *const c_void;
    let size = array.len() * size_of::<T>();
    nds_sys::cache::DC_InvalidateRange(base, size)
}
