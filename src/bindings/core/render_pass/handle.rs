use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkRenderPassT;
}

pub(crate) type VkRenderPass = NonNull<VkRenderPassT>;
