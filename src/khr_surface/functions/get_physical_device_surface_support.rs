use crate::{VkBool32, VkPhysicalDevice, VkResult, VkSurfaceKHR};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_surface, VK_FALSE, VK_TRUE};

/// Query if presentation is supported
///
/// # Parameters
///  - `physical_device` is the physical device.
///  - `queue_family_index` is the queue family.
///  - `surface` is the surface.
///  - `supported` is a pointer to a [`VkBool32`], which is set to [`VK_TRUE`] to indicate support,
///    and [`VK_FALSE`] otherwise.
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorSurfaceLostKHR`]
///
/// Provided by [`khr_surface`]
pub type VkGetPhysicalDeviceSurfaceSupportKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    surface: VkSurfaceKHR,
    supported: *mut VkBool32,
) -> VkResult;

/// The name of [`VkGetPhysicalDeviceSurfaceSupportKHR`]
pub const VK_GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceSurfaceSupportKHR\0") };
