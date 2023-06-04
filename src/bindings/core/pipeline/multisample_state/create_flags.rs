use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkPipelineMultisampleStateCreateFlags(VkFlags);

impl Default for VkPipelineMultisampleStateCreateFlags {
    fn default() -> Self {
        VkPipelineMultisampleStateCreateFlags(0)
    }
}
