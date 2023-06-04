use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkRenderPassCreateFlags(VkFlags);

impl Default for VkRenderPassCreateFlags {
    fn default() -> Self {
        VkRenderPassCreateFlags(0)
    }
}
