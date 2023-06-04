use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkCommandPoolT;
}

pub(crate) type VkCommandPool = NonNull<VkCommandPoolT>;
