use crate::VkImageLayout;

#[repr(C)]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: VkImageLayout,
}
