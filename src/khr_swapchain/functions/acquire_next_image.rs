use crate::{VkDevice, VkFence, VkResult, VkSemaphore, VkSwapchainKHR};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VkSwapchainCreateFlagBitsKHR, VK_NULL_HANDLE};

/// Retrieve the index of the next available presentable image
///
/// # Parameters
///  - `device` is the device associated with swapchain.
///  - `swapchain` is the non-retired swapchain from which an image is being acquired.
///  - `timeout` specifies how long the function waits, in nanoseconds, if no image is available.
///  - `semaphore` is [`VK_NULL_HANDLE`] or a semaphore to signal.
///  - `fence` is [`VK_NULL_HANDLE`] or a fence to signal.
///  - `image_index` is a pointer to a [`u32`] in which the index of the next image to use (i.e. an
///    index into the array of images returned by [`VkGetSwapchainImagesKHR`]) is returned.
///
/// # Description
/// If the swapchain has been created with the
/// [`VkSwapchainCreateFlagBitsKHR::DeferredMemoryAllocationBitExt`] flag, the image whose index is
/// returned in `image_index` will be fully backed by memory before this call returns to the
/// application, as if it is bound completely and contiguously to a single VkDeviceMemory object.
///
/// Provided by [`khr_swapchain`]
pub type VkAcquireNextImageKHR = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    timeout: u64,
    semaphore: VkSemaphore,
    fence: VkFence,
    image_index: *mut u32,
) -> VkResult;

/// The name of [`VkDestroySwapchainKHR`]
pub const VK_ACQUIRE_NEXT_IMAGE_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImageKHR\0") };
