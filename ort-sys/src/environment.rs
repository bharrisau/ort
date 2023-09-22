use std::ffi::CString;

use crate::logger::{LogLevel, Logger, LoggerPtr};
use crate::memory_info::{ArenaConfig, MemoryInfo};
use crate::string::OnnxString;
use crate::sys::{OrtEnv, OrtThreadingOptions};
use crate::{onnx_call, Api, Result};

// SetGlobalCustomCreateThreadFn
// SetGlobalCustomThreadCreationOptions
// SetGlobalCustomJoinThreadFn

#[derive(Debug)]
pub struct Env {
	ptr: *mut OrtEnv,
	logger: Option<LoggerPtr>
}

unsafe impl Send for Env {}
unsafe impl Sync for Env {}

impl Env {
	pub fn create<T: Logger + 'static>(log_severity_level: LogLevel, logid: &str, logger: Option<T>, tp_options: Option<&ThreadingOptions>) -> Result<Env> {
		OnnxString::prepare(logid.len() + 1);
		let (logid_ptr, _) = OnnxString::to_onnx(logid);
		let mut env_ptr: *mut OrtEnv = std::ptr::null_mut();
		let logger = logger.map(LoggerPtr::for_logger);

		match (logger.as_ref(), tp_options) {
			(None, None) => onnx_call!(Api::get_ptr(), CreateEnv(log_severity_level.into(), logid_ptr, &mut env_ptr))?,
			(Some(logger), None) => {
				onnx_call!(Api::get_ptr(), CreateEnvWithCustomLogger(logger.log, logger.ptr, log_severity_level.into(), logid_ptr, &mut env_ptr))?
			}
			(None, Some(tp)) => onnx_call!(Api::get_ptr(), CreateEnvWithGlobalThreadPools(log_severity_level.into(), logid_ptr, tp.as_ptr(), &mut env_ptr))?,
			(Some(logger), Some(tp)) => onnx_call!(
				Api::get_ptr(),
				CreateEnvWithCustomLoggerAndGlobalThreadPools(logger.log, logger.ptr, log_severity_level.into(), logid_ptr, tp.as_ptr(), &mut env_ptr)
			)?
		}

		Ok(Env { ptr: env_ptr, logger })
	}

	pub(crate) fn as_ptr(&self) -> *const OrtEnv {
		self.ptr
	}

	pub fn enable_telemetry_events(&self) -> Result<()> {
		onnx_call!(Api::get_ptr(), EnableTelemetryEvents(self.ptr))
	}

	/// Disable Telemetry.
	pub fn disable_telemetry_events(&self) -> Result<()> {
		onnx_call!(Api::get_ptr(), DisableTelemetryEvents(self.ptr))
	}

	pub fn create_and_register_allocator(&self, mem: &MemoryInfo, arena: &ArenaConfig) -> Result<()> {
		onnx_call!(Api::get_ptr(), CreateAndRegisterAllocator(self.ptr, mem.ptr, arena.ptr))
	}

	pub fn set_language_projection(&self, projection: LanguageProjection) -> Result<()> {
		onnx_call!(Api::get_ptr(), SetLanguageProjection(self.ptr, projection.into()))
	}
}

impl Drop for Env {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			onnx_call!(Api::get_ptr(), ReleaseEnv(self.ptr) -> ()).expect("unable to release OrtEnv");
			self.ptr = std::ptr::null_mut();
		}
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

	pub fn create() -> Result<ThreadingOptions> {
		let mut ptr: *mut OrtThreadingOptions = std::ptr::null_mut();

		onnx_call!(Api::get_ptr(), CreateThreadingOptions(&mut ptr))?;

		Ok(ThreadingOptions { ptr })
	}

	/// Set global intra-op thread count.
	pub fn set_global_intra_op_num_threads(self, num: i32) -> Result<Self> {
		onnx_call!(Api::get_ptr(), SetGlobalIntraOpNumThreads(self.ptr, num))?;
		Ok(self)
	}

	/// Set global inter-op thread count.
	pub fn set_global_inter_op_num_threads(self, num: i32) -> Result<Self> {
		onnx_call!(Api::get_ptr(), SetGlobalInterOpNumThreads(self.ptr, num))?;
		Ok(self)
	}

	/// Set global spin control options.
	pub fn set_global_spin_control(self, setting: SpinControl) -> Result<Self> {
		onnx_call!(Api::get_ptr(), SetGlobalSpinControl(self.ptr, setting.into()))?;
		Ok(self)
	}

	/// Set threading flush-to-zero and denormal-as-zero
	pub fn set_global_denormal_as_zero(self) -> Result<Self> {
		onnx_call!(Api::get_ptr(), SetGlobalDenormalAsZero(self.ptr))?;
		Ok(self)
	}

	pub fn set_global_intra_op_thread_affinity(self, affinity: impl Into<Vec<u8>>) -> Result<Self> {
		let affinity = CString::new(affinity).expect("invalid affinity string");

		onnx_call!(Api::get_ptr(), SetGlobalIntraOpThreadAffinity(self.ptr, affinity.as_ptr()))?;
		Ok(self)
	}
}

impl Drop for ThreadingOptions {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			onnx_call!(Api::get_ptr(), ReleaseThreadingOptions(self.ptr) -> ()).expect("unable to release ThreadingOptions");
			self.ptr = std::ptr::null_mut();
		}
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
