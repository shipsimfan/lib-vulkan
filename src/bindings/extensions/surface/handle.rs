use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkSurfaceKHRT;
}

pub(crate) type VkSurfaceKHR = NonNull<VkSurfaceKHRT>;
