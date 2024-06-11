use crate::{VkDeviceQueueCreateFlags, VkStructureType};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VkGetPhysicalDeviceQueueFamilyProperties, VK_VERSION_1_0};

/// Structure specifying parameters of a newly created device queue
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceQueueCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `flags` is a bitmask indicating behavior of the queues.
    pub flags: VkDeviceQueueCreateFlags,

    /// `queue_family_index` is an unsigned integer indicating the index of the queue family in
    /// which to create the queues on this device. This index corresponds to the index of an
    /// element of the `queue_family_properties` array that was returned by
    /// [`VkGetPhysicalDeviceQueueFamilyProperties`].
    pub queue_family_index: u32,

    /// `queue_count` is an unsigned integer specifying the number of queues to create in the queue
    /// family indicated by `queue_family_index`, and with the behavior specified by `flags`.
    pub queue_count: u32,

    /// `queue_priorities` is a pointer to an array of `queue_count` normalized floating point
    /// values, specifying priorities of work that will be submitted to each created queue.
    pub queue_priorities: *const f32,
}

impl Default for VkDeviceQueueCreateInfo {
    fn default() -> Self {
        VkDeviceQueueCreateInfo {
            r#type: VkStructureType::DeviceQueueCreateInfo,
            next: null(),
            flags: 0,
            queue_family_index: 0,
            queue_count: 0,
            queue_priorities: null(),
        }
    }
}
