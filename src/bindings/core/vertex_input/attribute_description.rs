use crate::bindings::VkFormat;

#[repr(C)]
#[derive(Clone)]
pub struct VkVertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: VkFormat,
    pub offset: u32,
}
