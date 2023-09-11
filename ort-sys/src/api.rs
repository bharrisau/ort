#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::{ffi::CString, io::Error, ops::Deref, sync::Mutex};

use crate::sys::*;

static G_ORT_API: Mutex<Option<ApiPtr>> = Mutex::new(None);

struct ApiPtr {
	ptr: *mut OrtApi
}

unsafe impl Send for ApiPtr {}

impl Deref for ApiPtr {
	type Target = OrtApi;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.ptr }
	}
}

#[repr(transparent)]
pub struct Api<'a> {
	api: &'a ApiPtr
}

impl<'a> Api<'a> {
	pub fn with<T>(f: impl FnOnce(&mut Api) -> T) -> T {
		let lock = G_ORT_API.lock().unwrap();

		if lock.is_none() {
			todo!("Open the API");
		}

		f(&mut Api { api: lock.as_ref().unwrap() })
	}

	pub fn create_env(&mut self, log_severity_level: OrtLoggingLevel, logid: &str) -> Result<*mut OrtEnv, Error> {
		let logid = CString::new(logid).unwrap();
		let mut env_ptr: *mut OrtEnv = std::ptr::null_mut();

		if let Some(f) = self.api.CreateEnv {
			unsafe {
				let result = f(log_severity_level, logid.as_ptr(), &mut env_ptr);

				if result.is_null() {
					// Success
					Ok(env_ptr)
				} else {
					// Generate the error from the OrtStatus and release it after
					// Result is an OrtStatus - can call
					todo!("Api error")
				}
			}
		} else {
			todo!("Function not available")
		}
	}

	pub fn release_env(&mut self, env: *mut OrtEnv) {
		// 	pub ReleaseEnv: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtEnv))>,
		if let Some(f) = self.api.ReleaseEnv {
			unsafe {
				f(env);
			}
		} else {
			todo!("Function not available")
		}
	}
}
