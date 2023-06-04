use crate::{
    LogicOp, PipelineColorBlendAttachmentState, VkPipelineColorBlendAttachmentState,
    VkPipelineColorBlendStateCreateFlags, VkPipelineColorBlendStateCreateInfo, VkStructureType,
};
use std::ptr::null;

pub struct PipelineColorBlendStateCreateInfo {
    pub logic_op: Option<LogicOp>,
    pub attachments: Vec<PipelineColorBlendAttachmentState>,
    pub blend_constants: [f32; 4],
}

impl PipelineColorBlendStateCreateInfo {
    pub(super) fn into_binding(
        &self,
    ) -> (
        VkPipelineColorBlendStateCreateInfo,
        Vec<VkPipelineColorBlendAttachmentState>,
    ) {
        let attachments: Vec<_> = self
            .attachments
            .iter()
            .map(|attachment| attachment.into_binding())
            .collect();

        (
            VkPipelineColorBlendStateCreateInfo {
                s_type: VkStructureType::PipelineColorBlendStateCreateInfo,
                p_next: null(),
                flags: VkPipelineColorBlendStateCreateFlags::default(),
                logic_op_enable: self.logic_op.is_some() as u32,
                logic_op: self.logic_op.unwrap_or(LogicOp::Set),
                attachment_count: attachments.len() as u32,
                p_attachments: if attachments.len() == 0 {
                    null()
                } else {
                    attachments.as_ptr()
                },
                blend_constants: self.blend_constants,
            },
            attachments,
        )
    }
}
