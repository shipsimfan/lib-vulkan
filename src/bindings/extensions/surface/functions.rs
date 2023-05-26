use crate::{VkAllocationCallbacks, VkInstance, VkSurfaceKHR};

pub(crate) type VkDestroySurfaceKHR = extern "system" fn(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    p_allocator: *const VkAllocationCallbacks,
);
