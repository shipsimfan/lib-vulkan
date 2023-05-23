use std::ptr::NonNull;

extern "system" {
    pub type VkDeviceT;
    pub type VkInstanceT;
    pub type VkPhysicalDeviceT;
    pub type VkQueueT;
}

pub type VkBool32 = u32;
pub type VkDevice = NonNull<VkDeviceT>;
pub type VkDeviceSize = u64;
pub type VkFlags = u32;
pub type VkInstance = NonNull<VkInstanceT>;
pub type VkPhysicalDevice = NonNull<VkPhysicalDeviceT>;
pub type VkQueue = NonNull<VkQueueT>;
