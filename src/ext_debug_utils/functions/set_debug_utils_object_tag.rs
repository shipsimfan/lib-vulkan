use crate::{VkDevice, VkResult, ext_debug_utils::VkDebugUtilsObjectTagInfoExt};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VkInstance, VkPhysicalDevice, ext_debug_utils};

/// Give a user-friendly name to an object
///
/// # Parameters
///  - `device` is the device that is associated with the named object passed in via
///    `object_handle`.
///  - `tag_info` is a pointer to a [`VkDebugUtilsObjectTagInfoExt`] structure specifying
///    parameters of the tag to attach to the object.
///
/// # Valid Usage
///  - If `name_info.object_handle` is the valid handle of an instance-level object, the
///    [`VkDevice`] identified by device must be a descendent of the same [`VkInstance`] as the
///    object identified by `name_info.object_handle`
///  - If `name_info.object_handle` is the valid handle of a physical-device-level object, the
///    [`VkDevice`] identified by device must be a descendant of the same [`VkPhysicalDevice`] as
///    the object identified by `name_info.object_handle`
///  - If `name_info.object_handle` is the valid handle of a device-level object, that object must
///    be a descendent of the [`VkDevice`] identified by device
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `tag_info` must be a valid pointer to a valid [`VkDebugUtilsObjectTagInfoExt`] structure
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///
/// Provided by [`ext_debug_utils`]
pub type VkSetDebugUtilsObjectTagExt =
    unsafe extern "system" fn(device: VkDevice, tag_info: *const VkDebugUtilsObjectTagInfoExt) -> VkResult;

/// The name of [`VkSetDebugUtilsObjectTagExt`]
pub const VK_SET_DEBUG_UTILS_OBJECT_TAG_EXT: &CStr = c"vkSetDebugUtilsObjectTagEXT";
