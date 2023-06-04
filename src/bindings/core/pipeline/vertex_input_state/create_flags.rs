use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkPipelineVertexInputStateCreateFlags(VkFlags);

impl Default for VkPipelineVertexInputStateCreateFlags {
    fn default() -> Self {
        VkPipelineVertexInputStateCreateFlags(0)
    }
}
