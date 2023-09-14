use std::ffi::CString;

use crate::logger::{log_function, Logger, LoggerHandle};
use crate::sys::{OrtEnv, OrtThreadingOptions};
use crate::{onnx_call, Api, Error, Result};

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

#[derive(Debug)]
pub struct EnvPtr {
	ptr: *mut OrtEnv,
	logger: Option<*mut LoggerHandle>,
	tp_options: Option<ThreadingOptions>
}

unsafe impl Send for EnvPtr {}
unsafe impl Sync for EnvPtr {}

// pub EnableTelemetryEvents: ::std::option::Option<crate::ctypes::_system!(unsafe fn(env: *const OrtEnv) ->
// OrtStatusPtr)>, pub DisableTelemetryEvents: ::std::option::Option<crate::ctypes::_system!(unsafe fn(env: *const
// OrtEnv) -> OrtStatusPtr)>,

// pub CreateThreadingOptions:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(out: *mut *mut OrtThreadingOptions) -> OrtStatusPtr)>,
// pub ReleaseThreadingOptions: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut
// OrtThreadingOptions))>,

// pub SetGlobalIntraOpNumThreads: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(tp_options: *mut OrtThreadingOptions, intra_op_num_threads:
// ::std::os::raw::c_int) -> OrtStatusPtr) >,
// pub SetGlobalInterOpNumThreads: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(tp_options: *mut OrtThreadingOptions, inter_op_num_threads:
// ::std::os::raw::c_int) -> OrtStatusPtr) >,
// pub SetGlobalSpinControl:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(tp_options: *mut OrtThreadingOptions, allow_spinning:
// ::std::os::raw::c_int) -> OrtStatusPtr)>,

// pub SetGlobalDenormalAsZero: ::std::option::Option<crate::ctypes::_system!(unsafe fn(tp_options: *mut
// OrtThreadingOptions) -> OrtStatusPtr)>, pub CreateArenaCfg: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             max_mem: size_t,
//             arena_extend_strategy: ::std::os::raw::c_int,
//             initial_chunk_size_bytes: ::std::os::raw::c_int,
//             max_dead_bytes_per_chunk: ::std::os::raw::c_int,
//             out: *mut *mut OrtArenaCfg
//         ) -> OrtStatusPtr
//     )
// >,
// pub SetGlobalIntraOpThreadAffinity: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(tp_options: *mut OrtThreadingOptions, affinity_string: *const
// ::std::os::raw::c_char) -> OrtStatusPtr) >,
// pub SetGlobalCustomCreateThreadFn: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(tp_options: *mut OrtThreadingOptions, ort_custom_create_thread_fn:
// OrtCustomCreateThreadFn) -> OrtStatusPtr) >,
// pub SetGlobalCustomThreadCreationOptions: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(tp_options: *mut OrtThreadingOptions, ort_custom_thread_creation_options: *mut
// ::std::os::raw::c_void) -> OrtStatusPtr     )
// >,
// pub SetGlobalCustomJoinThreadFn: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(tp_options: *mut OrtThreadingOptions, ort_custom_join_thread_fn:
// OrtCustomJoinThreadFn) -> OrtStatusPtr) >,
// pub CreateAndRegisterAllocator: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(env: *mut OrtEnv, mem_info: *const OrtMemoryInfo, arena_cfg: *const
// OrtArenaCfg) -> OrtStatusPtr) >,
// pub SetLanguageProjection:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(ort_env: *const OrtEnv, projection:
// OrtLanguageProjection) -> OrtStatusPtr)>

// pub ReleaseArenaCfg: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtArenaCfg))>,

// pub CreateArenaCfgV2: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             arena_config_keys: *const *const ::std::os::raw::c_char,
//             arena_config_values: *const size_t,
//             num_keys: size_t,
//             out: *mut *mut OrtArenaCfg
//         ) -> OrtStatusPtr
//     )
// >,
// pub RegisterAllocator:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(env: *mut OrtEnv, allocator: *mut OrtAllocator) ->
// OrtStatusPtr)>, pub UnregisterAllocator: ::std::option::Option<crate::ctypes::_system!(unsafe fn(env: *mut OrtEnv,
// mem_info: *const OrtMemoryInfo) -> OrtStatusPtr)>,

impl Api {
	/// Create a new OrtEnv
	pub fn env_create(&self, log_severity_level: LogLevel, logid: &str) -> Result<EnvPtr> {
		let logid = CString::new(logid).expect("invalid log identifier");
		let mut env_ptr: *mut OrtEnv = std::ptr::null_mut();

		onnx_call!(self.api, CreateEnv(log_severity_level.into(), logid.as_ptr(), &mut env_ptr))?;

		Ok(EnvPtr {
			ptr: env_ptr,
			logger: None,
			tp_options: None
		})
	}

	/// Create a new OrtEnv with logger
	pub fn env_create_with_custom_logger<T: Logger + 'static>(&self, logger: T, log_severity_level: LogLevel, logid: &str) -> Result<EnvPtr> {
		let logid = CString::new(logid).expect("invalid log identifier");
		let mut env_ptr: *mut OrtEnv = std::ptr::null_mut();

		let param = LoggerHandle::for_logger(logger);

		onnx_call!(self.api, CreateEnvWithCustomLogger(Some(log_function), param as _, log_severity_level.into(), logid.as_ptr(), &mut env_ptr))?;

		Ok(EnvPtr {
			ptr: env_ptr,
			logger: Some(param),
			tp_options: None
		})
	}

	/// Create a new OrtEnv with logger and threadpool
	pub fn env_create_with_custom_logger_and_global_thread_pools<T: Logger + 'static>(
		&self,
		logger: T,
		log_severity_level: LogLevel,
		logid: &str,
		tp_options: ThreadingOptions
	) -> Result<EnvPtr> {
		let logid = CString::new(logid).expect("invalid log identifier");
		let mut env_ptr: *mut OrtEnv = std::ptr::null_mut();

		let param = LoggerHandle::for_logger(logger);

		onnx_call!(
			self.api,
			CreateEnvWithCustomLoggerAndGlobalThreadPools(
				Some(log_function),
				param as _,
				log_severity_level.into(),
				logid.as_ptr(),
				tp_options.as_ptr(),
				&mut env_ptr
			)
		)?;

		Ok(EnvPtr {
			ptr: env_ptr,
			logger: Some(param),
			tp_options: Some(tp_options)
		})
	}

	/// Create a new OrtEnv with threadpool
	pub fn env_create_with_global_thread_pools(&self, log_severity_level: LogLevel, logid: &str, tp_options: ThreadingOptions) -> Result<EnvPtr> {
		let logid = CString::new(logid).expect("invalid log identifier");
		let mut env_ptr: *mut OrtEnv = std::ptr::null_mut();

		onnx_call!(self.api, CreateEnvWithGlobalThreadPools(log_severity_level.into(), logid.as_ptr(), tp_options.as_ptr(), &mut env_ptr))?;

		Ok(EnvPtr {
			ptr: env_ptr,
			logger: None,
			tp_options: Some(tp_options)
		})
	}

	/// Release the allocation for this OrtEnv
	pub(crate) fn env_release(&self, env: &mut EnvPtr) {
		if !env.ptr.is_null() {
			onnx_call!(self.api, ReleaseEnv(env.ptr) -> ()).expect("unable to release OrtEnv");
			env.ptr = std::ptr::null_mut();
		}
	}
}

impl Drop for EnvPtr {
	fn drop(&mut self) {
		Api::get().env_release(self);

		if let Some(logger) = self.logger.take() {
			LoggerHandle::destroy(logger);
		}
	}
}

#[derive(Debug)]
pub struct ThreadingOptions {
	ptr: *mut OrtThreadingOptions
}

impl ThreadingOptions {
	fn as_ptr(&self) -> *const OrtThreadingOptions {
		todo!()
	}
}
