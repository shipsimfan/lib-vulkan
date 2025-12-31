use crate::{VkPhysicalDevice, VkPhysicalDeviceMemoryProperties};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Reports memory information for the specified physical device
///
/// # Parameters
///  - `physical_device` is the handle to the device to query.
///  - `memory_properties` is a pointer to a [`VkPhysicalDeviceMemoryProperties`] structure in
///    which the properties are returned.
///
/// # Valid Usage (Implicit)
///  - `physical_device` must be a valid [`VkPhysicalDevice`] handle
///  - `memory_properties` must be a valid pointer to a [`VkPhysicalDeviceMemoryProperties`]
///    structure
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkGetPhysicalDeviceMemoryProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    memory_properties: *mut VkPhysicalDeviceMemoryProperties,
);

/// The name of [`VkGetPhysicalDeviceMemoryProperties`]
pub const VK_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES: &CStr = c"vkGetPhysicalDeviceMemoryProperties";
