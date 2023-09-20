#![allow(dead_code)]

mod ctypes;
mod error;

#[allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case, deref_nullptr)]
#[allow(clippy::all)]
mod sys;

mod api;
mod logger;
mod status;
mod string;

mod allocator;
mod environment;
mod io_binding;
mod kernel;
mod memory_info;
mod run_options;
mod session;
mod session_options;
mod tensor;
mod value;

use std::ffi::{c_char, CStr};

use api::ApiPtr;
pub use error::{Error, Result};
pub use logger::{LogLevel, Logger};
pub use status::Status;

#[repr(transparent)]
pub struct Api {
	api: &'static ApiPtr
}

impl Api {
	/// Get the onnxruntime API
	pub fn get() -> Self {
		Api { api: ApiPtr::get() }
	}

	/// Get the onnxruntime API internal pointer
	pub(crate) fn get_ptr() -> &'static ApiPtr {
		Self::get().api
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
	/// This API will never fail so you can rely on it in a noexcept code.
	pub(crate) fn release_available_providers(&self, value: &mut Providers) {
		if !value.ptr.is_null() {
			let len = value.len.try_into().expect("unable to convert provider length");
			onnx_call!(self.api, ReleaseAvailableProviders(value.ptr, len)).expect("unable to release AvailableProviders");
			value.ptr = std::ptr::null_mut();
		}
	}

	/// Set current GPU device ID.
	pub fn set_current_gpu_device_id(&self, id: i32) -> Result<()> {
		onnx_call!(self.api, SetCurrentGpuDeviceId(id))
	}

	/// Get current GPU device ID.
	pub fn get_current_gpu_device_id(&self) -> Result<i32> {
		let mut id = 0;
		onnx_call!(self.api, GetCurrentGpuDeviceId(&mut id))?;
		Ok(id)
	}
}

pub struct Providers {
	ptr: *mut *mut ::std::os::raw::c_char,
	len: usize
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

macro_rules! onnx_call {
	($api:expr, $call:ident($($n:expr),* $(,)?)) => {
		if let Some(f) = $api.$call {
			unsafe {
				let result = f($($n),*);

				if result.is_null() {
					Ok(())
				} else {
					Err(crate::Status::new(result, stringify!($call)).into())
				}
			}
		} else {
			Err(crate::Error::ApiUnavailable { call: stringify!($call) })
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

// OrtStatus * 	GetExecutionProviderApi (const char *provider_name, uint32_t version, const void **provider_api)
//  	Get a pointer to the requested version of the Execution Provider specific API extensions to the OrtApi.

// const OrtTrainingApi *(* 	GetTrainingApi )(uint32_t version)
//  	Gets the Training C Api struct.

// void(* 	ReleaseCANNProviderOptions )(OrtCANNProviderOptions *input)
//  	Release an OrtCANNProviderOptions.

// void(* 	MemoryInfoGetDeviceType )(const OrtMemoryInfo *ptr, OrtMemoryInfoDeviceType *out)

// void(* 	ReleaseDnnlProviderOptions )(OrtDnnlProviderOptions *input)
//  	Release an OrtDnnlProviderOptions.

// OrtStatus * 	CreateCustomOpDomain (const char *domain, OrtCustomOpDomain **out)
//  	Create a custom op domain.

// OrtStatus * 	CustomOpDomain_Add (OrtCustomOpDomain *custom_op_domain, const OrtCustomOp *op)
//  	Add a custom op to a custom op domain.

// void 	ReleaseCustomOpDomain (OrtCustomOpDomain *input)

// OrtStatus * 	SynchronizeBoundInputs (OrtIoBinding *binding_ptr)
//  	Synchronize bound inputs. The call may be necessary for some providers, such as cuda, in case the system that
// allocated bound memory operated on a different stream. However, the operation is provider specific and could be a
// no-op.

// OrtStatus * 	SynchronizeBoundOutputs (OrtIoBinding *binding_ptr)
//  	Synchronize bound outputs. The call may be necessary for some providers, such as cuda, in case the system that
// allocated bound memory operated on a different stream. However, the operation is provider specific and could be a
// no-op.

// OrtStatus * 	SessionOptionsAppendExecutionProvider_MIGraphX (OrtSessionOptions *options, const
// OrtMIGraphXProviderOptions *migraphx_options)  	Append MIGraphX provider to session options.

// OrtStatus * 	AddExternalInitializers (OrtSessionOptions *options, const char *const *initializer_names, const OrtValue
// *const *initializers, size_t initializers_num)  	Replace initialized Tensors with external data with the data provided
// in initializers.

// OrtStatus * 	CreateOpAttr (const char *name, const void *data, int len, OrtOpAttrType type, OrtOpAttr **op_attr)
//  	: Create attribute of onnxruntime operator

// void 	ReleaseOpAttr (OrtOpAttr *input)

// OrtStatus * 	CreateOp (const OrtKernelInfo *info, const char *op_name, const char *domain, int version, const char
// **type_constraint_names, const ONNXTensorElementDataType *type_constraint_values, int type_constraint_count, const
// OrtOpAttr *const *attr_values, int attr_count, int input_count, int output_count, OrtOp **ort_op)  	: Create
// onnxruntime native operator

// OrtStatus * 	InvokeOp (const OrtKernelContext *context, const OrtOp *ort_op, const OrtValue *const *input_values, int
// input_count, OrtValue *const *output_values, int output_count)  	: Invoke the operator created by OrtApi::CreateOp The
// inputs must follow the order as specified in onnx specification

// void 	ReleaseOp (OrtOp *input)

// OrtStatus * 	SessionOptionsAppendExecutionProvider (OrtSessionOptions *options, const char *provider_name, const char
// *const *provider_options_keys, const char *const *provider_options_values, size_t num_keys)  	: Append execution
// provider to the session options.

// OrtStatus * 	CopyKernelInfo (const OrtKernelInfo *info, OrtKernelInfo **info_copy)

// void 	ReleaseKernelInfo (OrtKernelInfo *input)

// OrtStatus * 	SessionOptionsAppendExecutionProvider_CANN (OrtSessionOptions *options, const OrtCANNProviderOptions
// *cann_options)  	Append CANN provider to session options.

// OrtStatus * 	CreateCANNProviderOptions (OrtCANNProviderOptions **out)
//  	Create an OrtCANNProviderOptions.

// OrtStatus * 	UpdateCANNProviderOptions (OrtCANNProviderOptions *cann_options, const char *const
// *provider_options_keys, const char *const *provider_options_values, size_t num_keys)  	Set options in a CANN Execution
// Provider.

// OrtStatus * 	GetCANNProviderOptionsAsString (const OrtCANNProviderOptions *cann_options, OrtAllocator *allocator, char
// **ptr)  	Get serialized CANN provider options string.

// OrtStatus * 	UpdateEnvWithCustomLogLevel (OrtEnv *ort_env, OrtLoggingLevel log_severity_level)

// OrtStatus * 	SetGlobalIntraOpThreadAffinity (OrtThreadingOptions *tp_options, const char *affinity_string)

// OrtStatus * 	RegisterCustomOpsLibrary_V2 (OrtSessionOptions *options, const char *library_name)
//  	Register custom ops from a shared library.

// OrtStatus * 	RegisterCustomOpsUsingFunction (OrtSessionOptions *options, const char *registration_func_name)
//  	Register custom ops by calling a RegisterCustomOpsFn function.

// OrtStatus * 	SessionOptionsAppendExecutionProvider_Dnnl (OrtSessionOptions *options, const OrtDnnlProviderOptions
// *dnnl_options)  	Append dnnl provider to session options.

// OrtStatus * 	CreateDnnlProviderOptions (OrtDnnlProviderOptions **out)
//  	Create an OrtDnnlProviderOptions.

// OrtStatus * 	UpdateDnnlProviderOptions (OrtDnnlProviderOptions *dnnl_options, const char *const
// *provider_options_keys, const char *const *provider_options_values, size_t num_keys)  	Set options in a oneDNN
// Execution Provider.

// OrtStatus * 	GetDnnlProviderOptionsAsString (const OrtDnnlProviderOptions *dnnl_options, OrtAllocator *allocator, char
// **ptr)

// OrtStatus * 	KernelInfoGetConstantInput_tensor (const OrtKernelInfo *info, size_t index, int *is_constant, const
// OrtValue **out)  	Get a OrtValue tensor stored as a constant initializer in the graph node.

// OrtStatus * 	CastTypeInfoToOptionalTypeInfo (const OrtTypeInfo *type_info, const OrtOptionalTypeInfo **out)
//  	Get Optional Type information from an OrtTypeInfo.

// OrtStatus * 	GetOptionalContainedTypeInfo (const OrtOptionalTypeInfo *optional_type_info, OrtTypeInfo **out)
//  	Get OrtTypeInfo for the allowed contained type from an OrtOptionalTypeInfo.

// OrtStatus * 	GetResizedStringTensorElementBuffer (OrtValue *value, size_t index, size_t length_in_bytes, char
// **buffer)  	Set a single string in a string tensor Do not zero terminate the string data.

// OrtStatus * 	KernelContext_GetAllocator (const OrtKernelContext *context, const OrtMemoryInfo *mem_info, OrtAllocator
// **out)  	Get Allocator from KernelContext for a specific memoryInfo. Please use C API ReleaseAllocator to release out
// object.

// OrtModelMetadata
// OrtStatus * 	ModelMetadataGetProducerName (const OrtModelMetadata *model_metadata, OrtAllocator *allocator, char
// **value)  	Get producer name from an OrtModelMetadata.

// OrtStatus * 	ModelMetadataGetGraphName (const OrtModelMetadata *model_metadata, OrtAllocator *allocator, char **value)
//  	Get graph name from an OrtModelMetadata.

// OrtStatus * 	ModelMetadataGetDomain (const OrtModelMetadata *model_metadata, OrtAllocator *allocator, char **value)
//  	Get domain from an OrtModelMetadata.

// OrtStatus * 	ModelMetadataGetDescription (const OrtModelMetadata *model_metadata, OrtAllocator *allocator, char
// **value)  	Get description from an OrtModelMetadata.

// OrtStatus * 	ModelMetadataLookupCustomMetadataMap (const OrtModelMetadata *model_metadata, OrtAllocator *allocator,
// const char *key, char **value)  	Return data for a key in the custom metadata map in an OrtModelMetadata.

// OrtStatus * 	ModelMetadataGetVersion (const OrtModelMetadata *model_metadata, int64_t *value)
//  	Get version number from an OrtModelMetadata.

// void 	ReleaseModelMetadata (OrtModelMetadata *input)

// OrtStatus * 	ModelMetadataGetCustomMetadataMapKeys (const OrtModelMetadata *model_metadata, OrtAllocator *allocator,
// char ***keys, int64_t *num_keys)

// OrtStatus * 	ModelMetadataGetGraphDescription (const OrtModelMetadata *model_metadata, OrtAllocator *allocator, char
// **value)
