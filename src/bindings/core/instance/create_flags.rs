use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkInstanceCreateFlags(VkFlags);

impl Default for VkInstanceCreateFlags {
    fn default() -> Self {
        VkInstanceCreateFlags(0)
    }
}
