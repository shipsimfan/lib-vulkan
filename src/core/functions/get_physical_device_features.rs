use crate::{VkPhysicalDevice, VkPhysicalDeviceFeatures};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_FALSE, VK_TRUE, VK_VERSION_1_0};

/// Reports capabilities of a physical device
///
/// # Parameters
///  - `physical_device` is the physical device from which to query the supported features.
///  - `features` is a pointer to a [`VkPhysicalDeviceFeatures`] structure in which the physical
///    device features are returned. For each feature, a value of [`VK_TRUE`] specifies that the
///    feature is supported on this physical device, and [`VK_FALSE`] specifies that the feature is
///    not supported.
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkGetPhysicalDeviceFeatures =
    extern "system" fn(physical_device: VkPhysicalDevice, features: *mut VkPhysicalDeviceFeatures);

/// The name of [`VkGetPhysicalDeviceFeatures`]
pub const VK_GET_PHYSICAL_DEVICE_FEATURES: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceFeatures\0") };
