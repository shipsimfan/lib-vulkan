use crate::{VkExtent2D, VkOffset2D};

#[repr(C)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}
