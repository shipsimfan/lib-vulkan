use crate::{
    VkPhysicalDevice, VkResult,
    khr_surface::{VkSurfaceCapabilitiesKhr, VkSurfaceKhr},
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VkInstance,
    khr_surface::{self, VkGetPhysicalDeviceSurfaceSupportKhr},
};

/// Query surface capabilities
///
/// # Parameters
///  - `physical_device` is the physical device that will be associated with the swapchain to be
///    created, as described for [`VkCreateSwapchainKhr`].
///  - `surface` is the surface that will be associated with the swapchain.
///  - `surface_capabilities` is a pointer to a [`VkSurfaceCapabilitiesKhr`] structure in which the
///    capabilities are returned.
///
/// # Valid Usage
///  - `surface` must be supported by `physical_device`, as reported by
///    [`VkGetPhysicalDeviceSurfaceSupportKhr`] or an equivalent platform-specific mechanism
///
/// # Valid Usage (Implicit)
///  - `physical_device` must be a valid [`VkPhysicalDevice`] handle
///  - `surface` must be a valid [`VkSurfaceKhr`] handle
///  - `surface_capabilities` must be a valid pointer to a [`VkSurfaceCapabilitiesKhr`] structure
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
pub type VkGetPhysicalDeviceSurfaceCapabilitiesKhr = unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKhr,
    surface_capabilities: *mut VkSurfaceCapabilitiesKhr,
) -> VkResult;

/// The name of [`VkGetPhysicalDeviceSurfaceCapabilitiesKhr`]
pub const VK_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES_KHR: &CStr =
    c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR";
