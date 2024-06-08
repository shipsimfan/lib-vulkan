use std::{ffi::c_void, ptr::null_mut};

/// Reserved non-valid object handle
pub const VK_NULL_HANDLE: *mut c_void = null_mut();
