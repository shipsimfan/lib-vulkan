use std::{ffi::c_void, ptr::null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Reserved non-valid object handle
///
/// [`VK_NULL_HANDLE`] is a reserved value representing a non-valid object handle. It may be passed
/// to and returned from Vulkan commands only when specifically allowed.
///
/// Provided by [`VK_VERSION_1_0`]
pub const VK_NULL_HANDLE: *mut c_void = null_mut();
