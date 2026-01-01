use crate::{VkDevice, VkResult, khr_swapchain::VkDeviceGroupPresentCapabilitiesKhr};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_1, khr_swapchain};

/// Query present capabilities from other physical devices
///
/// # Parameters
///  - `device` is the logical device.
///  - `device_group_present_capabilities` is a pointer to a
///    [`VkDeviceGroupPresentCapabilitiesKhr`] structure in which the device’s capabilities are
///    returned.
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `device_group_present_capabilities` must be a valid pointer to a
///    [`VkDeviceGroupPresentCapabilitiesKhr`] structure
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
pub type VkGetDeviceGroupPresentCapabilitiesKhr = extern "system" fn(
    device: VkDevice,
    device_group_present_capabilities: VkDeviceGroupPresentCapabilitiesKhr,
) -> VkResult;

/// The name of [`VkGetDeviceGroupPresentCapabilitiesKhr`]
pub const VK_GET_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: &CStr =
    c"vkGetDeviceGroupPresentCapabilitiesKHR";
