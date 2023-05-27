use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkSwapchainKHRT;
}

pub(crate) type VkSwapchainKHR = NonNull<VkSwapchainKHRT>;
