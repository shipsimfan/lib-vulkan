use crate::VkShaderStageFlags;

#[repr(C)]
pub struct VkPushConstantRange {
    pub stage_flags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}
