use crate::VkComponentSwizzle;

#[repr(C)]
#[derive(Clone, Default)]
pub struct VkComponentMapping {
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}
