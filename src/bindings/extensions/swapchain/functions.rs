use crate::{
    VkAllocationCallbacks, VkDevice, VkImage, VkResult, VkSwapchainCreateInfoKHR, VkSwapchainKHR,
};

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

pub(crate) type VkGetSwapchainImagesKHR = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    p_swapchain_image_count: *mut u32,
    p_swapchain_images: *mut VkImage,
) -> VkResult;
