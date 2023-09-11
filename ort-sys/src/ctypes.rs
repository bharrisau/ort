#![allow(non_camel_case_types)]
pub use std::os::raw::{c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void};

#[cfg(target_os = "windows")]
pub type c_char = std::os::raw::c_ushort;
#[cfg(not(target_os = "windows"))]
pub type c_char = std::os::raw::c_char;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub type size_t = usize;
#[cfg(all(target_arch = "aarch64", target_os = "windows"))]
pub type size_t = c_ulonglong;
#[cfg(all(any(target_arch = "aarch64", target_arch = "arm"), not(target_os = "windows")))]
pub type size_t = c_ulong;

#[cfg(not(all(target_arch = "x86", target_os = "windows")))]
macro_rules! _system {
	($(#[$meta:meta])* fn $($tt:tt)*) => ($(#[$meta])* extern "C" fn $($tt)*);
	($(#[$meta:meta])* $vis:vis fn $($tt:tt)*) => ($(#[$meta])* $vis extern "C" fn $($tt)*);
	($(#[$meta:meta])* unsafe fn $($tt:tt)*) => ($(#[$meta])* unsafe extern "C" fn $($tt)*);
	($(#[$meta:meta])* $vis:vis unsafe fn $($tt:tt)*) => ($(#[$meta])* $vis unsafe extern "C" fn $($tt)*);
}
#[cfg(all(target_arch = "x86", target_os = "windows"))]
macro_rules! _system {
	($(#[$meta:meta])* fn $($tt:tt)*) => ($(#[$meta])* extern "stdcall" fn $($tt)*);
	($(#[$meta:meta])* $vis:vis fn $($tt:tt)*) => ($(#[$meta])* $vis extern "stdcall" fn $($tt)*);
	($(#[$meta:meta])* unsafe fn $($tt:tt)*) => ($(#[$meta])* unsafe extern "stdcall" fn $($tt)*);
	($(#[$meta:meta])* $vis:vis unsafe fn $($tt:tt)*) => ($(#[$meta])* $vis unsafe extern "stdcall" fn $($tt)*);
}

#[cfg(not(all(target_arch = "x86", target_os = "windows")))]
macro_rules! _system_block {
	($(#[$meta:meta])* fn $($tt:tt)*) => (extern "C" { $(#[$meta])* fn $($tt)* });
	($(#[$meta:meta])* $vis:vis fn $($tt:tt)*) => (extern "C" { $(#[$meta])* $vis fn $($tt)* });
	($(#[$meta:meta])* unsafe fn $($tt:tt)*) => (extern "C" { $(#[$meta])* unsafe fn $($tt)* });
	($(#[$meta:meta])* $vis:vis unsafe fn $($tt:tt)*) => (extern "C" { $(#[$meta])* $vis unsafe fn $($tt)* });
}
#[cfg(all(target_arch = "x86", target_os = "windows"))]
macro_rules! _system_block {
	($(#[$meta:meta])* fn $($tt:tt)*) => (extern "stdcall" { $(#[$meta])* fn $($tt)* });
	($(#[$meta:meta])* $vis:vis fn $($tt:tt)*) => (extern "stdcall" { $(#[$meta])* $vis fn $($tt)* });
	($(#[$meta:meta])* unsafe fn $($tt:tt)*) => (extern "stdcall" { $(#[$meta])* unsafe fn $($tt)* });
	($(#[$meta:meta])* $vis:vis unsafe fn $($tt:tt)*) => (extern "stdcall" { $(#[$meta])* $vis unsafe fn $($tt)* });
}

pub(crate) use _system;
pub(crate) use _system_block;
