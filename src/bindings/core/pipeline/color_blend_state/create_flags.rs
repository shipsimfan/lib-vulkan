use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkPipelineColorBlendStateCreateFlags(VkFlags);

impl Default for VkPipelineColorBlendStateCreateFlags {
    fn default() -> Self {
        VkPipelineColorBlendStateCreateFlags(0)
    }
}
