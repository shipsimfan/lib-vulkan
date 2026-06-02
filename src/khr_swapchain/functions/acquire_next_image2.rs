use crate::{VkDevice, VkResult, khr_swapchain::VkAcquireNextImageInfoKhr};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_1,
    khr_swapchain::{self, VkSwapchainCreateFlagKhr},
};

/// Retrieve the index of the next available presentable image
///
/// # Parameters
///  - `device` is the device associated with swapchain.
///  - `acquire_info` is a pointer to a [`VkAcquireNextImageInfoKhr`] structure containing
///    parameters of the acquire.
///  - `image_index` is a pointer to a [`u32`] that is set to the index of the next image to use.
///
/// # Description
/// If the swapchain has been created with the
/// [`VkSwapchainCreateFlagKhr::DeferredMemoryAllocationExt`] flag, the image whose index is
/// returned in `image_index` will be fully backed by memory before this call returns to the
/// application.
///
/// # Valid Usage
///  - If forward progress cannot be guaranteed for the surface used to create `swapchain`, the
///    `timeout` member of `acquire_info` must not be [`u64::MAX`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `acquire_info` must be a valid pointer to a valid [`VkAcquireNextImageInfoKhr`] structure
///  - `image_index` must be a valid pointer to a [`u32`] value
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
pub type VkAcquireNextImage2Khr = unsafe extern "system" fn(
    device: VkDevice,
    acquire_info: *const VkAcquireNextImageInfoKhr,
    image_index: *mut u32,
) -> VkResult;

/// The name of [`VkAcquireNextImage2Khr`]
pub const VK_ACQUIRE_NEXT_IMAGE2_KHR: &CStr = c"vkAcquireNextImage2KHR";
