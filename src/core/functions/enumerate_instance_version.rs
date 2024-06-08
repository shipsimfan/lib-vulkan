use crate::VkResult;
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_1;

/// Query instance-level version before instance creation
///
/// # Parameters
///  - `api_version` is a pointer to a [`u32`], which is the version of Vulkan supported by
///    instance-level functionality. This must be a valid pointer to a [`u32`] value.
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///
/// Provided by [`VK_VERSION_1_1`]
pub type VkEnumerateInstanceVersion = extern "system" fn(api_version: *mut u32) -> VkResult;

/// The name of [`VkEnumerateInstanceVersion`]
pub const VK_ENUMERATE_INSTANCE_VERSION: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkEnumerateInstanceVersion\0") };
