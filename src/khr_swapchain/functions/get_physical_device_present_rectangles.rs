use crate::{VkPhysicalDevice, VkRect2D, VkResult, khr_surface::VkSurfaceKhr};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_1, VkInstance, khr_surface::VkGetPhysicalDeviceSurfaceSupportKhr, khr_swapchain,
};
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
/// # Valid Usage
///  - `surface` must be supported by `physical_device`, as reported by
///    [`VkGetPhysicalDeviceSurfaceSupportKhr`] or an equivalent platform-specific mechanism
///
/// # Valid Usage (Implicit)
///  - `physical_device` must be a valid [`VkPhysicalDevice`] handle
///  - `surface` must be a valid [`VkSurfaceKhr`] handle
///  - `rect_count` must be a valid pointer to a [`u32`] value
///  - If the value referenced by `rect_count` is not 0, and `rects` is not [`null_mut`], `rects`
///    must be a valid pointer to an array of `rect_count` [`VkRect2D`] structures
///  - Both of `physical_device`, and `surface` must have been created, allocated, or retrieved
///    from the same [`VkInstance`]
///
/// # Host Synchronization
///  - Host access to `surface` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
pub type VkGetPhysicalDevicePresentRectanglesKhr = unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKhr,
    rect_count: *mut u32,
    rects: *mut VkRect2D,
) -> VkResult;

/// The name of [`VkGetPhysicalDevicePresentRectanglesKhr`]
pub const VK_GET_PHYSICAL_DEVICE_PRESENT_RECTANGLES_KHR: &CStr =
    c"vkGetPhysicalDevicePresentRectanglesKHR";
