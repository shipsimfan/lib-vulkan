use crate::{
    VkBool32, VkLogicOp, VkPipelineColorBlendAttachmentState, VkPipelineColorBlendStateCreateFlags,
    VkStructureType,
};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkPipelineColorBlendStateCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkPipelineColorBlendStateCreateFlags,
    pub(crate) logic_op_enable: VkBool32,
    pub(crate) logic_op: VkLogicOp,
    pub(crate) attachment_count: u32,
    pub(crate) p_attachments: *const VkPipelineColorBlendAttachmentState,
    pub(crate) blend_constants: [f32; 4],
}
