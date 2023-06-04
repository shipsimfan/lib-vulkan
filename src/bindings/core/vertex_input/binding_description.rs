use crate::VkVertexInputRate;

#[repr(C)]
#[derive(Clone)]
pub struct VkVertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VkVertexInputRate,
}
