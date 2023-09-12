use std::panic::Location;

use thiserror::Error as ThisError;

use crate::{sys::*, Api};

pub type Result<T> = std::result::Result<T, Error>;

/// Errors.
#[derive(ThisError, Debug)]
#[non_exhaustive]
pub enum Error {
	#[error("unknown error")]
	Unknown,
	#[error("api versions do not match - requested {requested}, loaded {loaded}")]
	ApiVersionMismatch { loaded: u32, requested: u32 },
	#[error("unable to load requested api - requested {requested}, onnxruntime {version}")]
	ApiVersionUnavailable { requested: u32, version: &'static str },
	#[error("API function {call} unavailable")]
	ApiUnavailable { call: &'static str },
	#[error("onnxruntime error {status}")]
	OnnxError {
		#[from]
		status: Status
	},

	#[error("unknown error")]
	Unknown5,
	#[error("unknown error")]
	Unknown6,
	#[error("unknown error")]
	Unknown7,
	#[cfg(feature = "load-dynamic")]
	#[error(transparent)]
	LibLoading(#[from] libloading::Error)
}

#[derive(Debug)]
#[repr(u32)]
pub enum ErrorCode {
	Ok = OrtErrorCode_ORT_OK,
	Fail = OrtErrorCode_ORT_FAIL,
	InvalidArgument = OrtErrorCode_ORT_INVALID_ARGUMENT,
	NoSuchFile = OrtErrorCode_ORT_NO_SUCHFILE,
	NoModel = OrtErrorCode_ORT_NO_MODEL,
	EngineError = OrtErrorCode_ORT_ENGINE_ERROR,
	RuntimeException = OrtErrorCode_ORT_RUNTIME_EXCEPTION,
	InvalidProtobuf = OrtErrorCode_ORT_INVALID_PROTOBUF,
	ModelLoaded = OrtErrorCode_ORT_MODEL_LOADED,
	NotImplemented = OrtErrorCode_ORT_NOT_IMPLEMENTED,
	InvalidGraph = OrtErrorCode_ORT_INVALID_GRAPH,
	EpFail = OrtErrorCode_ORT_EP_FAIL,
	Unknown
}

#[allow(non_upper_case_globals)]
impl From<u32> for ErrorCode {
	fn from(value: u32) -> Self {
		match value {
			OrtErrorCode_ORT_OK => Self::Ok,
			OrtErrorCode_ORT_FAIL => Self::Fail,
			OrtErrorCode_ORT_INVALID_ARGUMENT => Self::InvalidArgument,
			OrtErrorCode_ORT_NO_SUCHFILE => Self::NoSuchFile,
			OrtErrorCode_ORT_NO_MODEL => Self::NoModel,
			OrtErrorCode_ORT_ENGINE_ERROR => Self::EngineError,
			OrtErrorCode_ORT_RUNTIME_EXCEPTION => Self::RuntimeException,
			OrtErrorCode_ORT_INVALID_PROTOBUF => Self::InvalidProtobuf,
			OrtErrorCode_ORT_MODEL_LOADED => Self::ModelLoaded,
			OrtErrorCode_ORT_NOT_IMPLEMENTED => Self::NotImplemented,
			OrtErrorCode_ORT_INVALID_GRAPH => Self::InvalidGraph,
			OrtErrorCode_ORT_EP_FAIL => Self::EpFail,
			_ => Self::Unknown
		}
	}
}

#[derive(Debug)]
pub struct Status {
	ptr: *mut OrtStatus,
	call: &'static str,
	loc: &'static Location<'static>
}

impl Status {
	pub fn new(ptr: *mut OrtStatus, call: &'static str, loc: &'static Location<'static>) -> Self {
		Self { ptr, call, loc }
	}
	pub fn message(&self) -> &str {
		Api::get().status_get_message(self.ptr)
	}

	pub fn code(&self) -> ErrorCode {
		Api::get().status_get_code(self.ptr).into()
	}
}

impl std::fmt::Display for Status {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("code: {:?} - message {}", self.code(), self.message()))
	}
}

impl Drop for Status {
	fn drop(&mut self) {
		Api::get().status_release(self.ptr);
	}
}

impl std::error::Error for Status {}
