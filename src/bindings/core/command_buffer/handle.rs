use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkCommandBufferT;
}

pub(crate) type VkCommandBuffer = NonNull<VkCommandBufferT>;
