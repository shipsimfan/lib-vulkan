use crate::{VkDynamicState, VkPipelineDynamicStateCreateFlags, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying parameters of a newly created pipeline dynamic state
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPipelineDynamicStateCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PipelineDynamicStateCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `flags` is reserved for future use.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be 0
    pub flags: VkPipelineDynamicStateCreateFlags,

    /// `dynamic_state_count` is the number of elements in the `dynamic_states` array.
    pub dynamic_state_count: u32,

    /// `dynamic_states` is a pointer to an array of [`VkDynamicState`] values specifying which
    /// pieces of pipeline state will use the values from dynamic state commands rather than from
    /// pipeline state creation information.
    ///
    /// # Valid Usage
    ///  - Each element of `dynamic_states` must be unique
    ///
    /// # Valid Usage (Implicit)
    ///  - If `dynamic_state_count` is not 0, `dynamic_states` must be a valid pointer to an array
    ///    of `dynamic_state_count` valid [`VkDynamicState`] values
    pub dynamic_states: *const VkDynamicState,
}

const impl Default for VkPipelineDynamicStateCreateInfo {
    fn default() -> Self {
        VkPipelineDynamicStateCreateInfo {
            r#type: VkStructureType::PipelineDynamicStateCreateInfo,
            next: null(),
            flags: VkPipelineDynamicStateCreateFlags::empty(),
            dynamic_state_count: 0,
            dynamic_states: null(),
        }
    }
}

impl NextChain for VkPipelineDynamicStateCreateInfo {
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
