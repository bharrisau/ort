use std::{any::TypeId, ffi::CStr};

use crate::{environment::LogLevel, sys::OrtLoggingLevel};

pub(crate) unsafe extern "C" fn log_function(
	param: *mut ::std::os::raw::c_void,
	severity: OrtLoggingLevel,
	category: *const ::std::os::raw::c_char,
	logid: *const ::std::os::raw::c_char,
	code_location: *const ::std::os::raw::c_char,
	message: *const ::std::os::raw::c_char
) {
	let handle: *mut LoggerHandle = param as _;
	let log = (*handle).log;

	let category = std::str::from_utf8_unchecked(CStr::from_ptr(category).to_bytes());
	let logid = std::str::from_utf8_unchecked(CStr::from_ptr(logid).to_bytes());
	let code_location = std::str::from_utf8_unchecked(CStr::from_ptr(code_location).to_bytes());
	let message = std::str::from_utf8_unchecked(CStr::from_ptr(message).to_bytes());

	log(handle, severity.into(), category, logid, code_location, message);
}

#[repr(C)]
pub(crate) struct LoggerHandle {
	pub(crate) type_id: TypeId,
	pub(crate) destroy: unsafe fn(*mut LoggerHandle),
	pub(crate) log: unsafe fn(*mut LoggerHandle, severity: LogLevel, category: &str, logid: &str, code_location: &str, message: &str)
}

impl LoggerHandle {
	pub fn for_logger<L: Logger + 'static>(logger: L) -> *mut Self {
		let repr = WrappedLogger {
			base: LoggerHandle::vtable::<L>(),
			logger
		};

		let boxed = Box::into_raw(Box::new(repr));

		// SAFETY: A pointer to the first field on a #[repr(C)] struct has the
		// same address as the struct itself
		boxed as *mut _
	}

	pub fn destroy(handle: *mut Self) {
		unsafe {
			let destroy = (*handle).destroy;
			destroy(handle);
		}
	}

	fn vtable<L: Logger + 'static>() -> Self {
		let type_id = TypeId::of::<L>();

		LoggerHandle {
			type_id,
			destroy: destroy::<L>,
			log: log::<L>
		}
	}
}

unsafe fn destroy<L>(handle: *mut LoggerHandle) {
	let repr = handle as *mut WrappedLogger<L>;
	let _ = Box::from_raw(repr);
}

unsafe fn log<L: Logger>(handle: *mut LoggerHandle, severity: LogLevel, category: &str, logid: &str, code_location: &str, message: &str) {
	let repr = &mut *(handle as *mut WrappedLogger<L>);
	repr.logger.log(severity, category, logid, code_location, message)
}

#[repr(C)]
pub(crate) struct WrappedLogger<L> {
	// SAFETY: The FileHandle must be the first field so we can cast between
	// *mut WrappedLogger<L> and *mut LoggerHandle
	pub(crate) base: LoggerHandle,
	pub(crate) logger: L
}

pub trait Logger {
	fn log(&mut self, severity: LogLevel, category: &str, logid: &str, code_location: &str, message: &str);
}
