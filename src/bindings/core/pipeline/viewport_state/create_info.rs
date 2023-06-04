use crate::{VkPipelineViewportStateCreateFlags, VkRect2D, VkStructureType, VkViewport};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkPipelineViewportStateCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkPipelineViewportStateCreateFlags,
    pub(crate) viewport_count: u32,
    pub(crate) p_viewports: *const VkViewport,
    pub(crate) scissor_count: u32,
    pub(crate) p_scissors: *const VkRect2D,
}
