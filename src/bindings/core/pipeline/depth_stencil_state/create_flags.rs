use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkPipelineDepthStencilStateCreateFlags(VkFlags);

impl Default for VkPipelineDepthStencilStateCreateFlags {
    fn default() -> Self {
        VkPipelineDepthStencilStateCreateFlags(0)
    }
}
