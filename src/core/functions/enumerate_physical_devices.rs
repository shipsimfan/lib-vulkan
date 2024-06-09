use crate::{VkInstance, VkPhysicalDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VkCreateInstance, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null;

/// Enumerates the physical devices accessible to a Vulkan instance
///
/// # Parameters
///  - `instance` is a handle to a Vulkan instance previously created with [`VkCreateInstance`].
///  - `physical_device_count` is a pointer to an integer related to the number of physical devices
///    available or queried, as described below.
///  - `physical_devices` is either [`null`] or a pointer to an array of [`VkPhysicalDevice`]
///    handl1es.
///
/// # Description
/// If `physical_devices` is [`null`], then the number of physical devices available is returned in
/// `physical_device_count`. Otherwise, `physical_device_count` must point to a variable set by the
/// user to the number of elements in the `physical_devices` array, and on return the variable is
/// overwritten with the number of handles actually written to `physical_devices`. If
/// `physical_device_count` is less than the number of physical devices available, at most
/// `physical_device_count` structures will be written, and [`VkResult::VkIncomplete`] will be
/// returned instead of [`VkResult::VkSuccess`], to indicate that not all the available physical
/// devices were returned.
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///  - [`VkResult::VkIncomplete`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorInitializationFailed`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkEnumeratePhysicalDevices = extern "system" fn(
    instance: VkInstance,
    physical_device_count: *mut u32,
    physical_devices: *mut VkPhysicalDevice,
) -> VkResult;

/// The name of [`VkEnumeratePhysicalDevices`]
pub const VK_ENUMERATE_PHYSICAL_DEVICES: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkEnumeratePhysicalDevices\0") };
