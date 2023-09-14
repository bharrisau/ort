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
