use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkFramebufferCreateFlags(VkFlags);

impl Default for VkFramebufferCreateFlags {
    fn default() -> Self {
        VkFramebufferCreateFlags(0)
    }
}
