#!/bin/sh

# Call with path to onnxruntime_c_api.h

bindgen $1 -o sys.rs --allowlist-type "Ort.+" --allowlist-function "Ort.+" --allowlist-var "Ort.+|ORT.+"  --rustified-enum "*" --no-size_t-is-usize --rust-target 1.59 --ctypes-prefix "crate::ctypes"
sed -i 's/pub type size_t = .*/use crate::ctypes::size_t;/' sys.rs
sed -Ez -i 's/<([ \n\r\t]*)(unsafe )?extern "C" ([^)]*\)( ->[^,>]*)?),?([\n\r\t]*)>/<\1crate::ctypes::_system!(\2\3)\5>/g' sys.rs
sed -i 's/extern "C" {/crate::ctypes::_system_block!{/' sys.rs