use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkPhysicalDeviceT;
}

pub(crate) type VkPhysicalDevice = NonNull<VkPhysicalDeviceT>;
