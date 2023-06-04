use crate::{
    bindings::{VkSubpassDescription, VkSubpassDescriptionFlags},
    AttachmentReference, PipelineBindPoint,
};
use std::ptr::null;

pub struct SubpassDescription {
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachments: Vec<AttachmentReference>,
    pub color_attachments: Vec<AttachmentReference>,
    pub resolve_attachments: Vec<AttachmentReference>,
    pub depth_stencil_attachment: Option<AttachmentReference>,
    pub preserve_attachments: Vec<u32>,
}

impl SubpassDescription {
    pub(super) fn into_binding(&self) -> VkSubpassDescription {
        if self.resolve_attachments.len() != 0 {
            assert_eq!(self.color_attachments.len(), self.resolve_attachments.len());
        }

        VkSubpassDescription {
            flags: VkSubpassDescriptionFlags::default(),
            pipeline_bind_points: self.pipeline_bind_point,
            input_attachment_count: self.input_attachments.len() as u32,
            p_input_attachments: if self.input_attachments.len() == 0 {
                null()
            } else {
                self.input_attachments.as_ptr()
            },
            color_attachment_count: self.color_attachments.len() as u32,
            p_color_attachments: if self.color_attachments.len() == 0 {
                null()
            } else {
                self.color_attachments.as_ptr()
            },
            p_resolve_attachments: if self.resolve_attachments.len() == 0 {
                null()
            } else {
                self.resolve_attachments.as_ptr()
            },
            p_depth_stencil_attachment: self
                .depth_stencil_attachment
                .as_ref()
                .map(|depth_stencil_attachment| depth_stencil_attachment as *const _)
                .unwrap_or(null()),
            preserve_attachment_count: self.preserve_attachments.len() as u32,
            p_preserve_attachments: if self.preserve_attachments.len() == 0 {
                null()
            } else {
                self.preserve_attachments.as_ptr()
            },
        }
    }
}
