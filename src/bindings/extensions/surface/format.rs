use crate::{VkColorSpaceKHR, VkFormat};

#[repr(C)]
pub struct VkSurfaceFormatKHR {
    pub format: VkFormat,
    pub color_space: VkColorSpaceKHR,
}
