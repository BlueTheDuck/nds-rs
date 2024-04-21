use core::mem::size_of_val;

use nds_sys::bindings;

/// Flushes the data cache to memory.
///
/// # Safety
///
/// If the contents in cache and memory are different, the
/// data in memory will be overwritten with the data in cache.
///
/// See also [`dc_invalidate_all`], which is the opposite operation.
#[inline]
pub unsafe fn dc_flush_all() {
    bindings::CP15_CleanAndFlushDCache();
}

/// Flushes the data cache to memory for the given slice.
///
/// # Safety
///
/// If the contents of the slice in cache and memory are different, the
/// data in memory will be overwritten with the data in cache.
///
/// See also [`dc_invalidate_slice`], which is the opposite operation.
#[inline]
pub unsafe fn dc_flush_slice<T>(slice: &[T]) {
    bindings::CP15_CleanAndFlushDCacheRange(slice.as_ptr() as _, size_of_val(slice));
}

/// Flushes the data cache to memory for the given array.
///
/// # Safety
///
/// If the contents of the array in cache and memory are different, the
/// data in memory will be overwritten with the data in cache.
///
/// See also [`dc_invalidate_array`], which is the opposite operation.
#[inline]
pub unsafe fn dc_flush_array<const N: usize, T>(array: &[T; N]) {
    bindings::CP15_CleanAndFlushDCacheRange(array.as_ptr() as _, size_of_val(array));
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
pub unsafe fn dc_invalidate_slice<T>(slice: &[T]) {
    bindings::CP15_FlushDCacheRange(slice.as_ptr() as _, size_of_val(slice))
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
pub unsafe fn dc_invalidate_array<const N: usize, T>(array: &[T; N]) {
    bindings::CP15_FlushDCacheRange(array.as_ptr() as _, size_of_val(array))
}
