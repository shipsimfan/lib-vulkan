use crate::{
    VkAttachmentDescriptionFlags, VkAttachmentLoadOp, VkAttachmentStoreOp, VkFormat, VkImageLayout,
    VkSampleCountFlagBits,
};

#[repr(C)]
pub struct VkAttachmentDescription {
    pub flags: VkAttachmentDescriptionFlags,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub load_op: VkAttachmentLoadOp,
    pub store_op: VkAttachmentStoreOp,
    pub stencil_load_op: VkAttachmentLoadOp,
    pub sencil_store_op: VkAttachmentStoreOp,
    pub initial_layout: VkImageLayout,
    pub final_layout: VkImageLayout,
}
