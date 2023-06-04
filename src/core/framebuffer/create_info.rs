use crate::{
    bindings::{VkFramebufferCreateFlags, VkFramebufferCreateInfo, VkImageView, VkStructureType},
    ImageView, Loader, NativeLoader, RenderPass,
};
use std::{ptr::null, sync::Arc};

pub struct FramebufferCreateInfo<L: Loader = NativeLoader> {
    pub render_pass: Arc<RenderPass<L>>,
    pub attachments: Vec<ImageView<L>>,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

impl<L: Loader> FramebufferCreateInfo<L> {
    pub(super) fn into_binding(&self) -> (VkFramebufferCreateInfo, Vec<VkImageView>) {
        let attachments: Vec<_> = self
            .attachments
            .iter()
            .map(|attachment| attachment.handle())
            .collect();

        (
            VkFramebufferCreateInfo {
                s_type: VkStructureType::FramebufferCreateInfo,
                p_next: null(),
                flags: VkFramebufferCreateFlags::default(),
                render_pass: self.render_pass.handle(),
                attachment_count: attachments.len() as u32,
                p_attachments: if attachments.len() == 0 {
                    null()
                } else {
                    attachments.as_ptr()
                },
                width: self.width,
                height: self.height,
                layers: self.layers,
            },
            attachments,
        )
    }

    pub(super) fn consume(self) -> (Arc<RenderPass<L>>, Vec<ImageView<L>>) {
        (self.render_pass, self.attachments)
    }
}
