use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkQueueT;
}

pub(crate) type VkQueue = NonNull<VkQueueT>;
