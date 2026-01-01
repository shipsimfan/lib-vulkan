use crate::{VkDevice, VkFence, VkResult, VkSemaphore, khr_swapchain::VkSwapchainKhr};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE,
    khr_swapchain::{self, VkGetSwapchainImagesKhr, VkSwapchainCreateFlagKhr},
};

/// Retrieve the index of the next available presentable image
///
/// # Parameters
///  - `device` is the device associated with swapchain.
///  - `swapchain` is the non-retired swapchain from which an image is being acquired.
///  - `timeout` specifies how long the function waits, in nanoseconds, if no image is available.
///  - `semaphore` is [`VK_NULL_HANDLE`] or a semaphore to signal.
///  - `fence` is [`VK_NULL_HANDLE`] or a fence to signal.
///  - `image_index` is a pointer to a [`u32`] in which the index of the next image to use (i.e. an
///    index into the array of images returned by [`VkGetSwapchainImagesKhr`]) is returned.
///
/// # Description
/// If the swapchain has been created with the
/// [`VkSwapchainCreateFlagKhr::DeferredMemoryAllocationBitExt`] flag, the image whose index is
/// returned in `image_index` will be fully backed by memory before this call returns to the
/// application, as if it is bound completely and contiguously to a single [`VkDeviceMemory`]
/// object.
///
/// # Valid Usage
///  - `swapchain` must not be in the retired state
///  - If `semaphore` is not [`VK_NULL_HANDLE`], it must be unsignaled
///  - If `semaphore` is not [`VK_NULL_HANDLE`], it must not have any uncompleted signal or wait
///    operations pending
///  - If `fence` is not [`VK_NULL_HANDLE`], `fence` must be unsignaled
///  - If `fence` is not [`VK_NULL_HANDLE`], `fence` must not be associated with any other queue
///    command that has not yet completed execution on that queue
///  - `semaphore` and `fence` must not both be equal to [`VK_NULL_HANDLE`]
///  - If forward progress cannot be guaranteed for the surface used to create the `swapchain`
///    member of `acquire_info`, timeout must not be [`u64::MAX`]
///  - `semaphore` must have a [`VkSemaphoreType`] of [`VkSemaphoreType::Binary`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `swapchain` must be a valid [`VkSwapchainKhr`] handle
///  - If `semaphore` is not [`VK_NULL_HANDLE`], `semaphore` must be a valid [`VkSemaphore`] handle
///  - If `fence` is not [`VK_NULL_HANDLE`], `fence` must be a valid [`VkFence`] handle
///  - `image_index` must be a valid pointer to a [`u32`] value
///  - `swapchain` must have been created, allocated, or retrieved from `device`
///  - If `semaphore` is a valid handle, it must have been created, allocated, or retrieved from
///    `device`
///  - If `fence` is a valid handle, it must have been created, allocated, or retrieved from
///    `device`
///
/// # Host Synchronization
///  - Host access to `swapchain` must be externally synchronized
///  - Host access to `semaphore` must be externally synchronized
///  - Host access to `fence` must be externally synchronized
///
/// Provided by [`khr_swapchain`]
pub type VkAcquireNextImageKhr = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKhr,
    timeout: u64,
    semaphore: VkSemaphore,
    fence: VkFence,
    image_index: *mut u32,
) -> VkResult;

/// The name of [`VkAcquireNextImageKhr`]
pub const VK_ACQUIRE_NEXT_IMAGE_KHR: &CStr = c"vkAcquireNextImageKHR";
