#![no_std]

use core::{ffi::CStr, ptr::NonNull};

use bindings::FILE;

unsafe extern "C" {
    #[link_name = "stdin"]
    pub safe static STDIN: NonNull<FILE>;
    #[link_name = "stdout"]
    pub safe static STDOUT: NonNull<FILE>;
    #[link_name = "stderr"]
    pub safe static STDERR: NonNull<FILE>;
}

pub mod bindings {
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/picolibc.rs"));
}

#[inline]
pub fn printf(s: &CStr) -> usize {
    unsafe { bindings::printf(s.as_ptr() as _) as _ }
}

pub fn fwrite(s: &[u8], stream: NonNull<FILE>) -> usize {
    unsafe { bindings::fwrite(s.as_ptr() as _, s.len() as _, 1, stream.as_ptr()) as _ }
}
