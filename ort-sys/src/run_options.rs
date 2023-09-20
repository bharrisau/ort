use crate::{onnx_call, Api, Result};

impl Api {}

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

// OrtStatus * 	CreateRunOptions (OrtRunOptions **out)
//  	Create an OrtRunOptions.

// OrtStatus * 	RunOptionsSetRunLogVerbosityLevel (OrtRunOptions *options, int log_verbosity_level)
//  	Set per-run log verbosity level.

// OrtStatus * 	RunOptionsSetRunLogSeverityLevel (OrtRunOptions *options, int log_severity_level)
//  	Set per-run log severity level.

// OrtStatus * 	RunOptionsSetRunTag (OrtRunOptions *options, const char *run_tag)
//  	Set per-run tag.

// OrtStatus * 	RunOptionsGetRunLogVerbosityLevel (const OrtRunOptions *options, int *log_verbosity_level)
//  	Get per-run log verbosity level.

// OrtStatus * 	RunOptionsGetRunLogSeverityLevel (const OrtRunOptions *options, int *log_severity_level)
//  	Get per-run log severity level.

// OrtStatus * 	RunOptionsGetRunTag (const OrtRunOptions *options, const char **run_tag)
//  	Get per-run tag.

// OrtStatus * 	RunOptionsSetTerminate (OrtRunOptions *options)
//  	Set terminate flag.

// OrtStatus * 	RunOptionsUnsetTerminate (OrtRunOptions *options)
//  	Clears the terminate flag.

// void 	ReleaseRunOptions (OrtRunOptions *input)

// OrtStatus * 	AddRunConfigEntry (OrtRunOptions *options, const char *config_key, const char *config_value)
//  	Set a single run configuration entry as a pair of strings.
