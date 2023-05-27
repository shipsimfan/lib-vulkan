use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkSwapchainCreateFlagsKHR(VkFlags);

impl Default for VkSwapchainCreateFlagsKHR {
    fn default() -> Self {
        VkSwapchainCreateFlagsKHR(0)
    }
}
