use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkPipelineRasterizationStateCreateFlags(VkFlags);

impl Default for VkPipelineRasterizationStateCreateFlags {
    fn default() -> Self {
        VkPipelineRasterizationStateCreateFlags(0)
    }
}
