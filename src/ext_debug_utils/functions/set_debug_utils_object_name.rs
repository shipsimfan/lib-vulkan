use crate::{VkDevice, VkResult, ext_debug_utils::VkDebugUtilsObjectNameInfoExt};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VkInstance, VkObjectType, VkPhysicalDevice, ext_debug_utils};

/// Give a user-friendly name to an object
///
/// # Parameters
///  - `device` is the device that is associated with the named object passed in via
///    `object_handle`.
///  - `name_info` is a pointer to a [`VkDebugUtilsObjectNameInfoExt`] structure specifying
///    parameters of the name to set on the object.
///
/// # Valid Usage
///  - `name_info.object_type` must not be [`VkObjectType::Unknown`]
///  - `name_info.object_handle` must not be [`VK_NULL_HANDLE`]
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
///  - `name_info` must be a valid pointer to a valid [`VkDebugUtilsObjectNameInfoExt`] structure
///
/// # Host Synchronization
///  - Host access to `name_info.object_handle` must be externally synchronized
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
pub type VkSetDebugUtilsObjectNameExt = unsafe extern "system" fn(
    device: VkDevice,
    name_info: *const VkDebugUtilsObjectNameInfoExt,
) -> VkResult;

/// The name of [`VkSetDebugUtilsObjectNameExt`]
pub const VK_SET_DEBUG_UTILS_OBJECT_NAME_EXT: &CStr = c"vkSetDebugUtilsObjectNameEXT";
