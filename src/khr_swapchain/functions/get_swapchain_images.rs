use crate::{VkDevice, VkImage, VkResult, VkSwapchainKHR};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_swapchain;
#[allow(unused_imports)]
use std::ptr::null;

/// Obtain the array of presentable images associated with a swapchain
///
/// # Parameters
///  - `device` is the device associated with swapchain.
///  - `swapchain` is the swapchain to query.
///  - `swapchain_image_count` is a pointer to an integer related to the number of presentable
///    images available or queried, as described below.
///  - `swapchain_images` is either [`null`] or a pointer to an array of [`VkImage`] handles.
///
/// # Description
///
/// If `swapchain_images` is [`null`], then the number of presentable images for swapchain is
/// returned in `swapchain_image_count`. Otherwise, `swapchain_image_count` must point to a
/// variable set by the application to the number of elements in the `swapchain_images` array, and
/// on return the variable is overwritten with the number of structures actually written to
/// `swapchain_images`. If the value of `swapchain_image_count` is less than the number of
/// presentable images for swapchain, at most `swapchain_image_count` structures will be written,
/// and [`VkResult::VkIncomplete`] will be returned instead of [`VkResult::VkSuccess`], to indicate
/// that not all the available presentable images were returned.
///
/// Provided by [`khr_swapchain`]
pub type VkGetSwapchainImagesKHR = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    swapchain_image_count: *mut u32,
    swapchain_images: *mut VkImage,
) -> VkResult;

/// The name of [`VkGetSwapchainImagesKHR`]
pub const VK_GET_SWAPCHAIN_IMAGES_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainImagesKHR\0") };
