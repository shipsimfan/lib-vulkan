use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkDeviceQueueCreateFlags(VkFlags);

impl Default for VkDeviceQueueCreateFlags {
    fn default() -> Self {
        VkDeviceQueueCreateFlags(0)
    }
}
