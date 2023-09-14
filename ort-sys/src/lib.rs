#![allow(dead_code)]

mod ctypes;
mod error;

#[allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case, deref_nullptr)]
#[allow(clippy::all)]
mod sys;

mod api;
mod logger;
mod status;

mod environment;

use std::ffi::{c_char, CStr};

use api::ApiPtr;
pub use error::{Error, Result};
pub use status::Status;

#[repr(transparent)]
pub struct Api {
	pub(crate) api: &'static ApiPtr
}

impl Api {
	/// Get the onnxruntime API
	pub fn get() -> Self {
		Api { api: ApiPtr::get() }
	}

	/// Tries to get the onnxruntime for a specific version.
	/// Will fail if API is already initialised at a different version.
	pub fn get_version(version: u32) -> Result<Self> {
		Ok(Api { api: ApiPtr::get_version(version)? })
	}

	/// Returns the build info including git info and cxx flags
	pub fn get_build_info_string(&self) -> Result<&'static str> {
		let ret = onnx_call!(self.api, GetBuildInfoString() -> *const i8)?;
		unsafe {
			let ret = CStr::from_ptr(ret);
			Ok(std::str::from_utf8_unchecked(ret.to_bytes()))
		}
	}

	/// Get the names of all available providers.
	///
	/// The providers in the list are not guaranteed to be usable.
	/// They may fail to load due to missing system dependencies.
	/// For example, if the CUDA/cuDNN libraries are not installed, the CUDA provider will report an error when it is
	/// added to the session options.
	pub fn get_available_providers(&self) -> Result<Providers> {
		let mut ptr: *mut *mut c_char = std::ptr::null_mut();
		let mut len: i32 = 0;

		onnx_call!(self.api, GetAvailableProviders(&mut ptr, &mut len))?;
		assert!(!ptr.is_null());
		let len = len.try_into().expect("unable to convert provider length");

		Ok(Providers { ptr, len })
	}

	/// Release data from Api::get_available_providers
	pub(crate) fn release_available_providers(&self, value: &mut Providers) {
		if !value.ptr.is_null() {
			let len = value.len.try_into().expect("unable to convert provider length");
			onnx_call!(self.api, ReleaseAvailableProviders(value.ptr, len)).expect("unable to release AvailableProviders");
			value.ptr = std::ptr::null_mut();
		}
	}
}

pub struct Providers {
	ptr: *mut *mut ::std::os::raw::c_char,
	len: usize
}

pub struct ProvidersIterator<'a> {
	slice: &'a [*mut ::std::os::raw::c_char]
}

impl<'a> Iterator for ProvidersIterator<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some((value, rest)) = self.slice.split_first() {
			self.slice = rest;
			unsafe {
				let ret = CStr::from_ptr(*value);
				Some(std::str::from_utf8_unchecked(ret.to_bytes()))
			}
		} else {
			None
		}
	}
}

impl<'a> IntoIterator for &'a Providers {
	type Item = &'a str;

	type IntoIter = ProvidersIterator<'a>;

	fn into_iter(self) -> Self::IntoIter {
		assert!(!self.ptr.is_null());
		let slice = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };

		ProvidersIterator { slice }
	}
}

impl Drop for Providers {
	fn drop(&mut self) {
		Api::get().release_available_providers(self);
	}
}

macro_rules! onnx_call {
	($api:expr, $call:ident($($n:expr),* $(,)?)) => {
		if let Some(f) = $api.$call {
			unsafe {
				let result = f($($n),*);

				if result.is_null() {
					Ok(())
				} else {
					Err(crate::status::Status::new(result, stringify!($call), std::panic::Location::caller()).into())
				}
			}
		} else {
			Err(Error::ApiUnavailable { call: stringify!($call) })
		}
	};
    ($api:expr, $call:ident($($n:expr),* $(,)?) -> $ret:ty) => {
		if let Some(f) = $api.$call {
			let result: $ret = unsafe { f($($n),*).into() };
            Ok(result)
		} else {
			Err(crate::Error::ApiUnavailable { call: stringify!($call) })
		}
	};
}

pub(crate) use onnx_call;

// pub ModelMetadataGetCustomMetadataMapKeys: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             model_metadata: *const OrtModelMetadata,
//             allocator: *mut OrtAllocator,
//             keys: *mut *mut *mut ::std::os::raw::c_char,
//             num_keys: *mut i64
//         ) -> OrtStatusPtr
//     )
// >,

// pub ModelMetadataGetGraphDescription: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(model_metadata: *const OrtModelMetadata, allocator: *mut OrtAllocator, value: *mut *mut
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
// pub SessionOptionsAppendExecutionProvider_TensorRT: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, tensorrt_options: *const
// OrtTensorRTProviderOptions) -> OrtStatusPtr) >,
// pub SetCurrentGpuDeviceId: ::std::option::Option<crate::ctypes::_system!(unsafe fn(device_id: ::std::os::raw::c_int)
// -> OrtStatusPtr)>, pub GetCurrentGpuDeviceId: ::std::option::Option<crate::ctypes::_system!(unsafe fn(device_id: *mut
// ::std::os::raw::c_int) -> OrtStatusPtr)>, pub KernelInfoGetAttributeArray_float: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(info: *const OrtKernelInfo, name: *const ::std::os::raw::c_char, out: *mut f32,
// size: *mut size_t) -> OrtStatusPtr) >,
// pub KernelInfoGetAttributeArray_int64: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(info: *const OrtKernelInfo, name: *const ::std::os::raw::c_char, out: *mut i64,
// size: *mut size_t) -> OrtStatusPtr) >,

// pub KernelContext_GetGPUComputeStream:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(context: *const OrtKernelContext, out: *mut *mut
// ::std::os::raw::c_void) -> OrtStatusPtr)>, pub GetTensorMemoryInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(value: *const OrtValue, mem_info: *mut *const
// OrtMemoryInfo) -> OrtStatusPtr)>, pub GetExecutionProviderApi: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(provider_name: *const ::std::os::raw::c_char, version: u32, provider_api: *mut *const
// ::std::os::raw::c_void) -> OrtStatusPtr     )
// >,
// pub SessionOptionsSetCustomCreateThreadFn: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, ort_custom_create_thread_fn:
// OrtCustomCreateThreadFn) -> OrtStatusPtr) >,
// pub SessionOptionsSetCustomThreadCreationOptions: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, ort_custom_thread_creation_options: *mut
// ::std::os::raw::c_void) -> OrtStatusPtr) >,
// pub SessionOptionsSetCustomJoinThreadFn: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, ort_custom_join_thread_fn:
// OrtCustomJoinThreadFn) -> OrtStatusPtr) >,

// pub SynchronizeBoundInputs: ::std::option::Option<crate::ctypes::_system!(unsafe fn(binding_ptr: *mut OrtIoBinding)
// -> OrtStatusPtr)>, pub SynchronizeBoundOutputs: ::std::option::Option<crate::ctypes::_system!(unsafe fn(binding_ptr:
// *mut OrtIoBinding) -> OrtStatusPtr)>, pub SessionOptionsAppendExecutionProvider_CUDA_V2: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, cuda_options: *const OrtCUDAProviderOptionsV2)
// -> OrtStatusPtr) >,
// pub CreateCUDAProviderOptions: ::std::option::Option<crate::ctypes::_system!(unsafe fn(out: *mut *mut
// OrtCUDAProviderOptionsV2) -> OrtStatusPtr)>, pub UpdateCUDAProviderOptions: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             cuda_options: *mut OrtCUDAProviderOptionsV2,
//             provider_options_keys: *const *const ::std::os::raw::c_char,
//             provider_options_values: *const *const ::std::os::raw::c_char,
//             num_keys: size_t
//         ) -> OrtStatusPtr
//     )
// >,
// pub GetCUDAProviderOptionsAsString: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(cuda_options: *const OrtCUDAProviderOptionsV2, allocator: *mut OrtAllocator, ptr: *mut *mut
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
// #[doc = " \\brief Release an ::OrtCUDAProviderOptionsV2\n\n \\note This is an exception in the naming convention of
// other Release* functions, as the name of the method does not have the V2 suffix, but the type does\n\n \\since
// Version 1.11."] pub ReleaseCUDAProviderOptions: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut
// OrtCUDAProviderOptionsV2))>, pub SessionOptionsAppendExecutionProvider_MIGraphX: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, migraphx_options: *const
// OrtMIGraphXProviderOptions) -> OrtStatusPtr) >,
// pub AddExternalInitializers: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             options: *mut OrtSessionOptions,
//             initializer_names: *const *const ::std::os::raw::c_char,
//             initializers: *const *const OrtValue,
//             initializers_num: size_t
//         ) -> OrtStatusPtr
//     )
// >,
// pub CreateOpAttr: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             name: *const ::std::os::raw::c_char,
//             data: *const ::std::os::raw::c_void,
//             len: ::std::os::raw::c_int,
//             type_: OrtOpAttrType,
//             op_attr: *mut *mut OrtOpAttr
//         ) -> OrtStatusPtr
//     )
// >,
// pub ReleaseOpAttr: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtOpAttr))>,
// pub CreateOp: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             info: *const OrtKernelInfo,
//             op_name: *const ::std::os::raw::c_char,
//             domain: *const ::std::os::raw::c_char,
//             version: ::std::os::raw::c_int,
//             type_constraint_names: *mut *const ::std::os::raw::c_char,
//             type_constraint_values: *const ONNXTensorElementDataType,
//             type_constraint_count: ::std::os::raw::c_int,
//             attr_values: *const *const OrtOpAttr,
//             attr_count: ::std::os::raw::c_int,
//             input_count: ::std::os::raw::c_int,
//             output_count: ::std::os::raw::c_int,
//             ort_op: *mut *mut OrtOp
//         ) -> OrtStatusPtr
//     )
// >,
// pub InvokeOp: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             context: *const OrtKernelContext,
//             ort_op: *const OrtOp,
//             input_values: *const *const OrtValue,
//             input_count: ::std::os::raw::c_int,
//             output_values: *const *mut OrtValue,
//             output_count: ::std::os::raw::c_int
//         ) -> OrtStatusPtr
//     )
// >,
// pub ReleaseOp: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtOp))>,
// pub SessionOptionsAppendExecutionProvider: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             options: *mut OrtSessionOptions,
//             provider_name: *const ::std::os::raw::c_char,
//             provider_options_keys: *const *const ::std::os::raw::c_char,
//             provider_options_values: *const *const ::std::os::raw::c_char,
//             num_keys: size_t
//         ) -> OrtStatusPtr
//     )
// >,
// pub CopyKernelInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(info: *const OrtKernelInfo, info_copy: *mut *mut
// OrtKernelInfo) -> OrtStatusPtr)>, pub ReleaseKernelInfo: ::std::option::Option<crate::ctypes::_system!(unsafe
// fn(input: *mut OrtKernelInfo))>, #[doc = " \\name Ort Training\n @{\n** \\brief Gets the Training C Api struct\n*\n*
// Call this function to access the ::OrtTrainingApi structure that holds pointers to functions that enable\n* training
// with onnxruntime.\n* \\note A NULL pointer will be returned and no error message will be printed if the training
// api\n* is not supported with this build. A NULL pointer will be returned and an error message will be\n* printed if
// the provided version is unsupported, for example when using a runtime older than the\n* version created with this
// header file.\n*\n* \\param[in] version Must be ::ORT_API_VERSION\n* \\return The ::OrtTrainingApi struct for the
// version requested.\n*\n* \\since Version 1.13\n*/"] pub GetTrainingApi:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(version: u32) -> *const OrtTrainingApi)>,
// pub SessionOptionsAppendExecutionProvider_CANN:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, cann_options: *const
// OrtCANNProviderOptions) -> OrtStatusPtr)>, pub CreateCANNProviderOptions:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(out: *mut *mut OrtCANNProviderOptions) -> OrtStatusPtr)>, pub
// UpdateCANNProviderOptions: ::std::option::Option<     crate::ctypes::_system!(
//         unsafe fn(
//             cann_options: *mut OrtCANNProviderOptions,
//             provider_options_keys: *const *const ::std::os::raw::c_char,
//             provider_options_values: *const *const ::std::os::raw::c_char,
//             num_keys: size_t
//         ) -> OrtStatusPtr
//     )
// >,
// pub GetCANNProviderOptionsAsString: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(cann_options: *const OrtCANNProviderOptions, allocator: *mut OrtAllocator, ptr: *mut *mut
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
// #[doc = " \\brief Release an OrtCANNProviderOptions\n\n \\param[in] the pointer of OrtCANNProviderOptions which will
// been deleted\n\n \\since Version 1.13."] pub ReleaseCANNProviderOptions:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtCANNProviderOptions))>,
// pub MemoryInfoGetDeviceType: ::std::option::Option<crate::ctypes::_system!(unsafe fn(ptr: *const OrtMemoryInfo, out:
// *mut OrtMemoryInfoDeviceType))>, pub UpdateEnvWithCustomLogLevel:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(ort_env: *mut OrtEnv, log_severity_level:
// OrtLoggingLevel) -> OrtStatusPtr)>,
// pub RegisterCustomOpsLibrary_V2:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, library_name: *const
// ORTCHAR_T) -> OrtStatusPtr)>, pub RegisterCustomOpsUsingFunction: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, registration_func_name: *const
// ::std::os::raw::c_char) -> OrtStatusPtr) >,
// pub KernelInfo_GetInputCount: ::std::option::Option<crate::ctypes::_system!(unsafe fn(info: *const OrtKernelInfo,
// out: *mut size_t) -> OrtStatusPtr)>, pub KernelInfo_GetOutputCount:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(info: *const OrtKernelInfo, out: *mut size_t) ->
// OrtStatusPtr)>, pub KernelInfo_GetInputName: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(info: *const OrtKernelInfo, index: size_t, out: *mut ::std::os::raw::c_char,
// size: *mut size_t) -> OrtStatusPtr) >,
// pub KernelInfo_GetOutputName: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(info: *const OrtKernelInfo, index: size_t, out: *mut ::std::os::raw::c_char,
// size: *mut size_t) -> OrtStatusPtr) >,
// pub KernelInfo_GetInputTypeInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(info: *const OrtKernelInfo, index: size_t, type_info:
// *mut *mut OrtTypeInfo) -> OrtStatusPtr)>, pub KernelInfo_GetOutputTypeInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(info: *const OrtKernelInfo, index: size_t, type_info:
// *mut *mut OrtTypeInfo) -> OrtStatusPtr)>, pub KernelInfoGetAttribute_tensor: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(info: *const OrtKernelInfo, name: *const ::std::os::raw::c_char, allocator: *mut OrtAllocator, out:
// *mut *mut OrtValue) -> OrtStatusPtr     )
// >,
// pub HasSessionConfigEntry: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(options: *const OrtSessionOptions, config_key: *const ::std::os::raw::c_char, out: *mut
// ::std::os::raw::c_int) -> OrtStatusPtr     )
// >,
// pub GetSessionConfigEntry: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             options: *const OrtSessionOptions,
//             config_key: *const ::std::os::raw::c_char,
//             config_value: *mut ::std::os::raw::c_char,
//             size: *mut size_t
//         ) -> OrtStatusPtr
//     )
// >,
// pub SessionOptionsAppendExecutionProvider_Dnnl:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtSessionOptions, dnnl_options: *const
// OrtDnnlProviderOptions) -> OrtStatusPtr)>, pub CreateDnnlProviderOptions:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(out: *mut *mut OrtDnnlProviderOptions) -> OrtStatusPtr)>, pub
// UpdateDnnlProviderOptions: ::std::option::Option<     crate::ctypes::_system!(
//         unsafe fn(
//             dnnl_options: *mut OrtDnnlProviderOptions,
//             provider_options_keys: *const *const ::std::os::raw::c_char,
//             provider_options_values: *const *const ::std::os::raw::c_char,
//             num_keys: size_t
//         ) -> OrtStatusPtr
//     )
// >,
// pub GetDnnlProviderOptionsAsString: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(dnnl_options: *const OrtDnnlProviderOptions, allocator: *mut OrtAllocator, ptr: *mut *mut
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
// #[doc = " \\brief Release an ::OrtDnnlProviderOptions\n\n \\since Version 1.15."]
// pub ReleaseDnnlProviderOptions: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut
// OrtDnnlProviderOptions))>, pub KernelInfo_GetNodeName: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(info: *const OrtKernelInfo, out: *mut ::std::os::raw::c_char, size: *mut
// size_t) -> OrtStatusPtr) >,
// pub KernelInfo_GetLogger:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(info: *const OrtKernelInfo, logger: *mut *const
// OrtLogger) -> OrtStatusPtr)>, pub KernelContext_GetLogger:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(context: *const OrtKernelContext, logger: *mut *const
// OrtLogger) -> OrtStatusPtr)>, pub Logger_LogMessage: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             logger: *const OrtLogger,
//             log_severity_level: OrtLoggingLevel,
//             message: *const ::std::os::raw::c_char,
//             file_path: *const ORTCHAR_T,
//             line_number: ::std::os::raw::c_int,
//             func_name: *const ::std::os::raw::c_char
//         ) -> OrtStatusPtr
//     )
// >,
// pub Logger_GetLoggingSeverityLevel:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(logger: *const OrtLogger, out: *mut OrtLoggingLevel) ->
// OrtStatusPtr)>, pub KernelInfoGetConstantInput_tensor: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(info: *const OrtKernelInfo, index: size_t, is_constant: *mut ::std::os::raw::c_int, out: *mut
// *const OrtValue) -> OrtStatusPtr     )
// >,
// pub CastTypeInfoToOptionalTypeInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(type_info: *const OrtTypeInfo, out: *mut *const
// OrtOptionalTypeInfo) -> OrtStatusPtr)>, pub GetOptionalContainedTypeInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(optional_type_info: *const OrtOptionalTypeInfo, out: *mut
// *mut OrtTypeInfo) -> OrtStatusPtr)>, pub GetResizedStringTensorElementBuffer: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(value: *mut OrtValue, index: size_t, length_in_bytes: size_t, buffer: *mut *mut
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
// pub KernelContext_GetAllocator: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(context: *const OrtKernelContext, mem_info: *const OrtMemoryInfo, out: *mut
// *mut OrtAllocator) -> OrtStatusPtr) >,
