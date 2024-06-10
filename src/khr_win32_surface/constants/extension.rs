use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_win32_surface;

/// The name of the [`khr_win32_surface`] extension
///
/// Provided by [`khr_win32_surface`]
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_win32_surface\0") };

/// The version of the [`khr_win32_surface`] extension provided by these bindings
///
/// Provided by [`khr_win32_surface`]
pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
