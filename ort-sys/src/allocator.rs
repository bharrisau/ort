use std::{
	ffi::c_void,
	ops::Deref,
	sync::{Arc, Mutex}
};

use crate::{ctypes::size_t, onnx_call, sys::OrtAllocator, Api, Result};

impl Api {}

//, pub AllocatorAlloc: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(ort_allocator: *mut OrtAllocator, size: size_t, out: *mut *mut
// ::std::os::raw::c_void) -> OrtStatusPtr) >,
// pub AllocatorFree:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(ort_allocator: *mut OrtAllocator, p: *mut
// ::std::os::raw::c_void) -> OrtStatusPtr)>, pub AllocatorGetInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(ort_allocator: *const OrtAllocator, out: *mut *const
// OrtMemoryInfo) -> OrtStatusPtr)>, pub GetAllocatorWithDefaultOptions:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(out: *mut *mut OrtAllocator) -> OrtStatusPtr)>,

// pub CreateAllocator: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(session: *const OrtSession, mem_info: *const OrtMemoryInfo, out: *mut *mut
// OrtAllocator) -> OrtStatusPtr) >,
// pub ReleaseAllocator: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtAllocator))>,
// pub CreateAndRegisterAllocator: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(env: *mut OrtEnv, mem_info: *const OrtMemoryInfo, arena_cfg: *const
// OrtArenaCfg) -> OrtStatusPtr) >,

// pub ReleaseArenaCfg: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtArenaCfg))>,

// pub CreateArenaCfgV2: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             arena_config_keys: *const *const ::std::os::raw::c_char,
//             arena_config_values: *const size_t,
//             num_keys: size_t,
//             out: *mut *mut OrtArenaCfg
//         ) -> OrtStatusPtr
//     )
// >,
// pub RegisterAllocator:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(env: *mut OrtEnv, allocator: *mut OrtAllocator) ->
// OrtStatusPtr)>, pub UnregisterAllocator: ::std::option::Option<crate::ctypes::_system!(unsafe fn(env: *mut OrtEnv,
// mem_info: *const OrtMemoryInfo) -> OrtStatusPtr)>,
// pub CreateArenaCfg: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             max_mem: size_t,
//             arena_extend_strategy: ::std::os::raw::c_int,
//             initial_chunk_size_bytes: ::std::os::raw::c_int,
//             max_dead_bytes_per_chunk: ::std::os::raw::c_int,
//             out: *mut *mut OrtArenaCfg
//         ) -> OrtStatusPtr
//     )
// >,

// OrtStatus * 	AllocatorAlloc (OrtAllocator *ort_allocator, size_t size, void **out)
//  	Calls OrtAllocator::Alloc function.

// OrtStatus * 	AllocatorFree (OrtAllocator *ort_allocator, void *p)
//  	Calls OrtAllocator::Free function.

// OrtStatus * 	AllocatorGetInfo (const OrtAllocator *ort_allocator, const struct OrtMemoryInfo **out)
//  	Calls OrtAllocator::Info function.

// OrtStatus * 	GetAllocatorWithDefaultOptions (OrtAllocator **out)
//  	Get the default allocator.

// OrtStatus * 	CreateAllocator (const OrtSession *session, const OrtMemoryInfo *mem_info, OrtAllocator **out)
//  	Create an allocator for an OrtSession following an OrtMemoryInfo.

// void 	ReleaseAllocator (OrtAllocator *input)
//  	Release an OrtAllocator obtained from OrtApi::CreateAllocator.

// OrtStatus * 	RegisterAllocator (OrtEnv *env, OrtAllocator *allocator)
//  	Register a custom allocator.

// OrtStatus * 	UnregisterAllocator (OrtEnv *env, const OrtMemoryInfo *mem_info)
//  	Unregister a custom allocator.

pub struct Allocator {
	inner: Arc<Mutex<AllocatorInner>>
}

#[repr(transparent)]
pub struct AllocatorInner {
	ptr: *mut OrtAllocator
}

impl Allocator {
	pub fn free<T>(&self, value: *const T) {
		todo!()
	}

	pub fn with_ptr<T>(&self, f: impl FnOnce(*mut OrtAllocator) -> T) -> T {
		let inner = self.inner.lock().unwrap();
		f(inner.ptr)
	}

	pub fn as_ptr(&self) -> *const OrtAllocator {
		let inner = self.inner.lock().unwrap();
		inner.ptr
	}

	#[allow(clippy::useless_conversion)]
	pub unsafe fn to_slice<'a, T>(&self, ptr: *const T, len: size_t) -> AllocatorValue<&'a [T]> {
		let len = len.try_into().expect("unable to handle size_t value");
		let slice = std::slice::from_raw_parts(ptr, len);
		AllocatorValue {
			value: slice,
			alloc: std::ptr::null_mut()
		}
	}
}

impl Drop for Allocator {
	fn drop(&mut self) {
		// Api::get().allocator_release(self)
	}
}

pub struct AllocatorValue<T> {
	value: T,
	alloc: *mut c_void
}

impl<T> AllocatorValue<T> {
	pub(crate) fn create(value: T, allocator: Allocator, allocation: *const c_void) -> Self {
		todo!()
	}
}

impl<T> Deref for AllocatorValue<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T> Drop for AllocatorValue<T> {
	fn drop(&mut self) {
		todo!()
	}
}
