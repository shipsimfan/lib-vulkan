use crate::{VkPipelineTessellationStateCreateFlags, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkPhysicalDeviceLimits};

/// Structure specifying parameters of a newly created pipeline tessellation state
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPipelineTessellationStateCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PipelineTessellationStateCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`] or a pointer to a valid instance of
    ///    [`VkPipelineTessellationDomainOriginStateCreateInfo`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is reserved for future use.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be 0
    pub flags: VkPipelineTessellationStateCreateFlags,

    /// `patch_control_points` is the number of control points per patch.
    ///
    /// # Valid Usage
    ///  - `patch_control_points` must be greater than zero and less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_tessellation_patch_size`]
    pub patch_control_points: u32,
}

const impl Default for VkPipelineTessellationStateCreateInfo {
    fn default() -> Self {
        VkPipelineTessellationStateCreateInfo {
            r#type: VkStructureType::PipelineTessellationStateCreateInfo,
            next: null(),
            flags: VkPipelineTessellationStateCreateFlags::empty(),
            patch_control_points: 0,
        }
    }
}

impl NextChain for VkPipelineTessellationStateCreateInfo {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&self) -> *const c_void {
        self.next
    }

    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.next = next.map_or(null(), |n| n.as_ptr());
    }
}
