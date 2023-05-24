use std::ptr::NonNull;

extern "system" {
    pub type VkSwapchainKHRT;
}

pub type VkSwapchainKHR = NonNull<VkSwapchainKHRT>;
