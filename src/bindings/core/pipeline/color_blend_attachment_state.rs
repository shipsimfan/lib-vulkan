use crate::{VkBlendFactor, VkBlendOp, VkBool32, VkColorComponentFlags};

#[repr(C)]
pub(crate) struct VkPipelineColorBlendAttachmentState {
    pub(crate) blend_enable: VkBool32,
    pub(crate) src_color_blend_factor: VkBlendFactor,
    pub(crate) dst_color_blend_factor: VkBlendFactor,
    pub(crate) color_blend_op: VkBlendOp,
    pub(crate) src_alpha_blend_factor: VkBlendFactor,
    pub(crate) dst_alpha_blend_factor: VkBlendFactor,
    pub(crate) alpha_blend_op: VkBlendOp,
    pub(crate) color_write_mask: VkColorComponentFlags,
}
