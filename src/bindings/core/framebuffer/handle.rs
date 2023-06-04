use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkFramebufferT;
}

pub(crate) type VkFramebuffer = NonNull<VkFramebufferT>;
