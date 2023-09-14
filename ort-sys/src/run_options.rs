// pub CreateRunOptions: ::std::option::Option<crate::ctypes::_system!(unsafe fn(out: *mut *mut OrtRunOptions) ->
// OrtStatusPtr)>, pub RunOptionsSetRunLogVerbosityLevel:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtRunOptions, log_verbosity_level:
// ::std::os::raw::c_int) -> OrtStatusPtr)>, pub RunOptionsSetRunLogSeverityLevel:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtRunOptions, log_severity_level:
// ::std::os::raw::c_int) -> OrtStatusPtr)>, pub RunOptionsSetRunTag:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtRunOptions, run_tag: *const
// ::std::os::raw::c_char) -> OrtStatusPtr)>, pub RunOptionsGetRunLogVerbosityLevel: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *const OrtRunOptions, log_verbosity_level: *mut ::std::os::raw::c_int)
// -> OrtStatusPtr) >,
// pub RunOptionsGetRunLogSeverityLevel: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(options: *const OrtRunOptions, log_severity_level: *mut ::std::os::raw::c_int)
// -> OrtStatusPtr) >,
// pub RunOptionsGetRunTag:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *const OrtRunOptions, run_tag: *mut *const
// ::std::os::raw::c_char) -> OrtStatusPtr)>, pub RunOptionsSetTerminate:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtRunOptions) -> OrtStatusPtr)>,
// pub RunOptionsUnsetTerminate: ::std::option::Option<crate::ctypes::_system!(unsafe fn(options: *mut OrtRunOptions) ->
// OrtStatusPtr)>
// pub ReleaseRunOptions: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtRunOptions))>,
// pub AddRunConfigEntry: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(options: *mut OrtRunOptions, config_key: *const ::std::os::raw::c_char, config_value: *const
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
