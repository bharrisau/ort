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
