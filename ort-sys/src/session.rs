use std::{
	ffi::{c_char, CStr},
	path::Path,
	ptr::null_mut
};

use smallvec::SmallVec;

use crate::{
	allocator::{Allocator, AllocatorValue},
	ctypes::size_t,
	environment::Env,
	io_binding::IoBinding,
	onnx_call,
	run_options::RunOptions,
	session_options::SessionOptions,
	string::OnnxString,
	sys::{OrtSession, OrtTypeInfo, OrtValue},
	value::Value,
	Api, Result
};

impl Api {}
struct Session {
	ptr: *mut OrtSession,
	alloc: Allocator
}

unsafe impl Sync for Session {}
unsafe impl Send for Session {}

impl Session {
	fn from_ptr(_ptr: *mut OrtSession) -> Self {
		todo!()
	}

	/// Create an OrtSession from a model file.
	pub fn create(env: Env, model_path: &Path, options: &SessionOptions) -> Result<Session> {
		let mut ptr = null_mut();
		let path = OnnxString::to_onnx_path(model_path);

		onnx_call!(Api::get_ptr(), CreateSession(env.as_ptr(), path.as_ptr(), options.as_ptr(), &mut ptr))?;

		Ok(Self::from_ptr(ptr))
	}

	/// Create an OrtSession from memory.
	pub fn create_from_array(env: Env, model_data: &[u8], options: &SessionOptions) -> Result<Session> {
		let mut ptr = null_mut();

		onnx_call!(Api::get_ptr(), CreateSessionFromArray(env.as_ptr(), model_data.as_ptr() as _, model_data.len(), options.as_ptr(), &mut ptr))?;

		Ok(Self::from_ptr(ptr))
	}

	/// Run the model in an OrtSession.
	///
	/// Blocks until the model run has completed.
	/// Multiple threads might be used to run the model based on the options in the Session and settings used when
	/// creating the Env.
	#[allow(clippy::useless_conversion)]
	pub fn run(
		&self,
		options: &RunOptions,
		inputs: &[&Value],
		input_names: &[&str],
		outputs: impl Iterator<Item = Option<Value>>,
		output_names: &[&str]
	) -> Result<impl Iterator<Item = Value>> {
		OnnxString::prepare(input_names.iter().chain(output_names.iter()).map(|s| s.len() + 1).sum());

		let input_name_ptrs: SmallVec<[*const c_char; 16]> = input_names.iter().map(|s| OnnxString::to_onnx(s).0).collect();
		let input_len: size_t = input_names.len().try_into().unwrap();
		let input_value_ptrs: SmallVec<[*const OrtValue; 16]> = inputs.iter().map(|v| v.as_ptr()).collect();

		let output_name_ptrs: SmallVec<[*const c_char; 16]> = output_names.iter().map(|s| OnnxString::to_onnx(s).0).collect();
		let output_len: size_t = output_names.len().try_into().unwrap();
		let mut outputs: SmallVec<[Option<Value>; 16]> = outputs.collect();
		let mut output_value_ptrs: SmallVec<[*mut OrtValue; 16]> = outputs
			.iter_mut()
			.map(|v| v.as_mut().map(|vv| vv.as_mut_ptr()).unwrap_or(null_mut()))
			.collect();

		onnx_call!(
			Api::get_ptr(),
			Run(
				self.ptr,
				options.as_ptr(),
				input_name_ptrs.as_ptr(),
				input_value_ptrs.as_ptr(),
				input_len,
				output_name_ptrs.as_ptr(),
				output_len,
				output_value_ptrs.as_mut_ptr()
			)
		)?;

		// Fill in the None's with the returned values
		let ret = outputs
			.into_iter()
			.zip(output_value_ptrs.into_iter())
			.map(|(opt, ptr)| opt.unwrap_or_else(|| Value::from_raw(ptr)));

		Ok(ret)
	}

	/// Run the model asynchronously in a thread owned by intra op thread pool.
	// #[allow(clippy::useless_conversion)]
	// pub fn run_async(
	// 	&self,
	// 	options: &RunOptions,
	// 	inputs: &[&Value],
	// 	input_names: &[&str],
	// 	outputs: impl Iterator<Item = Option<Value>>,
	// 	output_names: &[&str]
	// ) -> Result<impl Iterator<Item = Value>> {
	// 	OnnxString::prepare(input_names.iter().chain(output_names.iter()).map(|s| s.len() + 1).sum());

	// 	let input_name_ptrs: SmallVec<[*const c_char; 16]> = input_names.iter().map(|s| OnnxString::to_onnx(s).0).collect();
	// 	let input_len: size_t = input_names.len().try_into().unwrap();
	// 	let input_value_ptrs: SmallVec<[*const OrtValue; 16]> = inputs.iter().map(|v| v.as_ptr()).collect();

	// 	let output_name_ptrs: SmallVec<[*const c_char; 16]> = output_names.iter().map(|s|
	// OnnxString::to_onnx(s).0).collect(); 	let output_len: size_t = output_names.len().try_into().unwrap();
	// 	let mut outputs: SmallVec<[Option<Value>; 16]> = outputs.collect();
	// 	let mut output_value_ptrs: SmallVec<[*mut OrtValue; 16]> = outputs
	// 		.iter_mut()
	// 		.map(|v| v.as_mut().map(|vv| vv.as_mut_ptr()).unwrap_or(null_mut()))
	// 		.collect();

	// 	let callback: *mut c_void = std::ptr::null_mut();
	// 	// let shared = Arc
	// 	let shared_ptr: *mut c_void = std::ptr::null_mut();

	// 	// onnx_call!(
	// 	// 	Api::get_ptr(),
	// 	// 	RunAsync(
	// 	// 		self.ptr,
	// 	// 		options.as_ptr(),
	// 	// 		input_name_ptrs.as_ptr(),
	// 	// 		input_value_ptrs.as_ptr(),
	// 	// 		input_len,
	// 	// 		output_name_ptrs.as_ptr(),
	// 	// 		output_len,
	// 	// 		output_value_ptrs.as_mut_ptr()
	// 	// 	)
	// 	// )?;

	// 	Ok(output_value_ptrs.into_iter().map(Value::from_raw))
	// }

	// OrtStatus * 	SessionGetInputCount (const OrtSession *session, size_t *out)
	//  	Get input count for a session.
	pub fn input_count(&self) -> Result<usize> {
		let mut out = 0;
		onnx_call!(Api::get_ptr(), SessionGetInputCount(self.ptr, &mut out))?;
		Ok(out)
	}

	// Get output count for a session.
	pub fn outputs_count(&self) -> Result<usize> {
		let mut out = 0;
		onnx_call!(Api::get_ptr(), SessionGetOutputCount(self.ptr, &mut out))?;
		Ok(out)
	}

	/// Get overridable initializer count.
	pub fn initializer_count(&self) -> Result<usize> {
		let mut out = 0;
		onnx_call!(Api::get_ptr(), SessionGetOverridableInitializerCount(self.ptr, &mut out))?;
		Ok(out)
	}

	/// Get input type information.
	#[allow(clippy::useless_conversion)]
	pub fn input_type_info(&self, idx: usize) -> Result<TypeInfo> {
		let index = idx.try_into().unwrap();
		let mut ptr = null_mut();
		onnx_call!(Api::get_ptr(), SessionGetInputTypeInfo(self.ptr, index, &mut ptr))?;
		Ok(TypeInfo { ptr })
	}

	/// Get output type information.
	#[allow(clippy::useless_conversion)]
	pub fn output_type_info(&self, idx: usize) -> Result<TypeInfo> {
		let index = idx.try_into().unwrap();
		let mut ptr = null_mut();
		onnx_call!(Api::get_ptr(), SessionGetOutputTypeInfo(self.ptr, index, &mut ptr))?;
		Ok(TypeInfo { ptr })
	}

	/// Get overridable initializer type information.
	#[allow(clippy::useless_conversion)]
	pub fn initializer_type_info(&self, idx: usize) -> Result<TypeInfo> {
		let index = idx.try_into().unwrap();
		let mut ptr = null_mut();
		onnx_call!(Api::get_ptr(), SessionGetOverridableInitializerTypeInfo(self.ptr, index, &mut ptr))?;
		Ok(TypeInfo { ptr })
	}

	/// Get input name.
	#[allow(clippy::useless_conversion)]
	pub fn input_name(&self, idx: usize, alloc: Allocator) -> Result<AllocatorValue<&str>> {
		let index = idx.try_into().unwrap();
		let mut ptr = null_mut();

		alloc.with_ptr(|alloc_ptr| onnx_call!(Api::get_ptr(), SessionGetInputName(self.ptr, index, alloc_ptr, &mut ptr)))?;

		let name = unsafe { std::str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()) };

		Ok(AllocatorValue::create(name, alloc, ptr as _))
	}

	/// Get output name.
	#[allow(clippy::useless_conversion)]
	pub fn output_name(&mut self, idx: usize, alloc: Allocator) -> Result<AllocatorValue<&str>> {
		let index = idx.try_into().unwrap();
		let mut ptr = null_mut();
		alloc.with_ptr(|alloc_ptr| onnx_call!(Api::get_ptr(), SessionGetOutputName(self.ptr, index, alloc_ptr, &mut ptr)))?;

		let name = unsafe { std::str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()) };

		Ok(AllocatorValue::create(name, alloc, ptr as _))
	}

	/// Get overridable initializer name.
	#[allow(clippy::useless_conversion)]
	pub fn initializer_name(&self, idx: usize, alloc: Allocator) -> Result<AllocatorValue<&str>> {
		let index = idx.try_into().unwrap();
		let mut ptr = null_mut();
		alloc.with_ptr(|alloc_ptr| onnx_call!(Api::get_ptr(), SessionGetOverridableInitializerName(self.ptr, index, alloc_ptr, &mut ptr)))?;

		let name = unsafe { std::str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()) };

		Ok(AllocatorValue::create(name, alloc, ptr as _))
	}

	/// End profiling and return filename of the profile data.
	pub fn end_profiling(&self, alloc: Allocator) -> Result<AllocatorValue<&str>> {
		let mut ptr = null_mut();
		alloc.with_ptr(|alloc_ptr| onnx_call!(Api::get_ptr(), SessionEndProfiling(self.ptr, alloc_ptr, &mut ptr)))?;

		let name = unsafe { std::str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()) };

		Ok(AllocatorValue::create(name, alloc, ptr as _))
	}

	// OrtStatus * 	SessionGetModelMetadata (const OrtSession *session, OrtModelMetadata **out)
	//  	Get OrtModelMetadata from an OrtSession.

	/// Run a model using Io Bindings for the inputs & outputs.
	pub fn run_with_binding(&mut self, options: &RunOptions, binding: &mut IoBinding) -> Result<()> {
		onnx_call!(Api::get_ptr(), RunWithBinding(self.ptr, options.as_ptr(), binding.as_ptr()))?;
		binding.update_outputs(&self.alloc)?;

		Ok(())
	}

	/// Create an IoBinding instance.
	pub fn create_binding(&self) -> Result<IoBinding> {
		let mut ptr = null_mut();
		onnx_call!(Api::get_ptr(), CreateIoBinding(self.ptr, &mut ptr))?;

		Ok(IoBinding::from_ptr(ptr))
	}

	// OrtStatus * 	SessionGetProfilingStartTimeNs (const OrtSession *session, uint64_t *out)
	//  	Return the time that profiling was started.

	// OrtStatus * 	CreateSessionWithPrepackedWeightsContainer (const OrtEnv *env, const char *model_path, const
	// OrtSessionOptions *options, OrtPrepackedWeightsContainer *prepacked_weights_container, OrtSession **out)
	//  	Create session with prepacked weights container.

	// OrtStatus * 	CreateSessionFromArrayWithPrepackedWeightsContainer (const OrtEnv *env, const void *model_data, size_t
	// model_data_length, const OrtSessionOptions *options, OrtPrepackedWeightsContainer *prepacked_weights_container,
	// OrtSession **out)  	Create session from memory with prepacked weights container.

	// OrtStatus * 	CreatePrepackedWeightsContainer (OrtPrepackedWeightsContainer **out)
	//  	Create an OrtPrepackedWeightsContainer.

	// void 	ReleasePrepackedWeightsContainer (OrtPrepackedWeightsContainer *input)
	//  	Release OrtPrepackedWeightsContainer instance.
}

impl Drop for Session {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			onnx_call!(Api::get_ptr(), ReleaseSession(self.ptr) -> ()).expect("unable to release OrtSession");
			self.ptr = null_mut();
		}
	}
}

struct TypeInfo {
	ptr: *mut OrtTypeInfo
}
