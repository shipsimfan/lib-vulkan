use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkPipelineTessellationStateCreateFlags(VkFlags);

impl Default for VkPipelineTessellationStateCreateFlags {
    fn default() -> Self {
        VkPipelineTessellationStateCreateFlags(0)
    }
}
