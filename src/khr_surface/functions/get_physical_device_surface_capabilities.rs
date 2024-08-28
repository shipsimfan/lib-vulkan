use crate::{VkPhysicalDevice, VkResult, VkSurfaceCapabilitiesKHR, VkSurfaceKHR};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_surface;

/// Query surface capabilities
///
/// # Parameters
///  - `physical_device` is the physical device that will be associated with the swapchain to be
///    created, as described for [`VkCreateSwapchainKHR`].
///  - `surface` is the surface that will be associated with the swapchain.
///  - `surface_capabilities` is a pointer to a [`VkSurfaceCapabilitiesKHR`] structure in which the
///    capabilities are returned.
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
pub type VkGetPhysicalDeviceSurfaceCapabilitiesKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    surface_capabilities: *mut VkSurfaceCapabilitiesKHR,
) -> VkResult;

/// The name of [`VkGetPhysicalDeviceSurfaceCapabilitiesKHR`]
pub const VK_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0") };
