use crate::{VkAllocationCallbacks, VkDevice, VkDeviceCreateInfo, VkPhysicalDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::null;

/// Create a new device instance
///
/// # Parameters
///  - `physical_device` must be one of the device handles returned from a call to
///    [`VkEnumeratePhysicalDevices`].
///  - `create_info` is a pointer to a [`VkDeviceCreateInfo`] structure containing information
///    about how to create the device.
///  - `allocator` controls host memory allocation.
///  - `device` is a pointer to a handle in which the created [`VkDevice`] is returned.
///
/// # Description
/// [`VkCreateDevice`] verifies that extensions and features requested in the
/// `enabled_extension_names` and `enabled_features` members of `create_info`, respectively, are
/// supported by the implementation. If any requested extension is not supported,
/// [`VkCreateDevice`] must return [`VkResult::VkErrorExtensionNotPresent`]. If any requested
/// feature is not supported, [`VkCreateDevice`] must return
/// [`VkResult::VkErrorFeatureNotPresent`]. Support for extensions can be checked before creating a
/// device by querying [`VkEnumerateDeviceExtensionProperties`]. Support for features can similarly
/// be checked by querying [`VkGetPhysicalDeviceFeatures`].
///
/// After verifying and enabling the extensions the [`VkDevice`] object is created and returned to
/// the application.
///
/// Multiple logical devices can be created from the same physical device. Logical device creation
/// may fail due to lack of device-specific resources (in addition to other errors). If that
/// occurs, [`VkCreateDevice`] will return [`VkResult::VkErrorTooManyObjects`].
///
/// # Valid Usage
///  - All required device extensions for each extension in the
///    [`VkDeviceCreateInfo::enabled_extension_names`] list must also be present in that list
///
/// # Valid Usage (Implicit)
///  - `physical_device` must be a valid [`VkPhysicalDevice`] handle
///  - `create_info` must be a valid pointer to a valid [`VkDeviceCreateInfo`] structure
///  - if `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `device` must be a valid pointer to a [`VkDevice`] handle
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorInitializationFailed`]
///  - [`VkResult::VkErrorExtensionNotPresent`]
///  - [`VkResult::VkErrorFeatureNotPresent`]
///  - [`VkResult::VkErrorTooManyObjects`]
///  - [`VkResult::VkErrorDeviceLost`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCreateDevice = extern "system" fn(
    physical_device: VkPhysicalDevice,
    create_info: *const VkDeviceCreateInfo,
    allocator: *const VkAllocationCallbacks,
    device: *mut VkDevice,
) -> VkResult;

/// The name of [`VkCreateDevice`]
pub const VK_CREATE_DEVICE: &CStr = c"vkCreateDevice";
