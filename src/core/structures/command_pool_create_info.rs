use crate::{VkCommandPoolCreateFlags, VkStructureType};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_0, VkCommandPoolCreateFlag, VkQueueFlag};

/// Structure specifying parameters of a newly created command pool
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkCommandPoolCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::CommandPoolCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `next` chain includes a [`VkDataGraphProcessingEngineCreateInfoArm`] structure,
    ///    each member of `processing_engines` must be identical to
    ///    [`VkQueueFamilyDataGraphPropertiesArm::engine`] retrieved from
    ///    [`VkGetPhysicalDeviceQueueFamilyDataGraphPropertiesArm`] with `queue_family_index` and
    ///    the `physical_device` that was used to create `device`
    ///  - If `queue_family_index` designates a queue family that supports
    ///    [`VkQueueFlag::DataGraphBitArm`] and enumerates a foreign engine through
    ///    [`VkGetPhysicalDeviceQueueFamilyDataGraphPropertiesArm`] with type
    ///    [`VkPhysicalDeviceDataGraphProcessingEngineTypeArm::NerualQcom`] or
    ///    [`VkPhysicalDeviceDataGraphProcessingEngineTypeArm::ComputeQcom`], the `next` chain must
    ///    include [`VkDataGraphProcessingEngineCreateInfoArm`] with
    ///    [`VkPhysicalDeviceDataGraphProcessingEngineArm::is_foreign`] set to [`VK_TRUE`] for all
    ///    elements of `processing_engines`
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`] or a pointer to a valid instance of
    ///    [`VkDataGraphProcessingEngineCreateInfoArm`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkCommandPoolCreateFlag`]s indicating usage behavior for the
    /// pool and command buffers allocated from it.
    ///
    /// # Valid Usage
    ///  - If the `protected_memory` feature is not enabled, the
    ///    [`VkCommandPoolCreateFlag::ProtectedBit`] bit of `flags` must not be set
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkCommandPoolCreateFlag`]s values
    pub flags: VkCommandPoolCreateFlags,

    /// `queue_family_index` designates a queue family. All command buffers allocated from this
    /// command pool must be submitted on queues from the same queue family.
    ///
    /// # Valid Usage
    ///  - If the `next` chain includes a [`VkDataGraphProcessingEngineCreateInfoArm`] structure,
    ///    then `queue_family_index` must designate a queue family that supports
    ///    [`VkQueueFlag::DataGraphBitArm`]
    pub queue_family_index: u32,
}

impl Default for VkCommandPoolCreateInfo {
    fn default() -> Self {
        VkCommandPoolCreateInfo {
            r#type: VkStructureType::CommandPoolCreateInfo,
            next: null(),
            flags: VkCommandPoolCreateFlags::default(),
            queue_family_index: 0,
        }
    }
}
