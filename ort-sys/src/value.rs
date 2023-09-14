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
