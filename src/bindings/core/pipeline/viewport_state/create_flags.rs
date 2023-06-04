use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkPipelineViewportStateCreateFlags(VkFlags);

impl Default for VkPipelineViewportStateCreateFlags {
    fn default() -> Self {
        VkPipelineViewportStateCreateFlags(0)
    }
}
