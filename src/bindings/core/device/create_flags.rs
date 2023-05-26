use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkDeviceCreateFlags(VkFlags);

impl Default for VkDeviceCreateFlags {
    fn default() -> Self {
        VkDeviceCreateFlags(0)
    }
}
