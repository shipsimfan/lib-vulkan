use crate::{VkPhysicalDevice, VkPhysicalDeviceProperties};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Returns properties of a physical device
///
/// # Parameters
///  - `physical_device` is the handle to the physical device whose properties will be queried.
///  - `properties` is a pointer to a [`VkPhysicalDeviceProperties`] structure in which properties
///    are returned.
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkGetPhysicalDeviceProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    properties: *mut VkPhysicalDeviceProperties,
);

/// The name of [`VkGetPhysicalDeviceProperties`]
pub const VK_GET_PHYSICAL_DEVICE_PROPERTIES: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceProperties\0") };
