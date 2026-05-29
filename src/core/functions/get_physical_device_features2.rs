use crate::{VkPhysicalDevice, VkPhysicalDeviceFeatures2};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_FALSE, VK_TRUE, VK_VERSION_1_1, VkBool32};

/// Reports capabilities of a physical device
///
/// # Parameters
///  * `physical_device` is the physical device from which to query the supported features.
///  * `features` is a pointer to a [`VkPhysicalDeviceFeatures2`] structure in which the physical
///    device features are returned.
///
/// # Description
/// Each structure in `features` and its `next` chain contains members corresponding to
/// fine-grained features. Each structure in `features` and its `next` chain contains [`VkBool32`]
/// members corresponding to fine-grained features. Each such member is returned with a [`VK_TRUE`]
/// value indicating that feature is supported on this physical device, or a [`VK_FALSE`] value
/// indicating it is unsupported.
///
/// # Valid Usage (Implicit)
///  - `physical_device` must be a valid [`VkPhysicalDevice`] handle
///  - `features` must be a valid pointer to a [`VkPhysicalDeviceFeatures2`] structure
///
/// Provided by [`VK_VERSION_1_1`]
pub type VkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    features: *mut VkPhysicalDeviceFeatures2,
);

/// The name of [`VkGetPhysicalDeviceFeatures2`]
pub const VK_GET_PHYSICAL_DEVICE_FEATURES_2: &CStr = c"vkGetPhysicalDeviceFeatures2";
