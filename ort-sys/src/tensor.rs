use crate::{onnx_call, Api, Result};

impl Api {}

// OrtStatus * 	CastTypeInfoToTensorInfo (const OrtTypeInfo *type_info, const OrtTensorTypeAndShapeInfo **out)
//  	Get OrtTensorTypeAndShapeInfo from an OrtTypeInfo.

// OrtStatus * 	GetOnnxTypeFromTypeInfo (const OrtTypeInfo *type_info, enum ONNXType *out)
//  	Get ONNXType from OrtTypeInfo.

// void 	ReleaseTypeInfo (OrtTypeInfo *input)

// OrtStatus * 	GetDenotationFromTypeInfo (const OrtTypeInfo *type_info, const char **const denotation, size_t *len)
//  	Get denotation from type information.

// OrtStatus * 	CastTypeInfoToMapTypeInfo (const OrtTypeInfo *type_info, const OrtMapTypeInfo **out)
//  	Get detailed map information from an OrtTypeInfo.

// OrtStatus * 	CastTypeInfoToSequenceTypeInfo (const OrtTypeInfo *type_info, const OrtSequenceTypeInfo **out)
//  	Cast OrtTypeInfo to an OrtSequenceTypeInfo.

// OrtMapTypeInfo
// OrtStatus * 	GetMapKeyType (const OrtMapTypeInfo *map_type_info, enum ONNXTensorElementDataType *out)
//  	Get key type from an OrtMapTypeInfo.

// OrtStatus * 	GetMapValueType (const OrtMapTypeInfo *map_type_info, OrtTypeInfo **type_info)
//  	Get the value type from an OrtMapTypeInfo.

// void 	ReleaseMapTypeInfo (OrtMapTypeInfo *input)

// OrtSequenceTypeInfo
// OrtStatus * 	GetSequenceElementType (const OrtSequenceTypeInfo *sequence_type_info, OrtTypeInfo **type_info)
//  	Get element type from an OrtSequenceTypeInfo.

// void 	ReleaseSequenceTypeInfo (OrtSequenceTypeInfo *input)

// OrtStatus * 	CreateTensorTypeAndShapeInfo (OrtTensorTypeAndShapeInfo **out)
//  	Create an OrtTensorTypeAndShapeInfo object.

// OrtStatus * 	SetTensorElementType (OrtTensorTypeAndShapeInfo *info, enum ONNXTensorElementDataType type)
//  	Set element type in OrtTensorTypeAndShapeInfo.

// OrtStatus * 	SetDimensions (OrtTensorTypeAndShapeInfo *info, const int64_t *dim_values, size_t dim_count)
//  	Set shape information in OrtTensorTypeAndShapeInfo.

// OrtStatus * 	GetTensorElementType (const OrtTensorTypeAndShapeInfo *info, enum ONNXTensorElementDataType *out)
//  	Get element type in OrtTensorTypeAndShapeInfo.

// OrtStatus * 	GetDimensionsCount (const OrtTensorTypeAndShapeInfo *info, size_t *out)
//  	Get dimension count in OrtTensorTypeAndShapeInfo.

// OrtStatus * 	GetDimensions (const OrtTensorTypeAndShapeInfo *info, int64_t *dim_values, size_t dim_values_length)
//  	Get dimensions in OrtTensorTypeAndShapeInfo.

// OrtStatus * 	GetSymbolicDimensions (const OrtTensorTypeAndShapeInfo *info, const char *dim_params[], size_t
// dim_params_length)  	Get symbolic dimension names in OrtTensorTypeAndShapeInfo.

// OrtStatus * 	GetTensorShapeElementCount (const OrtTensorTypeAndShapeInfo *info, size_t *out)
//  	Get total number of elements in a tensor shape from an OrtTensorTypeAndShapeInfo.

// void 	ReleaseTensorTypeAndShapeInfo (OrtTensorTypeAndShapeInfo *input)
