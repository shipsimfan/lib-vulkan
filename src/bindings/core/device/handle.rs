use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkDeviceT;
}

pub(crate) type VkDevice = NonNull<VkDeviceT>;
