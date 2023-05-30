use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkPipelineLayoutCreateFlags(VkFlags);

impl Default for VkPipelineLayoutCreateFlags {
    fn default() -> Self {
        VkPipelineLayoutCreateFlags(0)
    }
}
