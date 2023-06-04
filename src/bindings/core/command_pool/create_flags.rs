use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkCommandPoolCreateFlags(VkFlags);

impl Default for VkCommandPoolCreateFlags {
    fn default() -> Self {
        VkCommandPoolCreateFlags(0)
    }
}
