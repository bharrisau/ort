use std::ffi::CString;

use crate::logger::{LogLevel, Logger, LoggerPtr};
use crate::memory_info::{ArenaConfigPtr, MemoryInfoPtr};
use crate::sys::{OrtEnv, OrtThreadingOptions};
use crate::{onnx_call, Api, Result};

impl Api {
	/// Create a new OrtEnv
	pub fn env_create(&self, log_severity_level: LogLevel, logid: impl Into<Vec<u8>>) -> Result<EnvPtr> {
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
	pub fn env_create_with_custom_logger<T: Logger + 'static>(&self, logger: T, log_severity_level: LogLevel, logid: impl Into<Vec<u8>>) -> Result<EnvPtr> {
		let logid = CString::new(logid).expect("invalid log identifier");
		let mut env_ptr: *mut OrtEnv = std::ptr::null_mut();

		// let param = LoggerHandle::for_logger(logger);
		// let param = Box::into_raw(Box::new(logger));
		let logger = LoggerPtr::for_logger(logger);

		// onnx_call!(self.api, CreateEnvWithCustomLogger(Some(log_function), param as _, log_severity_level.into(),
		// logid.as_ptr(), &mut env_ptr))?;
		onnx_call!(self.api, CreateEnvWithCustomLogger(logger.log, logger.ptr, log_severity_level.into(), logid.as_ptr(), &mut env_ptr))?;

		Ok(EnvPtr {
			ptr: env_ptr,
			logger: Some(logger),
			tp_options: None
		})
	}

	/// Enable Telemetry.
	pub fn env_enable_telemetry_events(&self, env: &EnvPtr) -> Result<()> {
		onnx_call!(self.api, EnableTelemetryEvents(env.ptr))
	}

	/// Disable Telemetry.
	pub fn env_disable_telemetry_events(&self, env: &EnvPtr) -> Result<()> {
		onnx_call!(self.api, DisableTelemetryEvents(env.ptr))
	}

	/// Release the allocation for this OrtEnv
	pub(crate) fn env_release(&self, env: &mut EnvPtr) {
		if !env.ptr.is_null() {
			onnx_call!(self.api, ReleaseEnv(env.ptr) -> ()).expect("unable to release OrtEnv");
			env.ptr = std::ptr::null_mut();
		}
	}

	/// Create a new OrtEnv with threadpool
	pub fn env_create_with_global_thread_pools(&self, log_severity_level: LogLevel, logid: impl Into<Vec<u8>>, tp_options: ThreadingOptions) -> Result<EnvPtr> {
		let logid = CString::new(logid).expect("invalid log identifier");
		let mut env_ptr: *mut OrtEnv = std::ptr::null_mut();

		onnx_call!(self.api, CreateEnvWithGlobalThreadPools(log_severity_level.into(), logid.as_ptr(), tp_options.as_ptr(), &mut env_ptr))?;

		Ok(EnvPtr {
			ptr: env_ptr,
			logger: None,
			tp_options: Some(tp_options)
		})
	}

	/// Create an allocator and register it with the OrtEnv.
	pub fn env_create_and_register_allocator(&self, env: &mut EnvPtr, mem: &MemoryInfoPtr, arena: &ArenaConfigPtr) -> Result<()> {
		onnx_call!(self.api, CreateAndRegisterAllocator(env.ptr, mem.ptr, arena.ptr))
	}

	pub fn env_set_language_projection(&self, env: &EnvPtr, projection: LanguageProjection) -> Result<()> {
		onnx_call!(self.api, SetLanguageProjection(env.ptr, projection.into()))
	}

	/// Create a new OrtEnv with logger and threadpool
	pub fn env_create_with_custom_logger_and_global_thread_pools<T: Logger + 'static>(
		&self,
		logger: T,
		log_severity_level: LogLevel,
		logid: impl Into<Vec<u8>>,
		tp_options: ThreadingOptions
	) -> Result<EnvPtr> {
		let logid = CString::new(logid).expect("invalid log identifier");
		let mut env_ptr: *mut OrtEnv = std::ptr::null_mut();

		// let param = LoggerHandle::for_logger(logger);
		let logger = LoggerPtr::for_logger(logger);

		onnx_call!(
			self.api,
			CreateEnvWithCustomLoggerAndGlobalThreadPools(logger.log, logger.ptr, log_severity_level.into(), logid.as_ptr(), tp_options.as_ptr(), &mut env_ptr)
		)?;

		Ok(EnvPtr {
			ptr: env_ptr,
			logger: Some(logger),
			tp_options: Some(tp_options)
		})
	}

	/// Create an allocation for ThreadingOptions
	pub fn env_create_threading_options(&self) -> Result<ThreadingOptions> {
		let mut ptr: *mut OrtThreadingOptions = std::ptr::null_mut();

		onnx_call!(self.api, CreateThreadingOptions(&mut ptr))?;

		Ok(ThreadingOptions { ptr })
	}

	/// Release the allocation for ThreadingOptions
	pub(crate) fn env_release_threading_options(&self, value: &mut ThreadingOptions) {
		if !value.ptr.is_null() {
			onnx_call!(self.api, ReleaseThreadingOptions(value.ptr) -> ()).expect("unable to release ThreadingOptions");
			value.ptr = std::ptr::null_mut();
		}
	}

	/// Set global intra-op thread count.
	pub fn env_set_global_intra_op_num_threads(&self, tp: ThreadingOptions, num: i32) -> Result<()> {
		onnx_call!(self.api, SetGlobalIntraOpNumThreads(tp.ptr, num))
	}

	/// Set global inter-op thread count.
	pub fn env_set_global_inter_op_num_threads(&self, tp: ThreadingOptions, num: i32) -> Result<()> {
		onnx_call!(self.api, SetGlobalInterOpNumThreads(tp.ptr, num))
	}

	/// Set global spin control options.
	pub fn env_set_global_spin_control(&self, tp: ThreadingOptions, setting: SpinControl) -> Result<()> {
		onnx_call!(self.api, SetGlobalSpinControl(tp.ptr, setting.into()))
	}

	/// Set threading flush-to-zero and denormal-as-zero
	pub fn env_set_global_denormal_as_zero(&self, tp: ThreadingOptions) -> Result<()> {
		onnx_call!(self.api, SetGlobalDenormalAsZero(tp.ptr))
	}

	pub fn env_set_global_intra_op_thread_affinity(&self, tp: ThreadingOptions, affinity: impl Into<Vec<u8>>) -> Result<()> {
		let affinity = CString::new(affinity).expect("invalid affinity string");

		onnx_call!(self.api, SetGlobalIntraOpThreadAffinity(tp.ptr, affinity.as_ptr()))
	}

	// Not yet impl
	// SetGlobalCustomCreateThreadFn
	// SetGlobalCustomThreadCreationOptions
	// SetGlobalCustomJoinThreadFn
}

#[derive(Debug)]
pub struct EnvPtr {
	ptr: *mut OrtEnv,
	logger: Option<LoggerPtr>,
	tp_options: Option<ThreadingOptions>
}

unsafe impl Send for EnvPtr {}
unsafe impl Sync for EnvPtr {}

impl Drop for EnvPtr {
	fn drop(&mut self) {
		Api::get().env_release(self);
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ThreadingOptions {
	ptr: *mut OrtThreadingOptions
}

impl ThreadingOptions {
	fn as_ptr(&self) -> *const OrtThreadingOptions {
		todo!()
	}
}

impl Drop for ThreadingOptions {
	fn drop(&mut self) {
		Api::get().env_release_threading_options(self);
	}
}

#[repr(i32)]
pub enum SpinControl {
	/// It won't spin (recommended if CPU usage is high)
	Disabled = 0,
	/// Threadpool will spin to wait for queue to become non-empty
	Enabled = 1
}

impl From<SpinControl> for i32 {
	fn from(value: SpinControl) -> Self {
		value as Self
	}
}

#[repr(u32)]
pub enum LanguageProjection {
	C = crate::sys::OrtLanguageProjection_ORT_PROJECTION_C,
	CPlusPlus = crate::sys::OrtLanguageProjection_ORT_PROJECTION_CPLUSPLUS,
	CSharp = crate::sys::OrtLanguageProjection_ORT_PROJECTION_CSHARP,
	Python = crate::sys::OrtLanguageProjection_ORT_PROJECTION_PYTHON,
	Java = crate::sys::OrtLanguageProjection_ORT_PROJECTION_JAVA,
	WinML = crate::sys::OrtLanguageProjection_ORT_PROJECTION_WINML,
	NodeJS = crate::sys::OrtLanguageProjection_ORT_PROJECTION_NODEJS
}

impl From<LanguageProjection> for u32 {
	fn from(value: LanguageProjection) -> Self {
		value as Self
	}
}
