use crate::{VkDebugUtilsObjectNameInfoEXT, VkDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Give a user-friendly name to an object
///
/// # Parameters
///  - `device` is the device that is associated with the named object passed in via
///    `object_handle`.
///  - `name_info` is a pointer to a [`VkDebugUtilsObjectNameInfoEXT`] structure specifying
///    parameters of the name to set on the object.
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///
///  Provided by [`ext_debug_utils`]
pub type VkSetDebugUtilsObjectNameEXT = extern "system" fn(
    device: VkDevice,
    name_info: *const VkDebugUtilsObjectNameInfoEXT,
) -> VkResult;

/// The name of [`VkSetDebugUtilsObjectNameEXT`]
pub const VK_SET_DEBUG_UTILS_OBJECT_NAME_EXT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkSetDebugUtilsObjectNameEXT\0") };
