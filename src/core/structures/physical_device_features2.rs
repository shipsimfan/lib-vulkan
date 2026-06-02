use crate::{VkPhysicalDeviceFeatures, VkStructureType};
use std::{ffi::c_void, ptr::null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_1, VkDeviceCreateInfo};

/// Structure describing the fine-grained features that can be supported by an implementation
///
/// # Description
/// The `next` chain of this structure is used to extend the structure with features defined by
/// extensions. This structure can be used in [`VkGetPhysicalDeviceFeatures2`] or can be included
/// in the `next` chain of a [`VkDeviceCreateInfo`] structure, in which case it controls which
/// features are enabled in lieu of `enabled_features`.
///
/// Provided by [`VK_VERSION_1_1`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceFeatures2 {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PhysicalDeviceFeatures2`]
    pub r#type: VkStructureType,

    /// `next` is [`null_mut`] or a pointer to a structure extending this structure.
    pub next: *mut c_void,

    /// `features` is a [`VkPhysicalDeviceFeatures`] structure describing the fine-grained features
    /// of the Vulkan 1.0 API.
    pub features: VkPhysicalDeviceFeatures,
}

impl const Default for VkPhysicalDeviceFeatures2 {
    fn default() -> Self {
        VkPhysicalDeviceFeatures2 {
            r#type: VkStructureType::PhysicalDeviceFeatures2,
            next: null_mut(),
            features: VkPhysicalDeviceFeatures::default(),
        }
    }
}
