use crate::{
    VkBool32, VkLogicOp, VkPipelineColorBlendAttachmentState, VkPipelineColorBlendStateCreateFlags,
    VkStructureType, util::NextChain,
};
use std::{
    ffi::{c_float, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_FALSE, VK_TRUE, VK_VERSION_1_0, VkDynamicState, VkPipelineColorBlendStateCreateFlag,
};

/// Structure specifying parameters of a newly created pipeline color blend state
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VkPipelineColorBlendStateCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PipelineColorBlendStateCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkPipelineColorBlendAdvancedStateCreateInfoExt`] or
    ///    [`VkPipelineColorWriteCreateInfoExt`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkPipelineColorBlendStateCreateFlag`]s specifying additional
    /// color blending information.
    ///
    /// # Valid Usage
    ///  - If the `rasterization_order_color_attachment_access` feature is not enabled, `flags`
    ///    must not include
    ///    [`VkPipelineColorBlendStateCreateFlag::RasterizationOrderAttachmnetAccessExt`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkPipelineColorBlendStateCreateFlag`] values
    pub flags: VkPipelineColorBlendStateCreateFlags,

    /// `logic_op_enable` controls whether to apply Logical Operations.
    ///
    /// # Valid Usage
    ///  - If the `logic_op` feature is not enabled, `logic_op_enable` must be [`VK_FALSE`]
    pub logic_op_enable: VkBool32,

    /// `logic_op` selects which logical operation to apply.
    ///
    /// # Valid Usage
    ///  - If `logic_op_enable` is [`VK_TRUE`], `logic_op` must be a valid [`VkLogicOp`] value
    pub logic_op: VkLogicOp,

    /// `attachment_count` is the number of [`VkPipelineColorBlendAttachmentState`] elements in
    /// `attachments`. It is ignored if the pipeline is created with
    /// [`VkDynamicState::ColorBlendEnableExt`], [`VkDynamicState::ColorBlendEquationExt`], and
    /// [`VkDynamicState::ColorWriteMaskExt`] dynamic states set, and either
    /// [`VkDynamicState::ColorBlendAdvancedExt`] set or the `advanced_blend_coherent_operation`
    /// feature is not enabled.
    pub attachment_count: u32,

    /// `attachments` is a pointer to an array of [`VkPipelineColorBlendAttachmentState`]
    /// structures defining blend state for each color attachment. It is ignored if the pipeline is
    /// created with [`VkDynamicState::ColorBlendEnableExt`],
    /// [`VkDynamicState::ColorBlendEquationExt`], and [`VkDynamicState::ColorWriteMaskExt`]
    /// dynamic states set, and either [`VkDynamicState::ColorBlendAdvancedExt`] set or the
    /// `advanced_blend_coherent_operation` feature is not enabled.
    ///
    /// # Valid Usage
    ///  - If the `independent_blend` feature is not enabled, all elements of `attachments` must be
    ///    identical
    ///  - If `attachment_count` is not 0 , and any of [`VkDynamicState::ColorBlendAdvancedExt`],
    ///    [`VkDynamicState::ColorBlendEnableExt`], [`VkDynamicState::ColorBlendEquationExt`], or
    ///    [`VkDynamicState::ColorWriteMaskExt`] are not set, `attachments` must be a valid pointer
    ///    to an array of `attachment_count` valid [`VkPipelineColorBlendAttachmentState`]
    ///    structures
    ///
    /// # Valid Usage (Implicit)
    ///  - If `attachment_count` is not 0, and `attachments` is not [`null`], `attachments` must be
    ///    a valid pointer to an array of `attachment_count` valid
    ///    [`VkPipelineColorBlendAttachmentState`] structures
    pub attachments: *const VkPipelineColorBlendAttachmentState,

    /// `blend_constants` is a pointer to an array of four values used as the R, G, B, and A
    /// components of the blend constant that are used in blending, depending on the blend factor.
    pub blend_constants: [c_float; 4],
}

impl const Default for VkPipelineColorBlendStateCreateInfo {
    fn default() -> Self {
        VkPipelineColorBlendStateCreateInfo {
            r#type: VkStructureType::PipelineColorBlendStateCreateInfo,
            next: null(),
            flags: VkPipelineColorBlendStateCreateFlags::empty(),
            logic_op_enable: 0,
            logic_op: VkLogicOp::Clear,
            attachment_count: 0,
            attachments: null(),
            blend_constants: [0.0; 4],
        }
    }
}

impl NextChain for VkPipelineColorBlendStateCreateInfo {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&self) -> *const c_void {
        self.next
    }

    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.next = next.map_or(null(), |n| n.as_ptr());
    }
}
