use crate::{VkDevice, VkDeviceGroupPresentCapabilitiesKHR, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VK_VERSION_1_1};

/// Query present capabilities from other physical devices
///
/// # Parameters
///  - `device` is the logical device.
///  - `device_group_present_capabilities` is a pointer to a
///    [`VkDeviceGroupPresentCapabilitiesKHR`] structure in which the device’s capabilities are
///    returned.
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
pub type VkGetDeviceGroupPresentCapabilitiesKHR = extern "system" fn(
    device: VkDevice,
    device_group_present_capabilities: VkDeviceGroupPresentCapabilitiesKHR,
) -> VkResult;

/// The name of [`VkAcquireNextImage2KHR`]
pub const VK_GET_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceGroupPresentCapabilitiesKHR\0") };
