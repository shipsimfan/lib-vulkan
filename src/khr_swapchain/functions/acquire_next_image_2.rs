use crate::{VkAcquireNextImageInfoKHR, VkDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VkSwapchainCreateFlagBitsKHR, VK_VERSION_1_1};

/// Retrieve the index of the next available presentable image
///
/// # Parameters
///  - `device` is the device associated with swapchain.
///  - `acquire_info` is a pointer to a [`VkAcquireNextImageInfoKHR`] structure containing
///    parameters of the acquire.
///  - `image_index` is a pointer to a [`u32`] that is set to the index of the next image to use.
///
/// # Description
/// If the swapchain has been created with the
/// [`VkSwapchainCreateFlagBitsKHR::DeferredMemoryAllocationBitExt`] flag, the image whose index is
/// returned in `image_index` will be fully backed by memory before this call returns to the
/// application.
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
pub type VkAcquireNextImage2KHR = extern "system" fn(
    device: VkDevice,
    acquire_info: *const VkAcquireNextImageInfoKHR,
    image_index: *mut u32,
) -> VkResult;

/// The name of [`VkAcquireNextImage2KHR`]
pub const VK_ACQUIRE_NEXT_IMAGE_2_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImage2KHR\0") };
