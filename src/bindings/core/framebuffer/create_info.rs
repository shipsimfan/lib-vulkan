use crate::{VkFramebufferCreateFlags, VkImageView, VkRenderPass, VkStructureType};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkFramebufferCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkFramebufferCreateFlags,
    pub(crate) render_pass: VkRenderPass,
    pub(crate) attachment_count: u32,
    pub(crate) p_attachments: *const VkImageView,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) layers: u32,
}
