use crate::{VkPipelineTessellationStateCreateFlags, VkStructureType};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkPipelineTessellationStateCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkPipelineTessellationStateCreateFlags,
    pub(crate) patch_control_points: u32,
}
