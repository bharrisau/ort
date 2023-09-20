use crate::{onnx_call, Api, Result};

impl Api {}

// pub KernelInfoGetAttribute_float: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(info: *const OrtKernelInfo, name: *const ::std::os::raw::c_char, out: *mut f32)
// -> OrtStatusPtr) >,
// pub KernelInfoGetAttribute_int64: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(info: *const OrtKernelInfo, name: *const ::std::os::raw::c_char, out: *mut i64)
// -> OrtStatusPtr) >,
// pub KernelInfoGetAttribute_string: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(info: *const OrtKernelInfo, name: *const ::std::os::raw::c_char, out: *mut ::std::os::raw::c_char,
// size: *mut size_t) -> OrtStatusPtr     )
// >,
// pub KernelContext_GetInputCount:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(context: *const OrtKernelContext, out: *mut size_t) ->
// OrtStatusPtr)>, pub KernelContext_GetOutputCount:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(context: *const OrtKernelContext, out: *mut size_t) ->
// OrtStatusPtr)>, pub KernelContext_GetInput:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(context: *const OrtKernelContext, index: size_t, out:
// *mut *const OrtValue) -> OrtStatusPtr)>, pub KernelContext_GetOutput: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(context: *mut OrtKernelContext, index: size_t, dim_values: *const i64, dim_count: size_t, out: *mut
// *mut OrtValue) -> OrtStatusPtr     )
// >,

// OrtStatus * 	KernelInfoGetAttribute_float (const OrtKernelInfo *info, const char *name, float *out)
// Get a float stored as an attribute in the graph node.

//  OrtStatus * 	KernelInfoGetAttribute_int64 (const OrtKernelInfo *info, const char *name, int64_t *out)
//       Fetch a 64-bit int stored as an attribute in the graph node.

//  OrtStatus * 	KernelInfoGetAttribute_string (const OrtKernelInfo *info, const char *name, char *out, size_t *size)
//       Fetch a string stored as an attribute in the graph node.

//  OrtStatus * 	KernelInfoGetAttributeArray_float (const OrtKernelInfo *info, const char *name, float *out, size_t
// *size)       Fetch an array of int64_t values stored as an attribute in the graph node.

//  OrtStatus * 	KernelInfoGetAttributeArray_int64 (const OrtKernelInfo *info, const char *name, int64_t *out, size_t
// *size)       Fetch an array of int64_t values stored as an attribute in the graph node.

//  OrtStatus * 	KernelInfo_GetInputCount (const OrtKernelInfo *info, size_t *out)
//       Get the number of inputs from OrtKernelInfo.

//  OrtStatus * 	KernelInfo_GetOutputCount (const OrtKernelInfo *info, size_t *out)
//       Get the number of outputs from OrtKernelInfo.

//  OrtStatus * 	KernelInfo_GetInputName (const OrtKernelInfo *info, size_t index, char *out, size_t *size)
//       Get the name of a OrtKernelInfo's input.

//  OrtStatus * 	KernelInfo_GetOutputName (const OrtKernelInfo *info, size_t index, char *out, size_t *size)
//       Get the name of a OrtKernelInfo's output.

//  OrtStatus * 	KernelInfo_GetInputTypeInfo (const OrtKernelInfo *info, size_t index, OrtTypeInfo **type_info)
//       Get the type information for a OrtKernelInfo's input.

//  OrtStatus * 	KernelInfo_GetOutputTypeInfo (const OrtKernelInfo *info, size_t index, OrtTypeInfo **type_info)
//       Get the type information for a OrtKernelInfo's output.

//  OrtStatus * 	KernelInfoGetAttribute_tensor (const OrtKernelInfo *info, const char *name, OrtAllocator *allocator,
// OrtValue **out)       Get a OrtValue tensor stored as an attribute in the graph node.

//  OrtStatus * 	KernelInfo_GetNodeName (const OrtKernelInfo *info, char *out, size_t *size)
//       Get the graph node name from OrtKernelInfo.

//  OrtStatus * 	KernelInfo_GetLogger (const OrtKernelInfo *info, const OrtLogger **logger)
//       Get the session logger from OrtKernelInfo.

// OrtStatus * 	KernelContext_GetInputCount (const OrtKernelContext *context, size_t *out)
//  	Used for custom operators, get the input count of a kernel.

// OrtStatus * 	KernelContext_GetOutputCount (const OrtKernelContext *context, size_t *out)
//  	Used for custom operators, get the output count of a kernel.

// OrtStatus * 	KernelContext_GetInput (const OrtKernelContext *context, size_t index, const OrtValue **out)
//  	Used for custom operators, get an input of a kernel.

// OrtStatus * 	KernelContext_GetOutput (OrtKernelContext *context, size_t index, const int64_t *dim_values, size_t
// dim_count, OrtValue **out)  	Used for custom operators, get an output of a kernel.

// OrtStatus * 	KernelContext_GetGPUComputeStream (const OrtKernelContext *context, void **out)
//  	Used for custom operators, gets the GPU compute stream to use to launch the custom a GPU kernel.

// OrtStatus * 	KernelContext_GetLogger (const OrtKernelContext *context, const OrtLogger **logger)
//  	Get the runtime logger from OrtKernelContext.

// OrtStatus * 	Logger_LogMessage (const OrtLogger *logger, OrtLoggingLevel log_severity_level, const char *message,
// const char *file_path, int line_number, const char *func_name)  	Logs a message at the given severity level using the
// provided OrtLogger.

// OrtStatus * 	Logger_GetLoggingSeverityLevel (const OrtLogger *logger, OrtLoggingLevel *out)
//  	Get the logging severity level of the OrtLogger.
