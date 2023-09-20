use crate::{onnx_call, Api, Result};

impl Api {}

// pub CreateSession: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(env: *const OrtEnv, model_path: *const ORTCHAR_T, options: *const OrtSessionOptions, out: *mut *mut
// OrtSession) -> OrtStatusPtr     )
// >,
// pub CreateSessionFromArray: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             env: *const OrtEnv,
//             model_data: *const ::std::os::raw::c_void,
//             model_data_length: size_t,
//             options: *const OrtSessionOptions,
//             out: *mut *mut OrtSession
//         ) -> OrtStatusPtr
//     )
// >,
// pub Run: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             session: *mut OrtSession,
//             run_options: *const OrtRunOptions,
//             input_names: *const *const ::std::os::raw::c_char,
//             inputs: *const *const OrtValue,
//             input_len: size_t,
//             output_names: *const *const ::std::os::raw::c_char,
//             output_names_len: size_t,
//             outputs: *mut *mut OrtValue
//         ) -> OrtStatusPtr
//     )
// >,
// pub SessionGetInputCount: ::std::option::Option<crate::ctypes::_system!(unsafe fn(session: *const OrtSession, out:
// *mut size_t) -> OrtStatusPtr)>, pub SessionGetOutputCount: ::std::option::Option<crate::ctypes::_system!(unsafe
// fn(session: *const OrtSession, out: *mut size_t) -> OrtStatusPtr)>, pub SessionGetOverridableInitializerCount:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(session: *const OrtSession, out: *mut size_t) ->
// OrtStatusPtr)>, pub SessionGetInputTypeInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(session: *const OrtSession, index: size_t, type_info:
// *mut *mut OrtTypeInfo) -> OrtStatusPtr)>, pub SessionGetOutputTypeInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(session: *const OrtSession, index: size_t, type_info:
// *mut *mut OrtTypeInfo) -> OrtStatusPtr)>, pub SessionGetOverridableInitializerTypeInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(session: *const OrtSession, index: size_t, type_info:
// *mut *mut OrtTypeInfo) -> OrtStatusPtr)>, pub SessionGetInputName: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(session: *const OrtSession, index: size_t, allocator: *mut OrtAllocator, value: *mut *mut
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
// pub SessionGetOutputName: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(session: *const OrtSession, index: size_t, allocator: *mut OrtAllocator, value: *mut *mut
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
// pub SessionGetOverridableInitializerName: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(session: *const OrtSession, index: size_t, allocator: *mut OrtAllocator, value: *mut *mut
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
// pub ReleaseSession: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtSession))>,
//, pub SessionEndProfiling: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(session: *mut OrtSession, allocator: *mut OrtAllocator, out: *mut *mut
// ::std::os::raw::c_char) -> OrtStatusPtr) >,
// pub SessionGetModelMetadata:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(session: *const OrtSession, out: *mut *mut
// OrtModelMetadata) -> OrtStatusPtr)>, pub ModelMetadataGetProducerName: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(model_metadata: *const OrtModelMetadata, allocator: *mut OrtAllocator, value: *mut *mut
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
// pub ModelMetadataGetGraphName: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(model_metadata: *const OrtModelMetadata, allocator: *mut OrtAllocator, value: *mut *mut
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
// pub ModelMetadataGetDomain: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(model_metadata: *const OrtModelMetadata, allocator: *mut OrtAllocator, value: *mut *mut
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
// pub ModelMetadataGetDescription: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(model_metadata: *const OrtModelMetadata, allocator: *mut OrtAllocator, value: *mut *mut
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
// pub ModelMetadataLookupCustomMetadataMap: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             model_metadata: *const OrtModelMetadata,
//             allocator: *mut OrtAllocator,
//             key: *const ::std::os::raw::c_char,
//             value: *mut *mut ::std::os::raw::c_char
//         ) -> OrtStatusPtr
//     )
// >,
// pub ModelMetadataGetVersion:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(model_metadata: *const OrtModelMetadata, value: *mut i64)
// -> OrtStatusPtr)>, pub ReleaseModelMetadata: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut
// OrtModelMetadata))>,
// pub CreatePrepackedWeightsContainer: ::std::option::Option<crate::ctypes::_system!(unsafe fn(out: *mut *mut
// OrtPrepackedWeightsContainer) -> OrtStatusPtr)>, pub ReleasePrepackedWeightsContainer:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtPrepackedWeightsContainer))>,
// pub CreateSessionWithPrepackedWeightsContainer: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             env: *const OrtEnv,
//             model_path: *const ORTCHAR_T,
//             options: *const OrtSessionOptions,
//             prepacked_weights_container: *mut OrtPrepackedWeightsContainer,
//             out: *mut *mut OrtSession
//         ) -> OrtStatusPtr
//     )
// >,
// pub CreateSessionFromArrayWithPrepackedWeightsContainer: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             env: *const OrtEnv,
//             model_data: *const ::std::os::raw::c_void,
//             model_data_length: size_t,
//             options: *const OrtSessionOptions,
//             prepacked_weights_container: *mut OrtPrepackedWeightsContainer,
//             out: *mut *mut OrtSession
//         ) -> OrtStatusPtr
//     )
// >,
// pub RunWithBinding: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(session: *mut OrtSession, run_options: *const OrtRunOptions, binding_ptr:
// *const OrtIoBinding) -> OrtStatusPtr) >,
// pub CreateIoBinding: ::std::option::Option<crate::ctypes::_system!(unsafe fn(session: *mut OrtSession, out: *mut *mut
// OrtIoBinding) -> OrtStatusPtr)>, pub ReleaseIoBinding: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input:
// *mut OrtIoBinding))>,
//, pub SessionGetProfilingStartTimeNs:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(session: *const OrtSession, out: *mut u64) -> OrtStatusPtr)>,

// OrtStatus * 	CreateSession (const OrtEnv *env, const char *model_path, const OrtSessionOptions *options, OrtSession
// **out)  	Create an OrtSession from a model file.

// OrtStatus * 	CreateSessionFromArray (const OrtEnv *env, const void *model_data, size_t model_data_length, const
// OrtSessionOptions *options, OrtSession **out)  	Create an OrtSession from memory.

// OrtStatus * 	Run (OrtSession *session, const OrtRunOptions *run_options, const char *const *input_names, const
// OrtValue *const *inputs, size_t input_len, const char *const *output_names, size_t output_names_len, OrtValue
// **outputs)  	Run the model in an OrtSession.

// OrtStatus * 	SessionGetInputCount (const OrtSession *session, size_t *out)
//  	Get input count for a session.

// OrtStatus * 	SessionGetOutputCount (const OrtSession *session, size_t *out)
//  	Get output count for a session.

// OrtStatus * 	SessionGetOverridableInitializerCount (const OrtSession *session, size_t *out)
//  	Get overridable initializer count.

// OrtStatus * 	SessionGetInputTypeInfo (const OrtSession *session, size_t index, OrtTypeInfo **type_info)
//  	Get input type information.

// OrtStatus * 	SessionGetOutputTypeInfo (const OrtSession *session, size_t index, OrtTypeInfo **type_info)
//  	Get output type information.

// OrtStatus * 	SessionGetOverridableInitializerTypeInfo (const OrtSession *session, size_t index, OrtTypeInfo
// **type_info)  	Get overridable initializer type information.

// OrtStatus * 	SessionGetInputName (const OrtSession *session, size_t index, OrtAllocator *allocator, char **value)
//  	Get input name.

// OrtStatus * 	SessionGetOutputName (const OrtSession *session, size_t index, OrtAllocator *allocator, char **value)
//  	Get output name.

// OrtStatus * 	SessionGetOverridableInitializerName (const OrtSession *session, size_t index, OrtAllocator *allocator,
// char **value)  	Get overridable initializer name.

// void 	ReleaseSession (OrtSession *input)

// OrtStatus * 	SessionEndProfiling (OrtSession *session, OrtAllocator *allocator, char **out)
//  	End profiling and return filename of the profile data.

// OrtStatus * 	SessionGetModelMetadata (const OrtSession *session, OrtModelMetadata **out)
//  	Get OrtModelMetadata from an OrtSession.

// OrtStatus * 	RunWithBinding (OrtSession *session, const OrtRunOptions *run_options, const OrtIoBinding *binding_ptr)
//  	Run a model using Io Bindings for the inputs & outputs.

// OrtStatus * 	CreateIoBinding (OrtSession *session, OrtIoBinding **out)
//  	Create an OrtIoBinding instance.

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
