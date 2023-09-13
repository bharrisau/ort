#![allow(dead_code)]

mod ctypes;
mod error;

#[allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case, deref_nullptr)]
#[allow(clippy::all)]
mod sys;

mod api;
mod environment;
mod status;

use api::ApiPtr;
pub use error::{Error, Result};
pub use status::Status;

#[repr(transparent)]
pub struct Api {
	pub(crate) api: &'static ApiPtr
}

impl Api {
	pub fn get() -> Self {
		Api { api: ApiPtr::get() }
	}
}

macro_rules! onnx_call {
	($api:expr, $call:ident($($n:expr),+ $(,)?)) => {
		if let Some(f) = $api.$call {
			unsafe {
				let result = f($($n),+);

				if result.is_null() {
					Ok(())
				} else {
					Err(crate::status::Status::new(result, stringify!($call), std::panic::Location::caller()).into())
				}
			}
		} else {
			Err(Error::ApiUnavailable { call: stringify!($call) })
		}
	};
    ($api:expr, $call:ident($($n:expr),+ $(,)?) -> $ret:ty) => {
		if let Some(f) = $api.$call {
			let result = unsafe { f($($n),+) };
            Ok(result)
		} else {
			Err(crate::Error::ApiUnavailable { call: stringify!($call) })
		}
	};
}

pub(crate) use onnx_call;
