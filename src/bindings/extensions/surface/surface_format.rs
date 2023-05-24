use crate::{VkColorSpaceKHR, VkFormat};

#[repr(C)]
#[derive(Clone)]
pub struct VkSurfaceFormatKHR {
    format: VkFormat,
    color_space: VkColorSpaceKHR,
}

impl VkSurfaceFormatKHR {
    pub fn format(&self) -> VkFormat {
        self.format
    }

    pub fn color_space(&self) -> VkColorSpaceKHR {
        self.color_space
    }
}
