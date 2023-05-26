use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkWin32SurfaceCreateFlagsKHR(VkFlags);

impl Default for VkWin32SurfaceCreateFlagsKHR {
    fn default() -> Self {
        VkWin32SurfaceCreateFlagsKHR(0)
    }
}
