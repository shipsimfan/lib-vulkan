// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkExtensionProperties, VkLayerProperties, VkMemoryHeap, VkMemoryType,
    VkPhysicalDeviceMemoryProperties, VkPhysicalDeviceProperties,
};
#[allow(unused_imports)]
use std::ffi::c_char;

/// Maximum length of a layer of extension name string
///
/// [`VK_MAX_EXTENSION_NAME_SIZE`] is the length in [`c_char`] values of an array containing a
/// layer or extension name string, as returned in [`VkLayerProperties::layer_name`],
/// [`VkExtensionProperties::extension_name`], and other queries.
///
/// Provided by [`VK_VERSION_1_0`]
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;

/// Length of a driver name string
///
/// [`VK_MAX_DESCRIPTION_SIZE`] is the length in [`c_char`] values of an array containing a string
/// with additional descriptive information about a query, as returned in
/// [`VkLayerProperties::description`] and other queries.
///
/// Provided by [`VK_VERSION_1_0`]
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;

/// Length of an array of memory heaps
///
/// [`VK_MAX_MEMORY_HEAPS`] is the length of an array of [`VkMemoryHeap`] structures describing
/// memory heaps, as returned in [`VkPhysicalDeviceMemoryProperties::memory_heaps`].
///
/// Provided by [`VK_VERSION_1_0`]
pub const VK_MAX_MEMORY_HEAPS: usize = 16;

/// Length of an array of memory types
///
/// [`VK_MAX_MEMORY_TYPES`] is the length of an array of [`VkMemoryType`] structures describing
/// memory types, as returned in [`VkPhysicalDeviceMemoryProperties::memory_types`].
///
/// Provided by [`VK_VERSION_1_0`]
pub const VK_MAX_MEMORY_TYPES: usize = 32;

/// Length of a physical device name string
///
/// [`VK_MAX_PHYSICAL_DEVICE_NAME_SIZE`] is the length in [`c_char`] values of an array containing
/// a physical device name string, as returned in [`VkPhysicalDeviceProperties::device_name`].
///
/// Provided by [`VK_VERSION_1_0`]
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;

/// Length of a universally unique device or driver build identifier
///
/// [`VK_UUID_SIZE`] is the length in [`u8`] values of an array containing a universally unique
/// device or driver build identifier, as returned in [`VkPhysicalDeviceIDProperties::device_uuid`]
/// and [`VkPhysicalDeviceIDProperties::driver_uuid`].
///
/// Provided by [`VK_VERSION_1_0`]
pub const VK_UUID_SIZE: usize = 16;
