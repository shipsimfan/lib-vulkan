use crate::{
    VkPipelineTessellationStateCreateFlags, VkPipelineTessellationStateCreateInfo, VkStructureType,
};
use std::ptr::null;

pub struct PipelineTessellationStateCreateInfo {
    pub patch_control_points: u32,
}

impl PipelineTessellationStateCreateInfo {
    pub(super) fn into_binding(&self) -> VkPipelineTessellationStateCreateInfo {
        VkPipelineTessellationStateCreateInfo {
            s_type: VkStructureType::PipelineTessellationStateCreateInfo,
            p_next: null(),
            flags: VkPipelineTessellationStateCreateFlags::default(),
            patch_control_points: self.patch_control_points,
        }
    }
}
