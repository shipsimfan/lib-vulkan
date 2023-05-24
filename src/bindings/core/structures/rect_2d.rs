use crate::{VkExtent2D, VkOffset2D};

#[repr(C)]
pub struct VkRect2D {
    offset: VkOffset2D,
    extent: VkExtent2D,
}

impl VkRect2D {
    pub const fn new(offset: VkOffset2D, extent: VkExtent2D) -> Self {
        VkRect2D { offset, extent }
    }

    pub const fn offset(&self) -> &VkOffset2D {
        &self.offset
    }

    pub const fn extent(&self) -> &VkExtent2D {
        &self.extent
    }

    pub fn set_offset(&mut self, offset: VkOffset2D) {
        self.offset = offset;
    }

    pub fn set_extent(&mut self, extent: VkExtent2D) {
        self.extent = extent;
    }
}
