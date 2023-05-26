use crate::{
    VkAllocationCallbacks, VkInstance, VkResult, VkSurfaceKHR, VkWin32SurfaceCreateInfoKHR,
};

pub(crate) type VkCreateWin32SurfaceKHR = extern "system" fn(
    instance: VkInstance,
    p_create_info: *const VkWin32SurfaceCreateInfoKHR,
    p_allocator: *const VkAllocationCallbacks,
    p_surface: *mut Option<VkSurfaceKHR>,
) -> VkResult;
