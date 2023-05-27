use crate::{VkAllocationCallbacks, VkDevice, VkResult, VkSwapchainCreateInfoKHR, VkSwapchainKHR};

pub(crate) type VkCreateSwapchainKHR = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkSwapchainCreateInfoKHR,
    p_allocator: *const VkAllocationCallbacks,
    p_swapchain: *mut Option<VkSwapchainKHR>,
) -> VkResult;

pub(crate) type VkDestroySwapchainKHR = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    p_allocator: *const VkAllocationCallbacks,
);
