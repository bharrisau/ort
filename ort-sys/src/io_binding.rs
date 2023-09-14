// pub BindInput: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(binding_ptr: *mut OrtIoBinding, name: *const ::std::os::raw::c_char, val_ptr:
// *const OrtValue) -> OrtStatusPtr) >,
// pub BindOutput: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(binding_ptr: *mut OrtIoBinding, name: *const ::std::os::raw::c_char, val_ptr:
// *const OrtValue) -> OrtStatusPtr) >,
// pub BindOutputToDevice: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(binding_ptr: *mut OrtIoBinding, name: *const ::std::os::raw::c_char, mem_info_ptr: *const
// OrtMemoryInfo) -> OrtStatusPtr     )
// >,
// pub GetBoundOutputNames: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             binding_ptr: *const OrtIoBinding,
//             allocator: *mut OrtAllocator,
//             buffer: *mut *mut ::std::os::raw::c_char,
//             lengths: *mut *mut size_t,
//             count: *mut size_t
//         ) -> OrtStatusPtr
//     )
// >,
// pub GetBoundOutputValues: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             binding_ptr: *const OrtIoBinding,
//             allocator: *mut OrtAllocator,
//             output: *mut *mut *mut OrtValue,
//             output_count: *mut size_t
//         ) -> OrtStatusPtr
//     )
// >,
// #[doc = " \\brief Clears any previously set Inputs for an ::OrtIoBinding"]
// pub ClearBoundInputs: ::std::option::Option<crate::ctypes::_system!(unsafe fn(binding_ptr: *mut OrtIoBinding))>,
// #[doc = " \\brief Clears any previously set Outputs for an ::OrtIoBinding"]
// pub ClearBoundOutputs: ::std::option::Option<crate::ctypes::_system!(unsafe fn(binding_ptr: *mut OrtIoBinding))>,
