use crate::{VkDebugUtilsObjectTagInfoEXT, VkDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Give a user-friendly name to an object
///
/// # Parameters
///  - `device` is the device that is associated with the named object passed in via
///    `object_handle`.
///  - `tag_info` is a pointer to a [`VkDebugUtilsObjectTagInfoEXT`] structure specifying
///    parameters of the tag to attach to the object.
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
pub type VkSetDebugUtilsObjectTagEXT =
    extern "system" fn(device: VkDevice, tag_info: *const VkDebugUtilsObjectTagInfoEXT) -> VkResult;

/// The name of [`VkSetDebugUtilsObjectTagEXT`]
pub const VK_SET_DEBUG_UTILS_OBJECT_TAG_EXT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkSetDebugUtilsObjectTagEXT\0") };
