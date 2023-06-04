use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkSubpassDescriptionFlags(VkFlags);

impl Default for VkSubpassDescriptionFlags {
    fn default() -> Self {
        VkSubpassDescriptionFlags(0)
    }
}
