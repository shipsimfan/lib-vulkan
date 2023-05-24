use super::VkSwapchainKHR;
use crate::bindings::{VkFence, VkSemaphore, VkStructureType};
use std::{ffi::c_void, ptr::NonNull};

#[repr(C)]
pub struct VkAcquireNextImageInfoKHR {
    s_type: VkStructureType,
    p_next: Option<NonNull<c_void>>,
    swapchain: VkSwapchainKHR,
    timeout: u64,
    semaphore: VkSemaphore,
    fence: VkFence,
    device_mask: u32,
}
