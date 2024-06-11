use crate::{
    VkDeviceCreateFlags, VkDeviceQueueCreateInfo, VkPhysicalDeviceFeatures, VkStructureType,
};
use std::{
    ffi::{c_char, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying parameters of a newly created device
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `flags` is reserved for future use.
    pub flags: VkDeviceCreateFlags,

    /// `queue_create_info_count` is the unsigned integer size of the `queue_create_infos` array.
    pub queue_create_info_count: u32,

    /// `queue_create_infos` is a pointer to an array of [`VkDeviceQueueCreateInfo`] structures
    /// describing the queues that are requested to be created along with the logical device.
    pub queue_create_infos: *const VkDeviceQueueCreateInfo,

    /// `enabled_layer_count` is deprecated and ignored.
    pub enabled_layer_count: u32,

    /// `enabled_layer_names` is deprecated and ignored.
    pub enabled_layer_names: *const *const c_char,

    /// `enabled_extension_count` is the number of device extensions to enable.
    pub enabled_extension_count: u32,

    /// `enabled_extension_names` is a pointer to an array of `enabled_extension_count`
    /// null-terminated UTF-8 strings containing the names of extensions to enable for the created
    /// device.
    pub enabled_extension_names: *const *const c_char,

    /// `enabled_features` is [`null`] or a pointer to a [`VkPhysicalDeviceFeatures`] structure
    /// containing boolean indicators of all the features to be enabled.
    pub enabled_features: *const VkPhysicalDeviceFeatures,
}

impl Default for VkDeviceCreateInfo {
    fn default() -> Self {
        VkDeviceCreateInfo {
            r#type: VkStructureType::DeviceCreateInfo,
            next: null(),
            flags: 0,
            queue_create_info_count: 0,
            queue_create_infos: null(),
            enabled_layer_count: 0,
            enabled_layer_names: null(),
            enabled_extension_count: 0,
            enabled_extension_names: null(),
            enabled_features: null(),
        }
    }
}
