use crate::{BlendFactor, BlendOp, ColorComponentFlags, VkPipelineColorBlendAttachmentState};

pub struct PipelineColorBlendAttachmentState {
    pub blend_enable: bool,
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
    pub color_write_mask: ColorComponentFlags,
}

impl PipelineColorBlendAttachmentState {
    pub(super) fn into_binding(&self) -> VkPipelineColorBlendAttachmentState {
        VkPipelineColorBlendAttachmentState {
            blend_enable: self.blend_enable as u32,
            src_color_blend_factor: self.src_color_blend_factor,
            dst_color_blend_factor: self.dst_color_blend_factor,
            color_blend_op: self.color_blend_op,
            src_alpha_blend_factor: self.src_alpha_blend_factor,
            dst_alpha_blend_factor: self.dst_alpha_blend_factor,
            alpha_blend_op: self.alpha_blend_op,
            color_write_mask: self.color_write_mask,
        }
    }
}

impl Default for PipelineColorBlendAttachmentState {
    fn default() -> Self {
        PipelineColorBlendAttachmentState {
            blend_enable: true,
            src_color_blend_factor: BlendFactor::SrcAlpha,
            dst_color_blend_factor: BlendFactor::OneMinusSrcAlpha,
            color_blend_op: BlendOp::Add,
            src_alpha_blend_factor: BlendFactor::One,
            dst_alpha_blend_factor: BlendFactor::Zero,
            alpha_blend_op: BlendOp::Add,
            color_write_mask: ColorComponentFlags::default(),
        }
    }
}
