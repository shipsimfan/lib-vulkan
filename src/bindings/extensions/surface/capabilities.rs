use crate::{
    VkCompositeAlphaFlagsKHR, VkExtent2D, VkImageUsageFlags, VkSurfaceTransformFlagBitsKHR,
    VkSurfaceTransformFlagsKHR,
};

#[repr(C)]
#[derive(Default)]
pub struct VkSurfaceCapabilitiesKHR {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: VkExtent2D,
    pub min_image_extent: VkExtent2D,
    pub max_image_extent: VkExtent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: VkSurfaceTransformFlagsKHR,
    pub current_transform: VkSurfaceTransformFlagBitsKHR,
    pub supported_composite_alpha: VkCompositeAlphaFlagsKHR,
    pub supported_usage_flags: VkImageUsageFlags,
}
