use std::{cell::RefCell, ffi::c_char, path::Path, rc::Rc, sync::Arc};

use crate::ctypes::sys_char;

pub(crate) enum OnnxString {
	Rc(Rc<str>),
	Arc(Arc<str>)
}

thread_local! {
	static CACHE: RefCell<Vec<u8>> = Default::default();
}

impl OnnxString {
	pub fn prepare(capacity: usize) {
		CACHE.with(|cache| {
			let mut vec = cache.borrow_mut();
			vec.clear();
			vec.reserve(capacity + 1);
		})
	}

	pub fn to_onnx(value: &str) -> (*const c_char, usize) {
		if value.as_bytes()[value.len() - 1] == 0 {
			// Already nul terminated?
			(value.as_ptr() as _, value.len())
		} else {
			if let Some(i) = value.as_bytes().iter().position(|&b| b == 0) {
				panic!("iobinding name includes null at position {} - {}", i, value)
			}

			CACHE.with(|cache| {
				let mut vec = cache.borrow_mut();
				let start = vec.len();
				// vec.reserve(value.len() + 1);
				// TODO assert string big enough
				vec.extend(value.as_bytes());
				vec.push(0);

				(vec[start..].as_ptr() as _, value.len() + 1)
			})
		}
	}

	#[cfg(target_os = "windows")]
	pub fn to_onnx_path(value: &Path) -> Vec<sys_char> {
		use std::os::windows::ffi::OsStrExt;

		value.as_os_str().encode_wide().chain(Some(0)).collect()
	}

	#[cfg(not(target_os = "windows"))]
	pub fn to_onnx_path(value: &Path) -> Vec<sys_char> {
		use std::os::unix::ffi::OsStrExt;

		value.as_os_str().as_bytes().iter().map(|v| *v as sys_char).chain(Some(0)).collect()
	}

	pub fn to_rc(value: &str) -> Self {
		if let Some(i) = value.as_bytes().iter().position(|&b| b == 0) {
			panic!("string includes null at position {} - {}", i, value)
		}

		let rc: Rc<str> = CACHE.with(|cache| {
			let mut vec = cache.borrow_mut();
			let start = vec.len();
			// TODO assert string big enough
			// vec.reserve(value.len() + 1);
			vec.extend(value.as_bytes());
			vec.push(0);

			unsafe { std::str::from_utf8_unchecked(&vec[start..]).into() }
		});

		Self::Rc(rc)
	}

	pub fn to_arc(value: &str) -> Self {
		if let Some(i) = value.as_bytes().iter().position(|&b| b == 0) {
			panic!("string includes null at position {} - {}", i, value)
		}

		let rc: Arc<str> = CACHE.with(|cache| {
			let mut vec = cache.borrow_mut();
			let start = vec.len();
			// TODO assert string big enough
			// vec.reserve(value.len() + 1);
			vec.extend(value.as_bytes());
			vec.push(0);

			unsafe { std::str::from_utf8_unchecked(&vec[start..]).into() }
		});

		Self::Arc(rc)
	}

	pub fn as_str(&self) -> &str {
		match self {
			OnnxString::Rc(data) => &data[..data.len().saturating_sub(1)],
			OnnxString::Arc(data) => &data[..data.len().saturating_sub(1)]
		}
	}

	pub fn as_str_with_nul(&self) -> &str {
		match self {
			OnnxString::Rc(data) => data,
			OnnxString::Arc(data) => data
		}
	}

	pub fn as_ptr(&self) -> *const c_char {
		match self {
			OnnxString::Rc(data) => data.as_ptr() as *const c_char,
			OnnxString::Arc(data) => data.as_ptr() as *const c_char
		}
	}
}

impl Clone for OnnxString {
	fn clone(&self) -> Self {
		match self {
			Self::Rc(data) => Self::Rc(data.clone()),
			Self::Arc(data) => Self::Arc(data.clone())
		}
	}
}
