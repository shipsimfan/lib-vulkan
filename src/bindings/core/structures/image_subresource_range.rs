use crate::VkImageAspectFlags;

#[repr(C)]
pub struct VkImageSubresourceRange {
    aspect_mask: VkImageAspectFlags,
    base_mip_level: u32,
    level_count: u32,
    base_array_layer: u32,
    layer_count: u32,
}

impl VkImageSubresourceRange {
    pub const fn new(
        aspect_mask: VkImageAspectFlags,
        base_mip_level: u32,
        level_count: u32,
        base_array_layer: u32,
        layer_count: u32,
    ) -> Self {
        VkImageSubresourceRange {
            aspect_mask,
            base_mip_level,
            level_count,
            base_array_layer,
            layer_count,
        }
    }
}
