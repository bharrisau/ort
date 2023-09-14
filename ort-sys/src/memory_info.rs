//, pub CreateMemoryInfo: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             name: *const ::std::os::raw::c_char,
//             type_: OrtAllocatorType,
//             id: ::std::os::raw::c_int,
//             mem_type: OrtMemType,
//             out: *mut *mut OrtMemoryInfo
//         ) -> OrtStatusPtr
//     )
// >,
// pub CreateCpuMemoryInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(type_: OrtAllocatorType, mem_type: OrtMemType, out: *mut
// *mut OrtMemoryInfo) -> OrtStatusPtr)>, pub CompareMemoryInfo: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(info1: *const OrtMemoryInfo, info2: *const OrtMemoryInfo, out: *mut
// ::std::os::raw::c_int) -> OrtStatusPtr) >,
// pub MemoryInfoGetName:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(ptr: *const OrtMemoryInfo, out: *mut *const
// ::std::os::raw::c_char) -> OrtStatusPtr)>, pub MemoryInfoGetId: ::std::option::Option<crate::ctypes::_system!(unsafe
// fn(ptr: *const OrtMemoryInfo, out: *mut ::std::os::raw::c_int) -> OrtStatusPtr)>, pub MemoryInfoGetMemType:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(ptr: *const OrtMemoryInfo, out: *mut OrtMemType) ->
// OrtStatusPtr)>, pub MemoryInfoGetType: ::std::option::Option<crate::ctypes::_system!(unsafe fn(ptr: *const
// OrtMemoryInfo, out: *mut OrtAllocatorType) -> OrtStatusPtr)>
// pub ReleaseMemoryInfo: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtMemoryInfo))>,
