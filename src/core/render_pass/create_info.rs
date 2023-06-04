use crate::{
    bindings::VkSubpassDescription, AttachmentDescription, SubpassDependency, SubpassDescription,
    VkRenderPassCreateFlags, VkRenderPassCreateInfo, VkStructureType,
};
use std::ptr::null;

pub struct RenderPassCreateInfo {
    pub attachments: Vec<AttachmentDescription>,
    pub subpasses: Vec<SubpassDescription>,
    pub dependencies: Vec<SubpassDependency>,
}

impl RenderPassCreateInfo {
    pub(super) fn into_binding(&self) -> (VkRenderPassCreateInfo, Vec<VkSubpassDescription>) {
        let subpasses: Vec<_> = self
            .subpasses
            .iter()
            .map(|subpass| subpass.into_binding())
            .collect();

        (
            VkRenderPassCreateInfo {
                s_type: VkStructureType::RenderPassCreateInfo,
                p_next: null(),
                flags: VkRenderPassCreateFlags::default(),
                attachment_count: self.attachments.len() as u32,
                p_attachments: if self.attachments.len() == 0 {
                    null()
                } else {
                    self.attachments.as_ptr()
                },
                subpass_count: subpasses.len() as u32,
                p_subpasses: if subpasses.len() == 0 {
                    null()
                } else {
                    subpasses.as_ptr()
                },
                dependency_count: self.dependencies.len() as u32,
                p_dependencies: if self.dependencies.len() == 0 {
                    null()
                } else {
                    self.dependencies.as_ptr()
                },
            },
            subpasses,
        )
    }
}
