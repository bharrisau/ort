use std::{ffi::CStr, panic::Location};

use crate::{onnx_call, sys::OrtStatus, Api};

impl Api {
	/// Create an OrtStatus from a null terminated string
	pub(crate) fn status_create(&self, code: ErrorCode, msg: impl AsRef<CStr>) -> Status {
		let ptr = onnx_call!(self.api, CreateStatus(code.into(), msg.as_ref().as_ptr()) -> *mut OrtStatus).expect("unable to create OrtStatus");
		Status::Custom { ptr }
	}

	/// Get OrtErrorCode from OrtStatus
	pub fn status_get_code(&self, status: &Status) -> ErrorCode {
		let ptr = match status {
			Status::Ok => return ErrorCode::Ok,
			Status::Custom { ptr } => ptr,
			Status::Fail { ptr, .. } => ptr
		};

		onnx_call!(self.api, GetErrorCode(*ptr) -> crate::sys::OrtErrorCode)
			.expect("unable to get error code")
			.into()
	}

	/// Get error string from OrtStatus
	pub fn status_get_message<'a>(&self, status: &Status) -> &'a str {
		let ptr = match status {
			Status::Ok => return "ok",
			Status::Custom { ptr } => ptr,
			Status::Fail { ptr, .. } => ptr
		};

		let ret = onnx_call!(self.api, GetErrorMessage(*ptr) -> *const i8).expect("unable to get error message");
		unsafe {
			let ret = CStr::from_ptr(ret);
			std::str::from_utf8_unchecked(ret.to_bytes())
		}
	}

	/// Release allocation for OrtStatus
	pub(crate) fn status_release(&self, status: &mut Status) {
		let ptr = match status {
			Status::Ok => return,
			Status::Custom { ptr } => ptr,
			Status::Fail { ptr, .. } => ptr
		};

		if !ptr.is_null() {
			onnx_call!(self.api, ReleaseStatus(*ptr) -> ()).expect("unable to release OrtStatus");
			*ptr = std::ptr::null_mut();
		}
	}
}

#[derive(Debug)]
#[repr(u32)]
pub enum ErrorCode {
	Ok = crate::sys::OrtErrorCode_ORT_OK,
	Fail = crate::sys::OrtErrorCode_ORT_FAIL,
	InvalidArgument = crate::sys::OrtErrorCode_ORT_INVALID_ARGUMENT,
	NoSuchFile = crate::sys::OrtErrorCode_ORT_NO_SUCHFILE,
	NoModel = crate::sys::OrtErrorCode_ORT_NO_MODEL,
	EngineError = crate::sys::OrtErrorCode_ORT_ENGINE_ERROR,
	RuntimeException = crate::sys::OrtErrorCode_ORT_RUNTIME_EXCEPTION,
	InvalidProtobuf = crate::sys::OrtErrorCode_ORT_INVALID_PROTOBUF,
	ModelLoaded = crate::sys::OrtErrorCode_ORT_MODEL_LOADED,
	NotImplemented = crate::sys::OrtErrorCode_ORT_NOT_IMPLEMENTED,
	InvalidGraph = crate::sys::OrtErrorCode_ORT_INVALID_GRAPH,
	EpFail = crate::sys::OrtErrorCode_ORT_EP_FAIL,
	Unknown
}

#[allow(non_upper_case_globals)]
impl From<u32> for ErrorCode {
	fn from(value: u32) -> Self {
		match value {
			crate::sys::OrtErrorCode_ORT_OK => Self::Ok,
			crate::sys::OrtErrorCode_ORT_FAIL => Self::Fail,
			crate::sys::OrtErrorCode_ORT_INVALID_ARGUMENT => Self::InvalidArgument,
			crate::sys::OrtErrorCode_ORT_NO_SUCHFILE => Self::NoSuchFile,
			crate::sys::OrtErrorCode_ORT_NO_MODEL => Self::NoModel,
			crate::sys::OrtErrorCode_ORT_ENGINE_ERROR => Self::EngineError,
			crate::sys::OrtErrorCode_ORT_RUNTIME_EXCEPTION => Self::RuntimeException,
			crate::sys::OrtErrorCode_ORT_INVALID_PROTOBUF => Self::InvalidProtobuf,
			crate::sys::OrtErrorCode_ORT_MODEL_LOADED => Self::ModelLoaded,
			crate::sys::OrtErrorCode_ORT_NOT_IMPLEMENTED => Self::NotImplemented,
			crate::sys::OrtErrorCode_ORT_INVALID_GRAPH => Self::InvalidGraph,
			crate::sys::OrtErrorCode_ORT_EP_FAIL => Self::EpFail,
			_ => Self::Unknown
		}
	}
}

impl From<ErrorCode> for u32 {
	fn from(val: ErrorCode) -> Self {
		val as Self
	}
}

#[derive(Debug)]
pub enum Status {
	Ok,
	Custom {
		ptr: *mut OrtStatus
	},
	Fail {
		ptr: *mut OrtStatus,
		call: &'static str,
		loc: &'static Location<'static>
	}
}

impl Status {
	pub(crate) fn new(ptr: *mut OrtStatus, call: &'static str, loc: &'static Location<'static>) -> Self {
		Self::Fail { ptr, call, loc }
	}
	pub fn message(&self) -> &str {
		Api::get().status_get_message(self)
	}

	pub fn code(&self) -> ErrorCode {
		Api::get().status_get_code(self)
	}
}

impl std::fmt::Display for Status {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Status::Ok => f.write_str("onnxruntime OK status"),
			Status::Custom { .. } => f.write_fmt(format_args!("onnxruntime custom status. error code: {:?}, error message {}", self.code(), self.message())),
			Status::Fail { call, loc, .. } => f.write_fmt(format_args!(
				"onnxruntime error while calling {} at {}. error code: {:?}, error message {}",
				call,
				loc,
				self.code(),
				self.message()
			))
		}
	}
}

impl Drop for Status {
	fn drop(&mut self) {
		Api::get().status_release(self);
	}
}

impl std::error::Error for Status {}
