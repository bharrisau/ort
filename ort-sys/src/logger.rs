use std::{
	any::Any,
	ffi::{c_void, CStr}
};

use ::std::os::raw::c_char;

use crate::sys::{OrtLoggingFunction, OrtLoggingLevel};
pub trait Logger: std::panic::RefUnwindSafe {
	fn log(&mut self, severity: LogLevel, category: &str, logid: &str, code_location: &str, message: &str);
	fn catch_unwind(&self, err: Box<dyn Any + Send + 'static>) {
		eprintln!("Panic in logger function: {:?}", err);
		std::process::abort();
	}
}

#[derive(Debug)]
pub(crate) struct LoggerPtr {
	pub(crate) ptr: *mut c_void,
	pub(crate) log: OrtLoggingFunction,
	destroy: unsafe fn(*mut c_void)
}

impl LoggerPtr {
	pub fn for_logger<L: Logger + 'static>(logger: L) -> Self {
		LoggerPtr {
			ptr: Box::into_raw(Box::new(logger)) as _,
			log: Some(log_raw::<L>),
			destroy: destroy::<L>
		}
	}
}

impl Drop for LoggerPtr {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			let destroy = self.destroy;
			unsafe { destroy(self.ptr) }
			self.ptr = std::ptr::null_mut();
		}
	}
}

unsafe fn destroy<L>(handle: *mut c_void) {
	let repr = handle as *mut L;
	let _ = Box::from_raw(repr);
}

unsafe extern "C" fn log_raw<L: Logger>(
	param: *mut ::std::os::raw::c_void,
	severity: OrtLoggingLevel,
	category: *const c_char,
	logid: *const c_char,
	code_location: *const c_char,
	message: *const c_char
) {
	let handle = param as *mut L;

	let result = std::panic::catch_unwind(|| {
		let logger = &mut *handle;
		let category = std::str::from_utf8_unchecked(CStr::from_ptr(category).to_bytes());
		let logid = std::str::from_utf8_unchecked(CStr::from_ptr(logid).to_bytes());
		let code_location = std::str::from_utf8_unchecked(CStr::from_ptr(code_location).to_bytes());
		let message = std::str::from_utf8_unchecked(CStr::from_ptr(message).to_bytes());

		logger.log(severity.into(), category, logid, code_location, message);
	});

	if let Err(e) = result {
		let logger = &mut *handle;
		logger.catch_unwind(e);
	}
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

impl From<LogLevel> for u32 {
	fn from(value: LogLevel) -> Self {
		value as Self
	}
}

impl From<u32> for LogLevel {
	fn from(value: u32) -> Self {
		match value {
			crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_VERBOSE => LogLevel::Verbose,
			crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_INFO => LogLevel::Info,
			crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_WARNING => LogLevel::Warning,
			crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_ERROR => LogLevel::Error,
			_ => LogLevel::Fatal
		}
	}
}
