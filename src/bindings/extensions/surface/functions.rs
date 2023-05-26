use crate::{
    VkAllocationCallbacks, VkBool32, VkInstance, VkPhysicalDevice, VkResult, VkSurfaceKHR,
};

pub(crate) type VkDestroySurfaceKHR = extern "system" fn(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    p_allocator: *const VkAllocationCallbacks,
);

pub(crate) type VkGetPhysicalDeviceSurfaceSupportKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    surface: VkSurfaceKHR,
    p_supported: *mut VkBool32,
) -> VkResult;
