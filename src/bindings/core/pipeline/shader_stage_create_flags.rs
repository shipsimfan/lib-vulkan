use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkPipelineShaderStageCreateFlags(VkFlags);

impl Default for VkPipelineShaderStageCreateFlags {
    fn default() -> Self {
        VkPipelineShaderStageCreateFlags(0)
    }
}
