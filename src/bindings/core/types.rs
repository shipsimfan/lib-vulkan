use std::ptr::NonNull;

extern "system" {
    pub type VkDeviceT;
    pub type VkFenceT;
    pub type VkImageT;
    pub type VkInstanceT;
    pub type VkPhysicalDeviceT;
    pub type VkQueueT;
    pub type VkSemaphoreT;
}

pub type VkBool32 = u32;
pub type VkDevice = NonNull<VkDeviceT>;
pub type VkDeviceSize = u64;
pub type VkFence = NonNull<VkFenceT>;
pub type VkFlags = u32;
pub type VkImage = NonNull<VkImageT>;
pub type VkInstance = NonNull<VkInstanceT>;
pub type VkPhysicalDevice = NonNull<VkPhysicalDeviceT>;
pub type VkQueue = NonNull<VkQueueT>;
pub type VkSemaphore = NonNull<VkSemaphoreT>;
