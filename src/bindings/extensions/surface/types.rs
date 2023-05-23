use std::ptr::NonNull;

extern "system" {
    pub type VkSurfaceKHRT;
}

pub type VkSurfaceKHR = NonNull<VkSurfaceKHRT>;
