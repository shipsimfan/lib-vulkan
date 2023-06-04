use crate::{VkDynamicState, VkPipelineDynamicStateCreateFlags, VkStructureType};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkPipelineDynamicStateCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkPipelineDynamicStateCreateFlags,
    pub(crate) dynamic_state_count: u32,
    pub(crate) p_dynamic_states: *const VkDynamicState,
}
