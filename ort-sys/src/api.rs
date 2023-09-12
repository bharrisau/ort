#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::{
	ffi::{CStr, CString},
	ops::Deref
};

use once_cell::sync::OnceCell;

use crate::{error::Status, sys::*};
use crate::{Error, Result};

#[derive(Debug)]
struct ApiPtr {
	api_ptr: *const OrtApi,
	version: u32,
	version_str: &'static str,

	#[cfg(feature = "load-dynamic")]
	lib: libloading::Library
}

unsafe impl Send for ApiPtr {}
unsafe impl Sync for ApiPtr {}

impl Deref for ApiPtr {
	type Target = OrtApi;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.api_ptr }
	}
}

impl ApiPtr {
	pub fn get_version(version: u32) -> Result<&'static ApiPtr> {
		static G_ORT_API: OnceCell<ApiPtr> = OnceCell::new();

		G_ORT_API.get_or_try_init(|| Self::open(version)).and_then(|api| {
			if version == 0 || version == api.version {
				Ok(api)
			} else {
				Err(Error::ApiVersionMismatch {
					loaded: api.version,
					requested: version
				})
			}
		})
	}

	pub fn get() -> &'static ApiPtr {
		Self::get_version(0).unwrap()
	}

	#[cfg(feature = "load-dynamic")]
	fn open(version: u32) -> Result<ApiPtr> {
		use std::ffi::OsStr;

		use libloading::{Library, Symbol};

		let version = match version {
			0 => ORT_API_VERSION,
			x => x
		};

		unsafe {
			#[cfg(target_os = "windows")]
			let default_path = OsStr::new("onnxruntime.dll");
			#[cfg(any(target_os = "linux", target_os = "android"))]
			let default_path = OsStr::new("libonnxruntime.so");
			#[cfg(target_os = "macos")]
			let default_path = OsStr::new("libonnxruntime.dylib");
			let env_path = std::env::var_os("ORT_DYLIB_PATH");

			let path = env_path.as_deref().unwrap_or(default_path);
			let lib = Library::new(path)?;

			let base_getter: Symbol<unsafe extern "C" fn() -> *const OrtApiBase> = lib.get(b"OrtGetApiBase")?;
			let base = base_getter();
			assert!(!base.is_null());

			let version_str = (*base).GetVersionString.unwrap()();
			let version_str = CStr::from_ptr(version_str);
			let version_str = std::str::from_utf8_unchecked(version_str.to_bytes());

			let api_ptr = (*base).GetApi.unwrap()(version);
			if api_ptr.is_null() {
				Err(Error::ApiVersionUnavailable {
					requested: version,
					version: version_str
				})
			} else {
				Ok(ApiPtr { api_ptr, version, version_str, lib })
			}
		}
	}

	#[cfg(not(feature = "load-dynamic"))]
	fn open(version: u32) -> Result<ApiPtr, std::io::Error> {
		let version = match version {
			0 => ORT_API_VERSION,
			x => x
		};

		unsafe {
			let base = OrtGetApiBase();
			assert!(!base.is_null());

			let version_str = (*base).GetVersionString.unwrap()();
			let version_str = CStr::from_ptr(version_str);
			let version_str = std::str::from_utf8_unchecked(version_str.to_bytes());

			let api_ptr = (*base).GetApi.unwrap()(version);
			if api_ptr.is_null() {
				Err(Error::ApiVersionUnavailable {
					requested: version,
					version: version_str
				})
			} else {
				Ok(ApiPtr { api_ptr, version, version_str })
			}
		}
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
					// Generate the error from the OrtStatus and release it after
					Err(Status::new(result, stringify!($call), std::panic::Location::caller()).into())
				}
			}
		} else {
			Err(Error::ApiUnavailable { call: stringify!($call) })
		}
	};
    (@no_return $api:expr, $call:ident($($n:expr),+ $(,)?)) => {
		if let Some(f) = $api.$call {
			unsafe {
				f($($n),+);
                Ok(())
			}
		} else {
			Err(Error::ApiUnavailable { call: stringify!($call) })
		}
	};
    (@direct $api:expr, $call:ident($($n:expr),+ $(,)?)) => {
		if let Some(f) = $api.$call {
			unsafe {
				Ok(f($($n),+))
			}
		} else {
			Err(Error::ApiUnavailable { call: stringify!($call) })
		}
	};
}

#[repr(transparent)]
pub struct Api {
	api: &'static ApiPtr
}

impl Api {
	pub fn get() -> Self {
		Api { api: ApiPtr::get() }
	}

	pub fn status_get_code(&self, status: *const OrtStatus) -> OrtErrorCode {
		onnx_call!(@direct self.api, GetErrorCode(status)).unwrap()
	}

	pub fn status_get_message<'a>(&self, status: *const OrtStatus) -> &'a str {
		let ret = onnx_call!(@direct self.api, GetErrorMessage(status)).unwrap();
		unsafe {
			let ret = CStr::from_ptr(ret);
			std::str::from_utf8_unchecked(ret.to_bytes())
		}
	}

	pub fn status_release(&self, status: *mut OrtStatus) {
		onnx_call!(@no_return self.api, ReleaseStatus(status)).unwrap();
	}

	pub fn env_create(&self, log_severity_level: OrtLoggingLevel, logid: &str) -> Result<*mut OrtEnv> {
		let logid = CString::new(logid).unwrap();
		let mut env_ptr: *mut OrtEnv = std::ptr::null_mut();

		onnx_call!(self.api, CreateEnv(log_severity_level, logid.as_ptr(), &mut env_ptr))?;

		Ok(env_ptr)
	}

	pub fn env_release(&self, env: *mut OrtEnv) {
		onnx_call!(@no_return self.api, ReleaseEnv(env)).unwrap();
	}
}
