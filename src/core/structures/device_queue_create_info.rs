use crate::{VkDeviceQueueCreateFlags, VkStructureType};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkDeviceQueueCreateFlag, VkGetPhysicalDeviceQueueFamilyProperties,
    VkQueueFamilyProperties, VkQueueFlag,
};

/// Structure specifying parameters of a newly created device queue
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceQueueCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::DeviceQueueCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `next` chain includes a [`VkDeviceQueueShaderCoreControlCreateInfoArm`] structure
    ///    then [`VkPhysicalDeviceSchedulingControlsPropertiesArm::scheduling_controls_flags`] must
    ///    contain [`VkPhysicalDeviceSchedulingControlsFlagArm::ShaderCoreCount`]
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkDeviceQueueGlobalPriorityCreateInfo`] or
    ///    [`VkDeviceQueueShaderCoreControlCreateInfoArm`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask indicating behavior of the queues.
    ///
    /// # Valid Usage
    ///  - If the `protected_memory` feature is not enabled, the
    ///    [`VkDeviceQueueCreateFlag::Protected`] bit of flags must not be set
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkDeviceQueueCreateFlag`] values
    pub flags: VkDeviceQueueCreateFlags,

    /// `queue_family_index` is an unsigned integer indicating the index of the queue family in
    /// which to create the queues on this device. This index corresponds to the index of an
    /// element of the `queue_family_properties` array that was returned by
    /// [`VkGetPhysicalDeviceQueueFamilyProperties`].
    ///
    /// # Valid Usage
    ///  - `queue_family_index` must be less than `queue_family_property_count` returned by
    ///    [`VkGetPhysicalDeviceQueueFamilyProperties`]
    ///  - If flags includes [`VkDeviceQueueCreateFlag::Protected`], `queue_family_index` must
    ///    be the index of a queue family that includes the [`VkQueueFlag::Protected`]
    ///    capability
    pub queue_family_index: u32,

    /// `queue_count` is an unsigned integer specifying the number of queues to create in the queue
    /// family indicated by `queue_family_index`, and with the behavior specified by `flags`.
    ///
    /// # Valid Usage
    ///  - `queue_count` must be less than or equal to the `queue_count` member of the
    ///    [`VkQueueFamilyProperties`] structure, as returned by
    ///    [`VkGetPhysicalDeviceQueueFamilyProperties`] in the
    ///    `queue_family_properties[queue_family_index]`
    ///
    /// # Valid Usage (Implicit)
    ///  - `queue_count` must be greater than 0
    pub queue_count: u32,

    /// `queue_priorities` is a pointer to an array of `queue_count` normalized floating point
    /// values, specifying priorities of work that will be submitted to each created queue.
    ///
    /// # Valid Usage
    ///  - Each element of `queue_priorities` must be between 0.0 and 1.0 inclusive
    ///
    /// # Valid Usage (Implicit)
    ///  - `queue_priorities` must be a valid pointer to an array of `queue_count` [`f32`] values
    pub queue_priorities: *const f32,
}

impl const Default for VkDeviceQueueCreateInfo {
    fn default() -> Self {
        VkDeviceQueueCreateInfo {
            r#type: VkStructureType::DeviceQueueCreateInfo,
            next: null(),
            flags: VkDeviceQueueCreateFlags::new(),
            queue_family_index: 0,
            queue_count: 0,
            queue_priorities: null(),
        }
    }
}
