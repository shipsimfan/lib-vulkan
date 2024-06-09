use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// The name of the [`ext_debug_utils`] extension
///
/// Provided by [`ext_debug_utils`]
pub const VK_EXT_DEBUG_UTILS_EXTENSION_NAME: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_utils\0") };

/// The version of the [`ext_debug_utils`] extension provided by these bindings
///
/// Provided by [`ext_debug_utils`]
pub const VK_EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 2;
