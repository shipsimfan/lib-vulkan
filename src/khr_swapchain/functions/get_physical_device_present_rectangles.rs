use crate::{VkPhysicalDevice, VkRect2D, VkResult, VkSurfaceKHR};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VK_VERSION_1_1};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Query present rectangles for a surface on a physical device
///
/// # Parameters
///  - `physical_device` is the physical device.
///  - `surface` is the surface.
///  - `rect_count` is a pointer to an integer related to the number of rectangles available or
///    queried, as described below.
///  - `rects` is either [`null_mut`] or a pointer to an array of [`VkRect2D`] structures.
///
/// # Description
/// If `rects` is [`null_mut`], then the number of rectangles used when presenting the given
/// surface is returned in `rect_count`. Otherwise, `rect_count` must point to a variable set by
/// the application to the number of elements in the `rects` array, and on return the variable is
/// overwritten with the number of structures actually written to `rects`. If the value of
/// `rect_count` is less than the number of rectangles, at most `rect_count` structures will be
/// written, and [`VkResult::VkIncomplete`] will be returned instead of [`VkResult::VkSuccess`], to
/// indicate that not all the available rectangles were returned.
///
/// The values returned by this command are not invariant, and may change in response to the
/// surface being moved, resized, or occluded.
///
/// The rectangles returned by this command must not overlap.
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
pub type VkGetPhysicalDevicePresentRectanglesKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    rect_count: *mut u32,
    rects: *mut VkRect2D,
) -> VkResult;

/// The name of [`VkGetPhysicalDevicePresentRectanglesKHR`]
pub const VK_GET_PHYSICAL_DEVICE_PRESENT_RECTANGLES_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDevicePresentRectanglesKHR\0") };
