use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkPipelineDynamicStateCreateFlags(VkFlags);

impl Default for VkPipelineDynamicStateCreateFlags {
    fn default() -> Self {
        VkPipelineDynamicStateCreateFlags(0)
    }
}
