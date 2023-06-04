use crate::{
    VkAttachmentDescription, VkRenderPassCreateFlags, VkStructureType, VkSubpassDependency,
    VkSubpassDescription,
};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkRenderPassCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkRenderPassCreateFlags,
    pub(crate) attachment_count: u32,
    pub(crate) p_attachments: *const VkAttachmentDescription,
    pub(crate) subpass_count: u32,
    pub(crate) p_subpasses: *const VkSubpassDescription,
    pub(crate) dependency_count: u32,
    pub(crate) p_dependencies: *const VkSubpassDependency,
}
