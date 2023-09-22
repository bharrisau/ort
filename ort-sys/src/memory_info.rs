use crate::{
	onnx_call,
	sys::{OrtArenaCfg, OrtMemoryInfo},
	Api
};

impl Api {
	// OrtStatus * 	CreateMemoryInfo (const char *name, enum OrtAllocatorType type, int id, enum OrtMemType mem_type,
	// OrtMemoryInfo **out)  	Create an OrtMemoryInfo.

	// OrtStatus * 	CreateCpuMemoryInfo (enum OrtAllocatorType type, enum OrtMemType mem_type, OrtMemoryInfo **out)
	//  	Create an OrtMemoryInfo for CPU memory.

	// OrtStatus * 	CompareMemoryInfo (const OrtMemoryInfo *info1, const OrtMemoryInfo *info2, int *out)
	//  	Compare OrtMemoryInfo objects for equality.

	// OrtStatus * 	MemoryInfoGetName (const OrtMemoryInfo *ptr, const char **out)
	//  	Get name from OrtMemoryInfo.

	// OrtStatus * 	MemoryInfoGetId (const OrtMemoryInfo *ptr, int *out)
	//  	Get the id from OrtMemoryInfo.

	// OrtStatus * 	MemoryInfoGetMemType (const OrtMemoryInfo *ptr, OrtMemType *out)
	//  	Get the OrtMemType from OrtMemoryInfo.

	// OrtStatus * 	MemoryInfoGetType (const OrtMemoryInfo *ptr, OrtAllocatorType *out)
	//  	Get the OrtAllocatorType from OrtMemoryInfo.

	// void 	ReleaseMemoryInfo (OrtMemoryInfo *input)

	// OrtStatus * 	CreateArenaCfg (size_t max_mem, int arena_extend_strategy, int initial_chunk_size_bytes, int
	// max_dead_bytes_per_chunk, OrtArenaCfg **out)

	// void 	ReleaseArenaCfg (OrtArenaCfg *input)

	// OrtStatus * 	CreateArenaCfgV2 (const char *const *arena_config_keys, const size_t *arena_config_values, size_t
	// num_keys, OrtArenaCfg **out)  	Create an OrtArenaCfg.
}

#[repr(transparent)]
pub struct MemoryInfo {
	pub(crate) ptr: *mut OrtMemoryInfo
}

#[repr(transparent)]
pub struct ArenaConfig {
	pub(crate) ptr: *mut OrtArenaCfg
}
