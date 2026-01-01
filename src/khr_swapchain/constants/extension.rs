use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_swapchain;

/// The name of the [`khr_swapchain`] extension
///
/// Provided by [`khr_swapchain`]
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &CStr = c"VK_KHR_swapchain";

/// The version of the [`khr_swapchain`] extension provided by these bindings
///
/// Provided by [`khr_swapchain`]
pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
