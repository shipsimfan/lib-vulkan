use crate::{VkAccessFlags, VkDependencyFlags, VkPipelineStageFlags};

#[repr(C)]
pub struct VkSubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: VkPipelineStageFlags,
    pub dst_stage_mask: VkPipelineStageFlags,
    pub src_access_mask: VkAccessFlags,
    pub dst_access_mask: VkAccessFlags,
    pub dependency_flags: VkDependencyFlags,
}
