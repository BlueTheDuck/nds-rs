macro_rules! bit {
    ($shift: literal) => {
        (1 << $shift)
    };
}

#[doc(hidden)]
#[repr(C)]
pub struct AlignedSlice<T, B: ?Sized> {
    pub _alignment: [T; 0],
    pub data: B,
}

/// Includes the contents of a file as a statically allocated array with specified alignment.
///
/// # Arguments
///
/// * `$alignment` - The alignment type to ensure for the byte slice.
/// * `$file` - The path to the file to include, see [`include_bytes!`](https://doc.rust-lang.org/core/macro.include_bytes.html)
///
/// # Returns
///
/// A reference to the aligned byte slice containing the included bytes.
///
/// # Examples
///
/// ```rust,no_run
/// // Including the contents of "data.bin" as a statically allocated byte slice with 4-byte alignment.
/// static DATA: &'static [u8] = include_bytes_aligned!(u32, "data.bin");
/// ```
///
#[macro_export]
macro_rules! include_bytes_aligned {
    ($alignment:ty, $file:expr) => {{
        static ALIGNED: &$crate::macros::AlignedSlice<$alignment, [u8]> =
            &$crate::macros::AlignedSlice {
                _alignment: [],
                data: *include_bytes!($file),
            };
        &ALIGNED.data
    }};
}

/// Includes the contents of a file as a statically allocated array, casting it to
/// an array of the specified type. While alignment is guaranteed
/// (Check [`include_bytes_aligned!`]), the size of the array is rounded down
/// to ensure it does not span past the end of the array.
///
/// # Arguments
///
/// * `$type` - The type to cast the included bytes as.
/// * `$file` - The path to the file to include, see [`include_bytes!`](https://doc.rust-lang.org/core/macro.include_bytes.html).
///
/// # Examples
///
/// ```no_run
/// // Including the contents of "data.bin" as a slice of u32.
/// static DATA: &'static [u32] = include_bytes_as!(u32, "data.bin");
/// ```
/// In the above example, if the file is 10 bytes long, the array will be 2 elements
/// long, leaving the last 2 bytes unreacheable.
///
/// This macro is useful for including data that can be directly used by the hardware, such as palettes or bitmaps, or copied using [DMA](crate::dma::copy()).
#[macro_export]
macro_rules! include_bytes_as {
    ($type:ty, $file:expr) => {{
        static ALIGNED: &'static [u8] = include_bytes_aligned!($type, $file);
        // # Safety
        // The aligment is guaranteed by the `include_bytes_aligned!` macro above,
        // and the size of the array is rounded down so it will never span past the
        // end of the array, but it might leave a few bytes unreacheable.
        unsafe {
            core::slice::from_raw_parts(
                ALIGNED.as_ptr() as *const $type,
                ALIGNED.len() / core::mem::size_of::<$type>(),
            )
        }
    }};
}
