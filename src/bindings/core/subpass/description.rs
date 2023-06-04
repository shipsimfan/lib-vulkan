use crate::{VkAttachmentReference, VkPipelineBindPoint, VkSubpassDescriptionFlags};

#[repr(C)]
pub(crate) struct VkSubpassDescription {
    pub(crate) flags: VkSubpassDescriptionFlags,
    pub(crate) pipeline_bind_points: VkPipelineBindPoint,
    pub(crate) input_attachment_count: u32,
    pub(crate) p_input_attachments: *const VkAttachmentReference,
    pub(crate) color_attachment_count: u32,
    pub(crate) p_color_attachments: *const VkAttachmentReference,
    pub(crate) p_resolve_attachments: *const VkAttachmentReference,
    pub(crate) p_depth_stencil_attachment: *const VkAttachmentReference,
    pub(crate) preserve_attachment_count: u32,
    pub(crate) p_preserve_attachments: *const u32,
}
