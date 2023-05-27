use crate::{VkImageAspectFlagBits, VkImageAspectFlags};

#[repr(C)]
#[derive(Clone)]
pub struct VkImageSubresourceRange {
    pub aspect_mask: VkImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

impl Default for VkImageSubresourceRange {
    fn default() -> Self {
        VkImageSubresourceRange {
            aspect_mask: VkImageAspectFlags::new(&[VkImageAspectFlagBits::Color]),
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        }
    }
}
