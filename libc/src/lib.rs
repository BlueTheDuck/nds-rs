#![no_std]
#![allow(non_camel_case_types)]

pub type c_char = u8;
pub type c_schar = i8;
pub type c_uchar = u8;

pub type c_short = i16;
pub type c_sshort = i16;
pub type c_ushort = u16;

pub type c_int = i32;
pub type c_sint = i32;
pub type c_uint = u32;

pub type c_long = i32;
pub type c_slong = i32;
pub type c_ulong = u32;

pub type c_longlong = i64;
pub type c_slonglong = i64;
pub type c_ulonglong = u64;

pub type c_void = core::ffi::c_void;
