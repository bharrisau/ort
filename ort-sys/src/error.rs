use thiserror::Error as ThisError;

use crate::{status::ErrorCode, Status};

pub type Result<T> = std::result::Result<T, Error>;

/// Errors.
#[derive(ThisError, Debug)]
#[non_exhaustive]
pub enum Error {
	#[error("unknown error")]
	Unknown,
	#[error("infallible")]
	Infallible(#[from] std::convert::Infallible),
	#[error("api versions do not match - requested {requested}, loaded {loaded}")]
	ApiVersionMismatch { loaded: u32, requested: u32 },
	#[error("unable to load requested api - requested {requested}, onnxruntime {version}")]
	ApiVersionUnavailable { requested: u32, version: &'static str },
	#[error("API function {call} unavailable")]
	ApiUnavailable { call: &'static str },
	#[cfg(feature = "load-dynamic")]
	#[error(transparent)]
	LibLoading(#[from] libloading::Error),

	// #[error("onnxruntime error {status}")]
	// OnnxError {
	// 	#[from]
	// 	status: Status
	// },
	#[error(transparent)]
	OrtOk(Status),
	#[error(transparent)]
	OrtFail(Status),
	#[error(transparent)]
	OrtInvalidArgument(Status),
	#[error(transparent)]
	OrtNoSuchFile(Status),
	#[error(transparent)]
	OrtNoModel(Status),
	#[error(transparent)]
	OrtEngineError(Status),
	#[error(transparent)]
	OrtRuntimeException(Status),
	#[error(transparent)]
	OrtInvalidProtobuf(Status),
	#[error(transparent)]
	OrtModelLoaded(Status),
	#[error(transparent)]
	OrtNotImplemented(Status),
	#[error(transparent)]
	OrtInvalidGraph(Status),
	#[error(transparent)]
	OrtEpFail(Status),
	#[error(transparent)]
	OrtUnknown(Status)
}

impl From<Status> for Error {
	fn from(value: Status) -> Self {
		match value.code() {
			ErrorCode::Ok => Error::OrtOk(value),
			ErrorCode::Fail => Error::OrtFail(value),
			ErrorCode::InvalidArgument => Error::OrtInvalidArgument(value),
			ErrorCode::NoSuchFile => Error::OrtNoSuchFile(value),
			ErrorCode::NoModel => Error::OrtNoModel(value),
			ErrorCode::EngineError => Error::OrtEngineError(value),
			ErrorCode::RuntimeException => Error::OrtRuntimeException(value),
			ErrorCode::InvalidProtobuf => Error::OrtInvalidProtobuf(value),
			ErrorCode::ModelLoaded => Error::OrtModelLoaded(value),
			ErrorCode::NotImplemented => Error::OrtNotImplemented(value),
			ErrorCode::InvalidGraph => Error::OrtInvalidGraph(value),
			ErrorCode::EpFail => Error::OrtEpFail(value),
			ErrorCode::Unknown => Error::OrtUnknown(value)
		}
	}
}
