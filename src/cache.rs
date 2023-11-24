use core::mem::size_of_val;

/// Flushes the data cache for the given slice.
///
/// # Safety
///
/// If the contents of the slice in cache and memory are different, the
/// data in memory will be overwritten with the data in cache.
///
/// See also [`dc_invalidate_slice`], which is the opposite operation.
#[inline]
pub unsafe fn dc_flush_slice(slice: &[impl Sized]) {
    nds_sys::cache::DC_FlushRange(slice.as_ptr() as _, size_of_val(slice))
}

/// Flushes the data cache for the given array.
///
/// # Safety
///
/// If the contents of the array in cache and memory are different, the
/// data in memory will be overwritten with the data in cache.
///
/// See also [`dc_invalidate_array`], which is the opposite operation.
#[inline]
pub unsafe fn dc_flush_array<const N: usize>(array: &[impl Sized; N]) {
    dc_flush_slice(array)
}

/// Invalidates the data cache for the given slice.
///
/// # Safety
///
/// If the contents of the slice in cache and memory are different, the
/// next time the slice is accessed, a new copy will be loaded from memory.
///
/// See also [`dc_flush_slice`], which is the opposite operation.
#[inline]
pub unsafe fn dc_invalidate_slice(slice: &[impl Sized]) {
    nds_sys::cache::DC_InvalidateRange(slice.as_ptr() as _, size_of_val(slice))
}

/// Invalidates the data cache for the given array.
///
/// # Safety
///
/// If the contents of the array in cache and memory are different, the
/// next time the slice is accessed, a new copy will be loaded from memory.
///
/// See also [`dc_flush_slice`], which is the opposite operation.
#[inline]
pub unsafe fn dc_invalidate_array<const N: usize>(array: &[impl Sized; N]) {
    dc_invalidate_slice(array)
}
