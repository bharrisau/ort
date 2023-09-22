use std::{
	cell::UnsafeCell,
	collections::{hash_map::Entry, HashMap},
	ffi::c_char,
	ops::Deref
};

use crate::{
	allocator::Allocator,
	ctypes::size_t,
	memory_info::MemoryInfo,
	onnx_call,
	string::OnnxString,
	sys::{OrtIoBinding, OrtValue},
	value::Value,
	Api, Result
};

pub struct IoBinding {
	ptr: *mut OrtIoBinding,
	input_mapping: HashMap<IobEntry, usize>,
	inputs: Vec<(OnnxString, Value)>,
	output_mapping: HashMap<IobEntry, usize>,
	outputs: Vec<(OnnxString, Option<Value>)>,
	dirty_outputs: bool
}

impl IoBinding {
	pub(crate) fn from_ptr(ptr: *mut OrtIoBinding) -> Self {
		IoBinding {
			ptr,
			input_mapping: Default::default(),
			inputs: Default::default(),
			output_mapping: Default::default(),
			outputs: Default::default(),
			dirty_outputs: false
		}
	}

	pub(crate) fn as_ptr(&self) -> *mut OrtIoBinding {
		self.ptr
	}

	pub fn clear_bound_inputs(&mut self) -> Result<()> {
		self.inputs.clear();
		self.input_mapping.clear();
		onnx_call!(Api::get_ptr(), ClearBoundInputs(self.ptr) -> ())
	}

	pub fn clear_bound_outputs(&mut self) -> Result<()> {
		self.outputs.clear();
		self.output_mapping.clear();
		onnx_call!(Api::get_ptr(), ClearBoundOutputs(self.ptr) -> ())
	}

	#[allow(clippy::mutable_key_type)]
	fn emplace(map: &mut HashMap<IobEntry, usize>, name: impl AsRef<str>, default: usize) -> std::result::Result<OnnxString, usize> {
		let query: &'static str = unsafe { std::mem::transmute(name.as_ref()) };
		match map.entry(query.into()) {
			Entry::Occupied(e) => Err(*e.get()),
			Entry::Vacant(e) => {
				let key = e.key().upgrade().clone();
				e.insert(default);
				Ok(key)
			}
		}
	}

	pub fn bind_input(&mut self, name: impl AsRef<str>, value: Value) -> Result<()> {
		let default = self.inputs.len();
		let index = match Self::emplace(&mut self.input_mapping, name, default) {
			Ok(name) => {
				self.inputs.push((name, value));
				self.inputs.len() - 1
			}
			Err(index) => {
				self.inputs[index].1 = value;
				index
			}
		};

		let (name, value) = &self.inputs[index];

		onnx_call!(Api::get_ptr(), BindInput(self.ptr, name.as_ptr(), value.as_ptr()))
	}

	pub fn bind_output(&mut self, name: impl AsRef<str>, value: Value) -> Result<()> {
		let default = self.outputs.len();
		let index = match Self::emplace(&mut self.output_mapping, name, default) {
			Ok(name) => {
				self.outputs.push((name, Some(value)));
				self.outputs.len() - 1
			}
			Err(index) => {
				self.outputs[index].1 = Some(value);
				index
			}
		};

		let (name, value) = &self.outputs[index];

		onnx_call!(Api::get_ptr(), BindOutput(self.ptr, name.as_ptr(), value.as_ref().unwrap().as_ptr()))
	}

	pub fn bind_output_to_device(&mut self, name: impl AsRef<str>, mem: &MemoryInfo) -> Result<()> {
		self.dirty_outputs = true;
		let default = self.outputs.len();
		let index = match Self::emplace(&mut self.output_mapping, name, default) {
			Ok(name) => {
				self.outputs.push((name, None));
				self.outputs.len() - 1
			}
			Err(index) => {
				self.outputs[index].1 = None;
				index
			}
		};

		let name = &self.outputs[index].0;

		onnx_call!(Api::get_ptr(), BindOutputToDevice(self.ptr, name.as_ptr(), mem.ptr))
	}

	pub(crate) fn update_outputs(&mut self, alloc: &Allocator) -> Result<()> {
		if !self.dirty_outputs {
			// We shotcut here as we don't need to get the values
			return Ok(());
		}

		//#[cfg(debug)]
		{
			let mut buffer: *mut c_char = std::ptr::null_mut();
			let mut lengths: *mut size_t = std::ptr::null_mut();
			let mut count: size_t = 0;

			alloc.with_ptr(|alloc_ptr| onnx_call!(Api::get_ptr(), GetBoundOutputNames(self.ptr, alloc_ptr, &mut buffer, &mut lengths, &mut count)))?;
			let length_slice = unsafe { alloc.to_slice(lengths, count) };
			let total_len: size_t = length_slice.iter().sum();
			let char_slice = unsafe { alloc.to_slice(buffer as *mut u8, total_len) };

			let valid_names = length_slice
				.iter()
				.zip(self.outputs.iter())
				.scan(*char_slice.deref(), |buf, (len, (rust_name, _value))| {
					let (onnx_name, state) = buf.split_at(*len);
					*buf = state;
					let onnx_name = unsafe { std::str::from_utf8_unchecked(onnx_name) };

					Some(rust_name.as_str().eq(onnx_name))
				})
				.all(|x| x);
			assert!(valid_names, "unable to match local names with onnx");
		}

		let mut values: *mut *mut OrtValue = std::ptr::null_mut();
		let mut count: size_t = 0;

		alloc.with_ptr(|alloc_ptr| onnx_call!(Api::get_ptr(), GetBoundOutputValues(self.ptr, alloc_ptr, &mut values, &mut count)))?;
		let value_slice = unsafe { alloc.to_slice(values, count) };

		for (value, (_name, option)) in value_slice.iter().zip(self.outputs.iter_mut()) {
			let value = Value::from_raw(*value);
			if option.is_none() {
				*option = Some(value);
			}
		}

		self.dirty_outputs = false;

		Ok(())
	}

	// TODO - use the type system to enforce the ending nul byte
	pub fn get_outputs(&self) -> impl Iterator<Item = (&str, &Value)> {
		assert!(!self.dirty_outputs, "tried to get outputs without running bindings");
		self.outputs.iter().map(|(name, value)| (name.as_str(), value.as_ref().unwrap()))
	}
}

impl Drop for IoBinding {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			onnx_call!(Api::get_ptr(), ReleaseIoBinding(self.ptr) -> ()).expect("unable to release io binding");
			self.ptr = std::ptr::null_mut();
		}
	}
}

#[repr(transparent)]
struct IobEntry(UnsafeCell<IobInner>);

impl IobEntry {
	pub fn upgrade(&self) -> &OnnxString {
		let inner = unsafe { &mut *self.0.get() };
		inner.upgrade()
	}
}

impl PartialEq for IobEntry {
	fn eq(&self, other: &Self) -> bool {
		let inner = unsafe { &*self.0.get() };
		let other = unsafe { &*other.0.get() };
		inner.eq(other)
	}
}

impl Eq for IobEntry {}

impl std::hash::Hash for IobEntry {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		let inner = unsafe { &*self.0.get() };
		inner.hash(state);
	}
}

enum IobInner {
	Query(&'static str),
	Owned(OnnxString)
}

impl IobInner {
	fn as_str(&self) -> &str {
		match self {
			IobInner::Query(s) => s,
			IobInner::Owned(r) => r.as_str()
		}
	}

	fn upgrade(&mut self) -> &OnnxString {
		match self {
			IobInner::Query(s) => {
				OnnxString::prepare(s.len() + 1);
				*self = IobInner::Owned(OnnxString::to_rc(s));

				if let IobInner::Owned(rc) = self { rc } else { unreachable!() }
			}
			IobInner::Owned(e) => e
		}
	}
}

impl PartialEq for IobInner {
	fn eq(&self, other: &Self) -> bool {
		self.as_str().eq(other.as_str())
	}
}

impl Eq for IobInner {}

impl std::hash::Hash for IobInner {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.as_str().hash(state);
	}
}

impl From<&'static str> for IobEntry {
	fn from(value: &'static str) -> Self {
		IobEntry(UnsafeCell::new(IobInner::Query(value)))
	}
}
