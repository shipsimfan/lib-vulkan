use super::{
    VkAcquireNextImageInfoKHR, VkDeviceGroupPresentCapabilitiesKHR,
    VkDeviceGroupPresentModeFlagsKHR, VkPresentInfoKHR, VkSwapchainKHR,
};
use crate::{
    bindings::{VkDevice, VkFence, VkImage, VkQueue, VkSemaphore, VkSurfaceKHR},
    VkPhysicalDevice, VkRect2D, VkResult, VkSwapchainCreateInfoKHR,
};
use std::{ffi::c_void, ptr::NonNull};

pub type VkAcquireNextImageKHR = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    timeout: u64,
    semaphore: Option<VkSemaphore>,
    fence: Option<VkFence>,
    p_image_index: &mut u32,
) -> VkResult;

pub type VkCreateSwapchainKHR = extern "system" fn(
    device: VkDevice,
    p_create_info: &VkSwapchainCreateInfoKHR,
    p_allocator: Option<NonNull<c_void>>,
    p_swapchain: &mut Option<VkSwapchainKHR>,
) -> VkResult;

pub type VkDestroySwapchainKHR = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    p_allocator: Option<NonNull<c_void>>,
);

pub type VkGetSwapchainImagesKHR = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    p_swapchain_image_count: &mut u32,
    p_swapchain_images: Option<NonNull<VkImage>>,
) -> VkResult;

pub type VkQueuePresentKHR =
    extern "system" fn(queue: VkQueue, p_present_info: &VkPresentInfoKHR) -> VkResult;

pub type VkAcquireNextImage2KHR = extern "system" fn(
    device: VkDevice,
    p_acquire_info: &VkAcquireNextImageInfoKHR,
    p_image_index: &mut u32,
) -> VkResult;

pub type VkGetDeviceGroupPresentCapabilitiesKHR = extern "system" fn(
    device: VkDevice,
    p_device_group_present_capabilities: &mut VkDeviceGroupPresentCapabilitiesKHR,
) -> VkResult;

pub type VkGetDeviceGroupSurfacePresentModesKHR = extern "system" fn(
    device: VkDevice,
    surface: VkSurfaceKHR,
    p_modes: &mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult;

pub type VkGetPhysicalDevicePresentRectanglesKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_rect_count: &mut u32,
    p_rects: Option<NonNull<VkRect2D>>,
);
