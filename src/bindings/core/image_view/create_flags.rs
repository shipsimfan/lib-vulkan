use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkImageViewCreateFlags(VkFlags);

impl Default for VkImageViewCreateFlags {
    fn default() -> Self {
        VkImageViewCreateFlags(0)
    }
}
