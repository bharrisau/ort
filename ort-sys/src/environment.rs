use std::ffi::CString;

use crate::sys::OrtEnv;
use crate::{onnx_call, Api, Error, Result};

pub enum EnvOptions<'a> {
	Singleton,
	Default(LogLevel, &'a str)
}

#[repr(u32)]
pub enum LogLevel {
	#[doc = "< Verbose informational messages (least severe)."]
	Verbose = crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_VERBOSE,
	#[doc = "< Informational messages."]
	Info = crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_INFO,
	#[doc = "< Warning messages."]
	Warning = crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_WARNING,
	#[doc = "< Error messages."]
	Error = crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_ERROR,
	#[doc = "< Fatal error messages (most severe)."]
	Fatal = crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_FATAL
}

#[derive(Debug)]
#[repr(transparent)]
pub struct EnvPtr {
	ptr: *mut OrtEnv
}

unsafe impl Send for EnvPtr {}
// unsafe impl Sync for EnvPtr {}

impl Api {
	pub fn env_create(&self, log_severity_level: LogLevel, logid: &str) -> Result<EnvPtr> {
		let logid = CString::new(logid).expect("invalid log identifier");
		let mut env_ptr: *mut OrtEnv = std::ptr::null_mut();

		onnx_call!(self.api, CreateEnv(log_severity_level as _, logid.as_ptr(), &mut env_ptr))?;

		Ok(env_ptr.into())
	}

	pub fn env_release(&self, env: &mut EnvPtr) {
		if !env.ptr.is_null() {
			onnx_call!(self.api, ReleaseEnv(env.ptr) -> ()).expect("unable to release OrtEnv");
			env.ptr = std::ptr::null_mut();
		}
	}
}

impl From<*mut OrtEnv> for EnvPtr {
	fn from(value: *mut OrtEnv) -> Self {
		EnvPtr { ptr: value }
	}
}

impl Drop for EnvPtr {
	fn drop(&mut self) {
		Api::get().env_release(self);
	}
}
