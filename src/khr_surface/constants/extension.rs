use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_surface;

/// The name of the [`khr_surface`] extension
///
/// Provided by [`khr_surface`]
pub const VK_KHR_SURFACE_EXTENSION_NAME: &CStr = c"VK_KHR_surface";

/// The version of the [`khr_surface`] extension provided by these bindings
///
/// Provided by [`khr_surface`]
pub const VK_KHR_SURFACE_SPEC_VERSION: u32 = 25;
