use crate::{VkMemoryRequirements, VkStructureType, util::NextChainMut};
use std::{ffi::c_void, ptr::null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_1;

/// Structure specifying memory requirements
///
/// Provided by [`VK_VERSION_1_1`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryRequirements2 {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::MemoryRequirements2`]
    pub r#type: VkStructureType,

    /// `next` is [`null_mut`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null_mut`] or a pointer to a valid instance of
    ///    [`VkMemoryDedicatedRequirements`] or [`VkTileMemoryRequirementsQcom`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *mut c_void,

    /// `memory_requirements` is a [`VkMemoryRequirements`] structure describing the memory
    /// requirements of the resource.
    pub memory_requirements: VkMemoryRequirements,
}

const impl Default for VkMemoryRequirements2 {
    fn default() -> Self {
        VkMemoryRequirements2 {
            r#type: VkStructureType::MemoryRequirements2,
            next: null_mut(),
            memory_requirements: VkMemoryRequirements::default(),
        }
    }
}

impl NextChainMut for VkMemoryRequirements2 {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&mut self) -> *mut c_void {
        self.next
    }

    fn as_mut_ptr(&mut self) -> *mut c_void {
        (self as *mut Self).cast()
    }

    fn set_next(&mut self, next: Option<&mut dyn NextChainMut>) {
        self.next = next.map_or(null_mut(), |n| n.as_mut_ptr());
    }
}
