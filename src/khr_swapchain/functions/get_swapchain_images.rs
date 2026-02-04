use crate::{VkDevice, VkImage, VkResult, khr_swapchain::VkSwapchainKhr};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_swapchain;
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Obtain the array of presentable images associated with a swapchain
///
/// # Parameters
///  - `device` is the device associated with swapchain.
///  - `swapchain` is the swapchain to query.
///  - `swapchain_image_count` is a pointer to an integer related to the number of presentable
///    images available or queried, as described below.
///  - `swapchain_images` is either [`null_mut`] or a pointer to an array of [`VkImage`] handles.
///
/// # Description
/// If `swapchain_images` is [`null_mut`], then the number of presentable images for swapchain is
/// returned in `swapchain_image_count`. Otherwise, `swapchain_image_count` must point to a
/// variable set by the application to the number of elements in the `swapchain_images` array, and
/// on return the variable is overwritten with the number of structures actually written to
/// `swapchain_images`. If the value of `swapchain_image_count` is less than the number of
/// presentable images for swapchain, at most `swapchain_image_count` structures will be written,
/// and [`VkResult::VkIncomplete`] will be returned instead of [`VkResult::VkSuccess`], to indicate
/// that not all the available presentable images were returned.
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `swapchain` must be a valid [`VkSwapchainKhr`] handle
///  - `swapchain_image_count` must be a valid pointer to a [`u32`] value
///  - If the value referenced by `swapchain_image_count` is not 0, and `swapchain_images` is not
///    [`null_mut`], `swapchain_images` must be a valid pointer to an array of
///    `swapchain_image_count` [`VkImage`] handles
///  - `swapchain` must have been created, allocated, or retrieved from `device`
///
/// Provided by [`khr_swapchain`]
pub type VkGetSwapchainImagesKhr = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKhr,
    swapchain_image_count: *mut u32,
    swapchain_images: *mut VkImage,
) -> VkResult;

/// The name of [`VkGetSwapchainImagesKhr`]
pub const VK_GET_SWAPCHAIN_IMAGES_KHR: &CStr = c"vkGetSwapchainImagesKHR";
