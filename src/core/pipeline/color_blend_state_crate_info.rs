use crate::{LogicOp, PipelineColorBlendAttachmentState};

pub struct PipelineColorBlendStateCreateInfo {
    pub logic_op: Option<LogicOp>,
    pub attachments: Vec<PipelineColorBlendAttachmentState>,
    pub blend_constants: [f32; 4],
}
