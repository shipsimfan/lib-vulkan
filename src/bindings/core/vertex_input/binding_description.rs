use crate::VkVertexInputRate;

#[repr(C)]
pub struct VkVertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VkVertexInputRate,
}
