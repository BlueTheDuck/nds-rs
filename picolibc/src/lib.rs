#![no_std]

pub mod bindings {
    #![allow(warnings)]
    
    include!(concat!(env!("OUT_DIR"), "/picolibc.rs"));
}

#[inline]
pub fn write_str_to_stderr(s: &str) -> usize {
    let buffer = s.as_bytes();
    unsafe {
        bindings::fwrite(buffer.as_ptr() as _, buffer.len() as _, 1, bindings::stderr) as _
    }
}
