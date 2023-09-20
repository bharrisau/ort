use crate::{onnx_call, sys::OrtSessionOptions, Api, Result};

impl Api {
	// OrtStatus * 	CreateSessionOptions (OrtSessionOptions **options)
	/// Create an OrtSessionOptions object.
	pub fn session_options_create(&self) -> Result<SessionOptionsPtr> {
		let mut ptr: *mut OrtSessionOptions = std::ptr::null_mut();
		onnx_call!(self.api, CreateSessionOptions(&mut ptr))?;
		Ok(SessionOptionsPtr { ptr })
	}

	// OrtStatus * 	SetOptimizedModelFilePath (OrtSessionOptions *options, const char *optimized_model_filepath)
	//  	Set filepath to save optimized model after graph level transformations.

	// OrtStatus * 	CloneSessionOptions (const OrtSessionOptions *in_options, OrtSessionOptions **out_options)
	//  	Create a copy of an existing OrtSessionOptions.

	// OrtStatus * 	SetSessionExecutionMode (OrtSessionOptions *options, ExecutionMode execution_mode)
	//  	Set execution mode.

	// OrtStatus * 	EnableProfiling (OrtSessionOptions *options, const char *profile_file_prefix)
	//  	Enable profiling for a session.

	// OrtStatus * 	DisableProfiling (OrtSessionOptions *options)
	//  	Disable profiling for a session.

	// OrtStatus * 	EnableMemPattern (OrtSessionOptions *options)
	//  	Enable the memory pattern optimization.

	// OrtStatus * 	DisableMemPattern (OrtSessionOptions *options)
	//  	Disable the memory pattern optimization.

	// OrtStatus * 	EnableCpuMemArena (OrtSessionOptions *options)
	//  	Enable the memory arena on CPU.

	// OrtStatus * 	DisableCpuMemArena (OrtSessionOptions *options)
	//  	Disable the memory arena on CPU.

	// OrtStatus * 	SetSessionLogId (OrtSessionOptions *options, const char *logid)
	//  	Set session log id.

	// OrtStatus * 	SetSessionLogVerbosityLevel (OrtSessionOptions *options, int session_log_verbosity_level)
	//  	Set session log verbosity level.

	// OrtStatus * 	SetSessionLogSeverityLevel (OrtSessionOptions *options, int session_log_severity_level)
	//  	Set session log severity level.

	// OrtStatus * 	SetSessionGraphOptimizationLevel (OrtSessionOptions *options, GraphOptimizationLevel
	// graph_optimization_level)  	Set the optimization level to apply when loading a graph.

	// OrtStatus * 	SetIntraOpNumThreads (OrtSessionOptions *options, int intra_op_num_threads)
	//  	Sets the number of threads used to parallelize the execution within nodes.

	// OrtStatus * 	SetInterOpNumThreads (OrtSessionOptions *options, int inter_op_num_threads)
	//  	Sets the number of threads used to parallelize the execution of the graph.

	// OrtStatus * 	AddCustomOpDomain (OrtSessionOptions *options, OrtCustomOpDomain *custom_op_domain)
	//  	Add custom op domain to a session options.

	// OrtStatus * 	RegisterCustomOpsLibrary (OrtSessionOptions *options, const char *library_path, void **library_handle)

	// OrtStatus * 	AddFreeDimensionOverride (OrtSessionOptions *options, const char *dim_denotation, int64_t dim_value)
	//  	Override session symbolic dimensions.

	/// Release allocation for session options.
	pub(crate) fn session_options_release(&self, opts: &mut SessionOptionsPtr) {
		if !opts.ptr.is_null() {
			onnx_call!(self.api, ReleaseSessionOptions(opts.ptr) -> ()).expect("unable to release session options");
			opts.ptr = std::ptr::null_mut();
		}
	}

	// OrtStatus * 	DisablePerSessionThreads (OrtSessionOptions *options)
	//  	Use global thread pool on a session.

	// OrtStatus * 	AddFreeDimensionOverrideByName (OrtSessionOptions *options, const char *dim_name, int64_t dim_value)

	// OrtStatus * 	AddSessionConfigEntry (OrtSessionOptions *options, const char *config_key, const char *config_value)
	//  	Set a session configuration entry as a pair of strings.

	// OrtStatus * 	AddInitializer (OrtSessionOptions *options, const char *name, const OrtValue *val)
	//  	Add a pre-allocated initializer to a session.

	// OrtStatus * 	SessionOptionsAppendExecutionProvider_CUDA (OrtSessionOptions *options, const OrtCUDAProviderOptions
	// *cuda_options)  	Append CUDA provider to session options.

	// OrtStatus * 	SessionOptionsAppendExecutionProvider_ROCM (OrtSessionOptions *options, const OrtROCMProviderOptions
	// *rocm_options)  	Append ROCM execution provider to the session options.

	// OrtStatus * 	SessionOptionsAppendExecutionProvider_OpenVINO (OrtSessionOptions *options, const
	// OrtOpenVINOProviderOptions *provider_options)  	Append OpenVINO execution provider to the session options.

	// OrtStatus * 	SessionOptionsAppendExecutionProvider_TensorRT (OrtSessionOptions *options, const
	// OrtTensorRTProviderOptions *tensorrt_options)  	Append TensorRT provider to session options.

	// OrtStatus * 	SessionOptionsAppendExecutionProvider_TensorRT_V2 (OrtSessionOptions *options, const
	// OrtTensorRTProviderOptionsV2 *tensorrt_options)  	Append TensorRT execution provider to the session options.

	// OrtStatus * 	EnableOrtCustomOps (OrtSessionOptions *options)
	//  	Enable custom operators.

	// OrtStatus * 	HasValue (const OrtValue *value, int *out)
	//  	Sets out to 1 iff an optional type OrtValue has an element, 0 otherwise (OrtValue is None) Use this API to find if
	// the optional type OrtValue is None or not. If the optional type OrtValue is not None, use the OrtValue just like any
	// other OrtValue. For example, if you get an OrtValue that corresponds to Optional(tensor) and if HasValue() returns
	// true, use it as tensor and so on.

	// OrtStatus * 	SessionOptionsAppendExecutionProvider_CUDA_V2 (OrtSessionOptions *options, const OrtCUDAProviderOptionsV2
	// *cuda_options)  	Append CUDA execution provider to the session options.

	// OrtStatus * 	HasSessionConfigEntry (const OrtSessionOptions *options, const char *config_key, int *out)
	//  	Checks if the given session configuration entry exists.

	// OrtStatus * 	GetSessionConfigEntry (const OrtSessionOptions *options, const char *config_key, char *config_value,
	// size_t *size)  	Get a session configuration value.

	// void(* 	ReleaseROCMProviderOptions )(OrtROCMProviderOptions *input)
	//  	Release an OrtROCMProviderOptions.

	// OrtStatus * 	CreateROCMProviderOptions (OrtROCMProviderOptions **out)
	//  	Create an OrtROCMProviderOptions.

	// OrtStatus * 	UpdateROCMProviderOptions (OrtROCMProviderOptions *rocm_options, const char *const
	// *provider_options_keys, const char *const *provider_options_values, size_t num_keys)  	Set options in a ROCm Execution
	// Provider.

	// OrtStatus * 	GetROCMProviderOptionsAsString (const OrtROCMProviderOptions *rocm_options, OrtAllocator *allocator, char
	// **ptr)

	// OrtStatus * 	CreateAndRegisterAllocatorV2 (OrtEnv *env, const char *provider_type, const OrtMemoryInfo *mem_info,
	// const OrtArenaCfg *arena_cfg, const char *const *provider_options_keys, const char *const *provider_options_values,
	// size_t num_keys)  	Create an allocator with specific type and register it with the OrtEnv This API enhance
	// CreateAndRegisterAllocator that it can create an allocator with specific type, not just CPU allocator Enables sharing
	// the allocator between multiple sessions that use the same env instance. Lifetime of the created allocator will be
	// valid for the duration of the environment. Returns an error if an allocator with the same OrtMemoryInfo is already
	// registered.

	// OrtStatus * 	RunAsync (OrtSession *session, const OrtRunOptions *run_options, const char *const *input_names, const
	// OrtValue *const *input, size_t input_len, const char *const *output_names, size_t output_names_len, OrtValue
	// **output, RunAsyncCallbackFn run_async_callback, void *user_data)  	Run the model asynchronously in a thread owned by
	// intra op thread pool.

	// OrtStatus * 	UpdateTensorRTProviderOptionsWithValue (OrtTensorRTProviderOptionsV2 *tensorrt_options, const char *key,
	// void *value)

	// OrtStatus * 	GetTensorRTProviderOptionsByName (const OrtTensorRTProviderOptionsV2 *tensorrt_options, const char *key,
	// void **ptr)

	// OrtStatus * 	UpdateCUDAProviderOptionsWithValue (OrtCUDAProviderOptionsV2 *cuda_options, const char *key, void *value)

	// OrtStatus * 	GetCUDAProviderOptionsByName (const OrtCUDAProviderOptionsV2 *cuda_options, const char *key, void **ptr)

	// OrtStatus * 	KernelContext_GetResource (const OrtKernelContext *context, int resouce_version, int resource_id, void
	// **resource)

	// OrtTensorRTProviderOptionsV2
	// void(* 	ReleaseTensorRTProviderOptions )(OrtTensorRTProviderOptionsV2 *input)
	//  	Release an OrtTensorRTProviderOptionsV2.

	// OrtStatus * 	CreateTensorRTProviderOptions (OrtTensorRTProviderOptionsV2 **out)
	//  	Create an OrtTensorRTProviderOptionsV2.

	// OrtStatus * 	UpdateTensorRTProviderOptions (OrtTensorRTProviderOptionsV2 *tensorrt_options, const char *const
	// *provider_options_keys, const char *const *provider_options_values, size_t num_keys)  	Set options in a TensorRT
	// Execution Provider.

	// OrtStatus * 	GetTensorRTProviderOptionsAsString (const OrtTensorRTProviderOptionsV2 *tensorrt_options, OrtAllocator
	// *allocator, char **ptr)  	Get serialized TensorRT provider options string.

	// OrtCUDAProviderOptionsV2
	// void(* 	ReleaseCUDAProviderOptions )(OrtCUDAProviderOptionsV2 *input)
	//  	Release an OrtCUDAProviderOptionsV2.

	// OrtStatus * 	CreateCUDAProviderOptions (OrtCUDAProviderOptionsV2 **out)
	//  	Create an OrtCUDAProviderOptionsV2.

	// OrtStatus * 	UpdateCUDAProviderOptions (OrtCUDAProviderOptionsV2 *cuda_options, const char *const
	// *provider_options_keys, const char *const *provider_options_values, size_t num_keys)  	Set options in a CUDA Execution
	// Provider.

	// OrtStatus * 	GetCUDAProviderOptionsAsString (const OrtCUDAProviderOptionsV2 *cuda_options, OrtAllocator *allocator,
	// char **ptr)

	// OrtStatus * 	SessionOptionsSetCustomCreateThreadFn (OrtSessionOptions *options, OrtCustomCreateThreadFn
	// ort_custom_create_thread_fn)  	Set custom thread creation function.

	// OrtStatus * 	SessionOptionsSetCustomThreadCreationOptions (OrtSessionOptions *options, void
	// *ort_custom_thread_creation_options)  	Set creation options for custom thread.

	// OrtStatus * 	SessionOptionsSetCustomJoinThreadFn (OrtSessionOptions *options, OrtCustomJoinThreadFn
	// ort_custom_join_thread_fn)  	Set custom thread join function.
}

#[repr(transparent)]
pub struct SessionOptionsPtr {
	ptr: *mut OrtSessionOptions
}

impl Drop for SessionOptionsPtr {
	fn drop(&mut self) {
		Api::get().session_options_release(self)
	}
}
