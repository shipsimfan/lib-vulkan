use crate::{VkFormat, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkGraphicsPipelineCreateInfo, VkRenderPass, VkRenderingInfo};

/// Structure specifying attachment formats
///
/// # Description
/// When a pipeline is created without a [`VkRenderPass`], if the `next` chain of
/// [`VkGraphicsPipelineCreateInfo`] includes this structure, it specifies the view mask and format
/// of attachments used for rendering. If this structure is not specified, and the pipeline does
/// not include a [`VkRenderPass`], `view_mask` and `color_attachment_count` are 0, and
/// `depth_attachment_format` and `stencil_attachment_format` are [`VkFormat::Undefined`]. If a
/// graphics pipeline is created with a valid [`VkRenderPass`], parameters of this structure are
/// ignored.
///
/// If `depth_attachment_format`, `stencil_attachment_format`, or any element of
/// `color_attachment_formats` is [`VkFormat::Undefined`], it indicates that the corresponding
/// attachment is unused within the render pass. Valid formats indicate that an attachment can be
/// used - but it is still valid to set the attachment to [`null`] when beginning rendering.
///
/// If the render pass is going to be used with an external format resolve attachment, a
/// [`VkExternalFormatAndroid`] structure must also be included in the `next` chain of
/// [`VkGraphicsPipelineCreateInfo`], defining the external format of the resolve attachment that
/// will be used.
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPipelineRenderingCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PipelineRenderingCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `view_mask` is a bitfield of view indices describing which views are active during
    /// rendering. It must match [`VkRenderingInfo::view_mask`] when rendering.
    pub view_mask: u32,

    /// `color_attachment_count` is the number of entries in `color_attachment_formats`
    ///
    /// # Valid Usage
    ///  - `color_attachment_count` must be less than or equal to `max_color_attachments`
    pub color_attachment_count: u32,

    /// `color_attachment_formats` is a pointer to an array of [`VkFormat`] values defining the
    /// format of color attachments used in this pipeline.
    pub color_attachment_formats: *const VkFormat,

    /// `depth_attachment_format` is a [`VkFormat`] value defining the format of the depth
    /// attachment used in this pipeline.
    pub depth_attachment_format: VkFormat,

    /// `stencil_attachment_format` is a [`VkFormat`] value defining the format of the stencil
    /// attachment used in this pipeline.
    pub stencil_attachment_format: VkFormat,
}

impl const Default for VkPipelineRenderingCreateInfo {
    fn default() -> Self {
        VkPipelineRenderingCreateInfo {
            r#type: VkStructureType::PipelineRenderingCreateInfo,
            next: null(),
            view_mask: 0,
            color_attachment_count: 0,
            color_attachment_formats: null(),
            depth_attachment_format: VkFormat::Undefined,
            stencil_attachment_format: VkFormat::Undefined,
        }
    }
}

impl NextChain for VkPipelineRenderingCreateInfo {
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
