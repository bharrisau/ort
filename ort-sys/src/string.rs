use std::{cell::RefCell, ffi::c_char, rc::Rc, sync::Arc};

pub(crate) enum OnnxString {
	Rc(Rc<str>),
	Arc(Arc<str>)
}

thread_local! {
	static CACHE: RefCell<Vec<u8>> = Default::default();
}

impl OnnxString {
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
				vec.clear();
				vec.reserve(value.len() + 1);
				vec.extend(value.as_bytes());
				vec.push(0);

				(vec.as_ptr() as _, vec.len())
			})
		}
	}

	pub fn to_rc(value: &str) -> Self {
		if let Some(i) = value.as_bytes().iter().position(|&b| b == 0) {
			panic!("string includes null at position {} - {}", i, value)
		}

		let rc: Rc<str> = CACHE.with(|cache| {
			let mut vec = cache.borrow_mut();
			vec.clear();
			vec.reserve(value.len() + 1);
			vec.extend(value.as_bytes());
			vec.push(0);

			unsafe { std::str::from_utf8_unchecked(&vec).into() }
		});

		Self::Rc(rc)
	}

	pub fn to_arc(value: &str) -> Self {
		if let Some(i) = value.as_bytes().iter().position(|&b| b == 0) {
			panic!("string includes null at position {} - {}", i, value)
		}

		let rc: Arc<str> = CACHE.with(|cache| {
			let mut vec = cache.borrow_mut();
			vec.clear();
			vec.reserve(value.len() + 1);
			vec.extend(value.as_bytes());
			vec.push(0);

			unsafe { std::str::from_utf8_unchecked(&vec).into() }
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
