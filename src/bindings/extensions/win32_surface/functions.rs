use crate::{
    bindings::{VkBool32, VkInstance, VkPhysicalDevice, VkSurfaceKHR},
    VkResult, VkWin32SurfaceCreateInfoKHR,
};
use std::{ffi::c_void, ptr::NonNull};

pub type VkCreateWin32SurfaceKHR = extern "system" fn(
    instance: VkInstance,
    p_create_info: &VkWin32SurfaceCreateInfoKHR,
    p_allocator: Option<NonNull<c_void>>,
    p_surface: &mut Option<VkSurfaceKHR>,
) -> VkResult;

pub type VkGetPhysicalDeviceWin32PresentationSupportKHR =
    extern "system" fn(physical_device: VkPhysicalDevice, queue_family_index: u32) -> VkBool32;
