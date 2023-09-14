// pub CreateSessionOptions: ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut *mut
// OrtSessionOptions) -> OrtStatusPtr)>, pub SetOptimizedModelFilePath:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions,
// optimized_model_filepath: *const ORTCHAR_T) -> OrtStatusPtr)>, pub CloneSessionOptions: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(in_options: *const OrtSessionOptions, out_options: *mut *mut OrtSessionOptions)
// -> OrtStatusPtr) >,
// pub SetSessionExecutionMode:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, execution_mode:
// ExecutionMode) -> OrtStatusPtr)>, pub EnableProfiling:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, profile_file_prefix:
// *const ORTCHAR_T) -> OrtStatusPtr)>, pub DisableProfiling: ::std::option::Option<crate::ctypes::_system!(unsafe
// fn(options: *mut OrtSessionOptions) -> OrtStatusPtr)>, pub EnableMemPattern:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions) -> OrtStatusPtr)>,
// pub DisableMemPattern: ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions) ->
// OrtStatusPtr)>, pub EnableCpuMemArena: ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut
// OrtSessionOptions) -> OrtStatusPtr)>, pub DisableCpuMemArena: ::std::option::Option<crate::ctypes::_system!(unsafe
// fn(options: *mut OrtSessionOptions) -> OrtStatusPtr)>, pub SetSessionLogId:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, logid: *const
// ::std::os::raw::c_char) -> OrtStatusPtr)>, pub SetSessionLogVerbosityLevel: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, session_log_verbosity_level:
// ::std::os::raw::c_int) -> OrtStatusPtr) >,
// pub SetSessionLogSeverityLevel: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, session_log_severity_level:
// ::std::os::raw::c_int) -> OrtStatusPtr) >,
// pub SetSessionGraphOptimizationLevel: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, graph_optimization_level:
// GraphOptimizationLevel) -> OrtStatusPtr) >,
// pub SetIntraOpNumThreads:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, intra_op_num_threads:
// ::std::os::raw::c_int) -> OrtStatusPtr)>, pub SetInterOpNumThreads:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, inter_op_num_threads:
// ::std::os::raw::c_int) -> OrtStatusPtr)>, pub CreateCustomOpDomain:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(domain: *const ::std::os::raw::c_char, out: *mut *mut
// OrtCustomOpDomain) -> OrtStatusPtr)>, pub CustomOpDomain_Add:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(custom_op_domain: *mut OrtCustomOpDomain, op: *const
// OrtCustomOp) -> OrtStatusPtr)>, pub AddCustomOpDomain:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, custom_op_domain: *mut
// OrtCustomOpDomain) -> OrtStatusPtr)>, pub RegisterCustomOpsLibrary: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             options: *mut OrtSessionOptions,
//             library_path: *const ::std::os::raw::c_char,
//             library_handle: *mut *mut ::std::os::raw::c_void
//         ) -> OrtStatusPtr
//     )
// >,
// pub AddFreeDimensionOverride: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, dim_denotation: *const ::std::os::raw::c_char,
// dim_value: i64) -> OrtStatusPtr) >,
/// pub ReleaseSessionOptions: ::std::option::Option<crate::ctypes::_system!(unsafe
// pub ReleaseCustomOpDomain: ::std::option::Option<crate::ctypes::_system!(unsafe
    // fn(input: *mut OrtCustomOpDomain))>,
    // pub DisablePerSessionThreads: ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut
// OrtSessionOptions) -> OrtStatusPtr)>, 
//pub AddInitializer: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, name: *const ::std::os::raw::c_char, val:
// *const OrtValue) -> OrtStatusPtr) >,
// pub AddFreeDimensionOverrideByName: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, dim_name: *const ::std::os::raw::c_char,
// dim_value: i64) -> OrtStatusPtr) >,
// pub SessionOptionsAppendExecutionProvider_CUDA:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, cuda_options: *const
// OrtCUDAProviderOptions) -> OrtStatusPtr)>, pub SessionOptionsAppendExecutionProvider_ROCM:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, rocm_options: *const
// OrtROCMProviderOptions) -> OrtStatusPtr)>, pub SessionOptionsAppendExecutionProvider_OpenVINO: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, provider_options: *const
// OrtOpenVINOProviderOptions) -> OrtStatusPtr) >,
// pub SessionOptionsAppendExecutionProvider_TensorRT_V2: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, tensorrt_options: *const
// OrtTensorRTProviderOptionsV2) -> OrtStatusPtr) >,
// pub CreateTensorRTProviderOptions: ::std::option::Option<crate::ctypes::_system!(unsafe fn(out: *mut *mut
// OrtTensorRTProviderOptionsV2) -> OrtStatusPtr)>, pub UpdateTensorRTProviderOptions: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             tensorrt_options: *mut OrtTensorRTProviderOptionsV2,
//             provider_options_keys: *const *const ::std::os::raw::c_char,
//             provider_options_values: *const *const ::std::os::raw::c_char,
//             num_keys: size_t
//         ) -> OrtStatusPtr
//     )
// >,
// pub GetTensorRTProviderOptionsAsString: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             tensorrt_options: *const OrtTensorRTProviderOptionsV2,
//             allocator: *mut OrtAllocator,
//             ptr: *mut *mut ::std::os::raw::c_char
//         ) -> OrtStatusPtr
//     )
// >,
// #[doc = " \\brief Release an ::OrtTensorRTProviderOptionsV2\n\n \\note This is an exception in the naming convention
// of other Release* functions, as the name of the method does not have the V2 suffix, but the type does"]
// pub ReleaseTensorRTProviderOptions: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut
// OrtTensorRTProviderOptionsV2))>, pub EnableOrtCustomOps: ::std::option::Option<crate::ctypes::_system!(unsafe
// fn(options: *mut OrtSessionOptions) -> OrtStatusPtr)>,