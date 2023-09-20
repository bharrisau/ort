use crate::{onnx_call, sys::OrtValue, Api, Result};

impl Api {}

//, pub CreateTensorAsOrtValue: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             allocator: *mut OrtAllocator,
//             shape: *const i64,
//             shape_len: size_t,
//             type_: ONNXTensorElementDataType,
//             out: *mut *mut OrtValue
//         ) -> OrtStatusPtr
//     )
// >,
// pub CreateTensorWithDataAsOrtValue: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             info: *const OrtMemoryInfo,
//             p_data: *mut ::std::os::raw::c_void,
//             p_data_len: size_t,
//             shape: *const i64,
//             shape_len: size_t,
//             type_: ONNXTensorElementDataType,
//             out: *mut *mut OrtValue
//         ) -> OrtStatusPtr
//     )
// >,
// pub IsTensor: ::std::option::Option<crate::ctypes::_system!(unsafe fn(value: *const OrtValue, out: *mut
// ::std::os::raw::c_int) -> OrtStatusPtr)>, pub GetTensorMutableData:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(value: *mut OrtValue, out: *mut *mut
// ::std::os::raw::c_void) -> OrtStatusPtr)>, pub FillStringTensor:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(value: *mut OrtValue, s: *const *const
// ::std::os::raw::c_char, s_len: size_t) -> OrtStatusPtr)>, pub GetStringTensorDataLength:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(value: *const OrtValue, len: *mut size_t) -> OrtStatusPtr)>,
// pub GetStringTensorContent: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(value: *const OrtValue, s: *mut ::std::os::raw::c_void, s_len: size_t, offsets: *mut size_t,
// offsets_len: size_t) -> OrtStatusPtr     )
// >,
// pub CastTypeInfoToTensorInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(type_info: *const OrtTypeInfo, out: *mut *const
// OrtTensorTypeAndShapeInfo) -> OrtStatusPtr)>, pub GetOnnxTypeFromTypeInfo:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(type_info: *const OrtTypeInfo, out: *mut ONNXType) ->
// OrtStatusPtr)>, pub CreateTensorTypeAndShapeInfo: ::std::option::Option<crate::ctypes::_system!(unsafe fn(out: *mut
// *mut OrtTensorTypeAndShapeInfo) -> OrtStatusPtr)>, pub SetTensorElementType:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(info: *mut OrtTensorTypeAndShapeInfo, type_:
// ONNXTensorElementDataType) -> OrtStatusPtr)>, pub SetDimensions: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(info: *mut OrtTensorTypeAndShapeInfo, dim_values: *const i64, dim_count:
// size_t) -> OrtStatusPtr) >,
// pub GetTensorElementType:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(info: *const OrtTensorTypeAndShapeInfo, out: *mut
// ONNXTensorElementDataType) -> OrtStatusPtr)>, pub GetDimensionsCount:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(info: *const OrtTensorTypeAndShapeInfo, out: *mut size_t) ->
// OrtStatusPtr)>, pub GetDimensions: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(info: *const OrtTensorTypeAndShapeInfo, dim_values: *mut i64,
// dim_values_length: size_t) -> OrtStatusPtr) >,
// pub GetSymbolicDimensions: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(info: *const OrtTensorTypeAndShapeInfo, dim_params: *mut *const ::std::os::raw::c_char,
// dim_params_length: size_t) -> OrtStatusPtr     )
// >,
// pub GetTensorShapeElementCount:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(info: *const OrtTensorTypeAndShapeInfo, out: *mut size_t)
// -> OrtStatusPtr)>, pub GetTensorTypeAndShape:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(value: *const OrtValue, out: *mut *mut
// OrtTensorTypeAndShapeInfo) -> OrtStatusPtr)>, pub GetTypeInfo: ::std::option::Option<crate::ctypes::_system!(unsafe
// fn(value: *const OrtValue, out: *mut *mut OrtTypeInfo) -> OrtStatusPtr)>, pub GetValueType:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(value: *const OrtValue, out: *mut ONNXType) ->
// OrtStatusPtr)>
// pub GetValue: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(value: *const OrtValue, index: ::std::os::raw::c_int, allocator: *mut OrtAllocator, out: *mut *mut
// OrtValue) -> OrtStatusPtr     )
// >,
// pub GetValueCount: ::std::option::Option<crate::ctypes::_system!(unsafe fn(value: *const OrtValue, out: *mut size_t)
// -> OrtStatusPtr)>, pub CreateValue: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(in_: *const *const OrtValue, num_values: size_t, value_type: ONNXType, out:
// *mut *mut OrtValue) -> OrtStatusPtr) >,
// pub CreateOpaqueValue: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             domain_name: *const ::std::os::raw::c_char,
//             type_name: *const ::std::os::raw::c_char,
//             data_container: *const ::std::os::raw::c_void,
//             data_container_size: size_t,
//             out: *mut *mut OrtValue
//         ) -> OrtStatusPtr
//     )
// >,
// pub GetOpaqueValue: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             domain_name: *const ::std::os::raw::c_char,
//             type_name: *const ::std::os::raw::c_char,
//             in_: *const OrtValue,
//             data_container: *mut ::std::os::raw::c_void,
//             data_container_size: size_t
//         ) -> OrtStatusPtr
//     )
// >,
// pub ReleaseValue: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtValue))>,
// pub ReleaseTypeInfo: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtTypeInfo))>,
// pub ReleaseTensorTypeAndShapeInfo: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut
// OrtTensorTypeAndShapeInfo))>,
// pub GetDenotationFromTypeInfo: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(type_info: *const OrtTypeInfo, denotation: *mut *const ::std::os::raw::c_char,
// len: *mut size_t) -> OrtStatusPtr) >,
// pub CastTypeInfoToMapTypeInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(type_info: *const OrtTypeInfo, out: *mut *const
// OrtMapTypeInfo) -> OrtStatusPtr)>, pub CastTypeInfoToSequenceTypeInfo:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(type_info: *const OrtTypeInfo, out: *mut *const
// OrtSequenceTypeInfo) -> OrtStatusPtr)>, pub GetMapKeyType:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(map_type_info: *const OrtMapTypeInfo, out: *mut
// ONNXTensorElementDataType) -> OrtStatusPtr)>, pub GetMapValueType:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(map_type_info: *const OrtMapTypeInfo, type_info: *mut
// *mut OrtTypeInfo) -> OrtStatusPtr)>, pub GetSequenceElementType: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(sequence_type_info: *const OrtSequenceTypeInfo, type_info: *mut *mut
// OrtTypeInfo) -> OrtStatusPtr) >,
// pub ReleaseMapTypeInfo: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut OrtMapTypeInfo))>,
// pub ReleaseSequenceTypeInfo: ::std::option::Option<crate::ctypes::_system!(unsafe fn(input: *mut
// OrtSequenceTypeInfo))>
// pub GetStringTensorElementLength:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(value: *const OrtValue, index: size_t, out: *mut size_t)
// -> OrtStatusPtr)>, pub GetStringTensorElement: ::std::option::Option<
//     crate::ctypes::_system!(unsafe fn(value: *const OrtValue, s_len: size_t, index: size_t, s: *mut
// ::std::os::raw::c_void) -> OrtStatusPtr) >,
// pub FillStringTensorElement:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(value: *mut OrtValue, s: *const ::std::os::raw::c_char,
// index: size_t) -> OrtStatusPtr)>, pub AddSessionConfigEntry: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(options: *mut OrtSessionOptions, config_key: *const ::std::os::raw::c_char, config_value: *const
// ::std::os::raw::c_char) -> OrtStatusPtr     )
// >,
// pub TensorAt: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(value: *mut OrtValue, location_values: *const i64, location_values_count: size_t, out: *mut *mut
// ::std::os::raw::c_void) -> OrtStatusPtr     )
// >,
// pub IsSparseTensor:
// ::std::option::Option<crate::ctypes::_system!(unsafe fn(value: *const OrtValue, out: *mut ::std::os::raw::c_int) ->
// OrtStatusPtr)>, pub CreateSparseTensorAsOrtValue: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             allocator: *mut OrtAllocator,
//             dense_shape: *const i64,
//             dense_shape_len: size_t,
//             type_: ONNXTensorElementDataType,
//             out: *mut *mut OrtValue
//         ) -> OrtStatusPtr
//     )
// >,
// pub FillSparseTensorCoo: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             ort_value: *mut OrtValue,
//             data_mem_info: *const OrtMemoryInfo,
//             values_shape: *const i64,
//             values_shape_len: size_t,
//             values: *const ::std::os::raw::c_void,
//             indices_data: *const i64,
//             indices_num: size_t
//         ) -> OrtStatusPtr
//     )
// >,
// pub FillSparseTensorCsr: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             ort_value: *mut OrtValue,
//             data_mem_info: *const OrtMemoryInfo,
//             values_shape: *const i64,
//             values_shape_len: size_t,
//             values: *const ::std::os::raw::c_void,
//             inner_indices_data: *const i64,
//             inner_indices_num: size_t,
//             outer_indices_data: *const i64,
//             outer_indices_num: size_t
//         ) -> OrtStatusPtr
//     )
// >,
// pub FillSparseTensorBlockSparse: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             ort_value: *mut OrtValue,
//             data_mem_info: *const OrtMemoryInfo,
//             values_shape: *const i64,
//             values_shape_len: size_t,
//             values: *const ::std::os::raw::c_void,
//             indices_shape_data: *const i64,
//             indices_shape_len: size_t,
//             indices_data: *const i32
//         ) -> OrtStatusPtr
//     )
// >,
// pub CreateSparseTensorWithValuesAsOrtValue: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             info: *const OrtMemoryInfo,
//             p_data: *mut ::std::os::raw::c_void,
//             dense_shape: *const i64,
//             dense_shape_len: size_t,
//             values_shape: *const i64,
//             values_shape_len: size_t,
//             type_: ONNXTensorElementDataType,
//             out: *mut *mut OrtValue
//         ) -> OrtStatusPtr
//     )
// >,
// pub UseCooIndices:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(ort_value: *mut OrtValue, indices_data: *mut i64,
// indices_num: size_t) -> OrtStatusPtr)>, pub UseCsrIndices: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(ort_value: *mut OrtValue, inner_data: *mut i64, inner_num: size_t, outer_data: *mut i64, outer_num:
// size_t) -> OrtStatusPtr     )
// >,
// pub UseBlockSparseIndices: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(ort_value: *mut OrtValue, indices_shape: *const i64, indices_shape_len: size_t, indices_data: *mut
// i32) -> OrtStatusPtr     )
// >,
// pub GetSparseTensorFormat: ::std::option::Option<crate::ctypes::_system!(unsafe fn(ort_value: *const OrtValue, out:
// *mut OrtSparseFormat) -> OrtStatusPtr)>, pub GetSparseTensorValuesTypeAndShape:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(ort_value: *const OrtValue, out: *mut *mut
// OrtTensorTypeAndShapeInfo) -> OrtStatusPtr)>, pub GetSparseTensorValues:
//     ::std::option::Option<crate::ctypes::_system!(unsafe fn(ort_value: *const OrtValue, out: *mut *const
// ::std::os::raw::c_void) -> OrtStatusPtr)>, pub GetSparseTensorIndicesTypeShape: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(ort_value: *const OrtValue, indices_format: OrtSparseIndicesFormat, out: *mut *mut
// OrtTensorTypeAndShapeInfo) -> OrtStatusPtr     )
// >,
// pub GetSparseTensorIndices: ::std::option::Option<
//     crate::ctypes::_system!(
//         unsafe fn(
//             ort_value: *const OrtValue,
//             indices_format: OrtSparseIndicesFormat,
//             num_indices: *mut size_t,
//             indices: *mut *const ::std::os::raw::c_void
//         ) -> OrtStatusPtr
//     )
// >,
// pub HasValue: ::std::option::Option<crate::ctypes::_system!(unsafe fn(value: *const OrtValue, out: *mut
// ::std::os::raw::c_int) -> OrtStatusPtr)>,

// OrtStatus * 	CreateTensorAsOrtValue (OrtAllocator *allocator, const int64_t *shape, size_t shape_len,
// ONNXTensorElementDataType type, OrtValue **out)  	Create a tensor.

// OrtStatus * 	CreateTensorWithDataAsOrtValue (const OrtMemoryInfo *info, void *p_data, size_t p_data_len, const int64_t
// *shape, size_t shape_len, ONNXTensorElementDataType type, OrtValue **out)  	Create a tensor backed by a user supplied
// buffer.

// OrtStatus * 	IsTensor (const OrtValue *value, int *out)
//  	Return if an OrtValue is a tensor type.

// OrtStatus * 	GetTensorMutableData (OrtValue *value, void **out)
//  	Get a pointer to the raw data inside a tensor.

// OrtStatus * 	FillStringTensor (OrtValue *value, const char *const *s, size_t s_len)
//  	Set all strings at once in a string tensor.

// OrtStatus * 	GetStringTensorDataLength (const OrtValue *value, size_t *len)
//  	Get total byte length for all strings in a string tensor.

// OrtStatus * 	GetStringTensorContent (const OrtValue *value, void *s, size_t s_len, size_t *offsets, size_t
// offsets_len)  	Get all strings from a string tensor.

// OrtStatus * 	GetTensorTypeAndShape (const OrtValue *value, OrtTensorTypeAndShapeInfo **out)
//  	Get type and shape information from a tensor OrtValue.

// OrtStatus * 	GetTypeInfo (const OrtValue *value, OrtTypeInfo **out)
//  	Get type information of an OrtValue.

// OrtStatus * 	GetValueType (const OrtValue *value, enum ONNXType *out)
//  	Get ONNXType of an OrtValue.

// OrtStatus * 	GetValue (const OrtValue *value, int index, OrtAllocator *allocator, OrtValue **out)
//  	Get non tensor data from an OrtValue.

// OrtStatus * 	GetValueCount (const OrtValue *value, size_t *out)
//  	Get non tensor value count from an OrtValue.

// OrtStatus * 	CreateValue (const OrtValue *const *in, size_t num_values, enum ONNXType value_type, OrtValue **out)
//  	Create a map or sequence OrtValue.

// OrtStatus * 	CreateOpaqueValue (const char *domain_name, const char *type_name, const void *data_container, size_t
// data_container_size, OrtValue **out)  	Create an opaque (custom user defined type) OrtValue.

// OrtStatus * 	GetOpaqueValue (const char *domain_name, const char *type_name, const OrtValue *in, void *data_container,
// size_t data_container_size)  	Get internal data from an opaque (custom user defined type) OrtValue.

// void 	ReleaseValue (OrtValue *input)

// OrtStatus * 	GetStringTensorElementLength (const OrtValue *value, size_t index, size_t *out)
//  	Get the length of a single string in a string tensor.

// OrtStatus * 	GetStringTensorElement (const OrtValue *value, size_t s_len, size_t index, void *s)
//  	Get a single string from a string tensor.

// OrtStatus * 	FillStringTensorElement (OrtValue *value, const char *s, size_t index)
//  	Set a single string in a string tensor.

// OrtStatus * 	TensorAt (OrtValue *value, const int64_t *location_values, size_t location_values_count, void **out)
//  	Direct memory access to a specified tensor element.

// OrtStatus * 	IsSparseTensor (const OrtValue *value, int *out)
//  	Sets *out to 1 iff an OrtValue is a SparseTensor, and 0 otherwise.

// OrtStatus * 	CreateSparseTensorAsOrtValue (OrtAllocator *allocator, const int64_t *dense_shape, size_t
// dense_shape_len, ONNXTensorElementDataType type, OrtValue **out)  	Create an OrtValue with a sparse tensor that is
// empty.

// OrtStatus * 	FillSparseTensorCoo (OrtValue *ort_value, const OrtMemoryInfo *data_mem_info, const int64_t
// *values_shape, size_t values_shape_len, const void *values, const int64_t *indices_data, size_t indices_num)

// OrtStatus * 	FillSparseTensorCsr (OrtValue *ort_value, const OrtMemoryInfo *data_mem_info, const int64_t
// *values_shape, size_t values_shape_len, const void *values, const int64_t *inner_indices_data, size_t
// inner_indices_num, const int64_t *outer_indices_data, size_t outer_indices_num)

// OrtStatus * 	FillSparseTensorBlockSparse (OrtValue *ort_value, const OrtMemoryInfo *data_mem_info, const int64_t
// *values_shape, size_t values_shape_len, const void *values, const int64_t *indices_shape_data, size_t
// indices_shape_len, const int32_t *indices_data)

// OrtStatus * 	CreateSparseTensorWithValuesAsOrtValue (const OrtMemoryInfo *info, void *p_data, const int64_t
// *dense_shape, size_t dense_shape_len, const int64_t *values_shape, size_t values_shape_len, ONNXTensorElementDataType
// type, OrtValue **out)

// OrtStatus * 	UseCooIndices (OrtValue *ort_value, int64_t *indices_data, size_t indices_num)

// OrtStatus * 	UseCsrIndices (OrtValue *ort_value, int64_t *inner_data, size_t inner_num, int64_t *outer_data, size_t
// outer_num)

// OrtStatus * 	UseBlockSparseIndices (OrtValue *ort_value, const int64_t *indices_shape, size_t indices_shape_len,
// int32_t *indices_data)

// OrtStatus * 	GetSparseTensorFormat (const OrtValue *ort_value, enum OrtSparseFormat *out)
//  	Returns sparse tensor format enum iff a given ort value contains an instance of sparse tensor.

// OrtStatus * 	GetSparseTensorValuesTypeAndShape (const OrtValue *ort_value, OrtTensorTypeAndShapeInfo **out)
//  	Returns data type and shape of sparse tensor values (nnz) iff OrtValue contains a SparseTensor.

// OrtStatus * 	GetSparseTensorValues (const OrtValue *ort_value, const void **out)
//  	Returns numeric data for sparse tensor values (nnz). For string values use GetStringTensor*().

// OrtStatus * 	GetSparseTensorIndicesTypeShape (const OrtValue *ort_value, enum OrtSparseIndicesFormat indices_format,
// OrtTensorTypeAndShapeInfo **out)  	Returns data type, shape for the type of indices specified by indices_format.

// OrtStatus * 	GetSparseTensorIndices (const OrtValue *ort_value, enum OrtSparseIndicesFormat indices_format, size_t
// *num_indices, const void **indices)  	Returns indices data for the type of the indices specified by indices_format.

// OrtStatus * 	GetTensorMemoryInfo (const OrtValue *value, const OrtMemoryInfo **mem_info)
//  	Returns a pointer to the OrtMemoryInfo of a Tensor.

#[repr(transparent)]
pub struct ValuePtr {
	pub(crate) ptr: *mut OrtValue
}

impl Drop for ValuePtr {
	fn drop(&mut self) {
		// Api::get().value_release(self)
	}
}
