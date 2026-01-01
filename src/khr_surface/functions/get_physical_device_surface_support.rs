use crate::{VkBool32, VkPhysicalDevice, VkResult, khr_surface::VkSurfaceKhr};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_FALSE, VK_TRUE, VkGetPhysicalDeviceQueueFamilyProperties, VkInstance, khr_surface};

/// Query if presentation is supported
///
/// # Parameters
///  - `physical_device` is the physical device.
///  - `queue_family_index` is the queue family.
///  - `surface` is the surface.
///  - `supported` is a pointer to a [`VkBool32`], which is set to [`VK_TRUE`] to indicate support,
///    and [`VK_FALSE`] otherwise.
///
/// # Valid Usage
///  - `queue_family_index` must be less than `queue_family_property_count` returned by
///    [`VkGetPhysicalDeviceQueueFamilyProperties`] for the given `physical_device`
///
/// # Valid Usage (Implicit)
///  - `physical_device` must be a valid [`VkPhysicalDevice`] handle
///  - `surface` must be a valid [`VkSurfaceKhr`] handle
///  - `supported` must be a valid pointer to a [`VkBool32`] value
///  - Both of `physical_device`, and `surface` must have been created, allocated, or retrieved
///    from the same [`VkInstance`]
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorSurfaceLostKhr`]
///
/// Provided by [`khr_surface`]
pub type VkGetPhysicalDeviceSurfaceSupportKhr = extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    surface: VkSurfaceKhr,
    supported: *mut VkBool32,
) -> VkResult;

/// The name of [`VkGetPhysicalDeviceSurfaceSupportKhr`]
pub const VK_GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR: &CStr =
    c"vkGetPhysicalDeviceSurfaceSupportKHR";
