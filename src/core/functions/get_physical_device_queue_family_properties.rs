use crate::{VkPhysicalDevice, VkQueueFamilyProperties};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::null;

/// Reports properties of the queues of the specified physical device
///
/// # Parameters
///  - `physical_device` is the handle to the physical device whose properties will be queried.
///  - `queue_family_property_count` is a pointer to an integer related to the number of queue
///    families available or queried, as described below.
///  - `queue_family_properties` is either [`null`] or a pointer to an array of
///    [`VkQueueFamilyProperties`] structures.
///
/// # Description
/// If `queue_family_properties` is [`null`], then the number of queue families available is
/// returned in `queue_family_property_count`. Implementations must support at least one queue
/// family. Otherwise, `queue_family_property_count` must point to a variable set by the user to
/// the number of elements in the `queue_family_properties` array, and on return the variable is
/// overwritten with the number of structures actually written to `queue_family_properties`. If
/// `queue_family_property_count` is less than the number of queue families available, at most
/// `queue_family_property_count` structures will be written.
///
/// # Valid Usage (Implicit)
///  - `physical_device` must be a valid [`VkPhysicalDevice`] handle
///  - `queue_family_property_count` must be a valid pointer to a [`u32`] value
///  - If the value referenced by `queue_family_property_count` is not 0, and
///    `queue_family_properties` is not [`null`], `queue_family_properties` must be a valid pointer
///    to an array of `queue_family_property_count` [`VkQueueFamilyProperties`] structures
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkGetPhysicalDeviceQueueFamilyProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_property_count: *mut u32,
    queue_family_properties: *mut VkQueueFamilyProperties,
);

/// The name of [`VkGetPhysicalDeviceQueueFamilyProperties`]
pub const VK_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES: &CStr =
    c"vkGetPhysicalDeviceQueueFamilyProperties";
