use crate::{
    VkPhysicalDevice, VkResult,
    khr_surface::{VkPresentModeKhr, VkSurfaceKhr},
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VkInstance,
    khr_surface::{self, VkGetPhysicalDeviceSurfaceSupportKhr},
};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Query supported presentation modes
///
/// # Parameters
///  - `physical_device` is the physical device that will be associated with the swapchain to be
///    created, as described for [`VkCreateSwapchainKhr`].
///  - `surface` is the surface that will be associated with the swapchain.
///  - `present_mode_count` is a pointer to an integer related to the number of presentation modes
///    available or queried, as described below.
///  - `present_modes` is either [`null_mut`] or a pointer to an array of [`VkPresentModeKhr`]
///    values, indicating the supported presentation modes.
///
/// # Description
/// If `present_modes` is [`null_mut`], then the number of presentation modes supported for the
/// given `surface` is returned in `present_mode_count`. Otherwise, `present_mode_count` must point
/// to a variable set by the user to the number of elements in the `present_modes` array, and on
/// return the variable is overwritten with the number of values actually written to
/// `present_modes`. If the value of `present_mode_count` is less than the number of presentation
/// modes supported, at most `present_mode_count` values will be written, and
/// [`VkResult::VkIncomplete`] will be returned instead of [`VkResult::VkSuccess`], to indicate
/// that not all the available modes were returned.
///
/// If the [`google_surfaceless_query`] extension is enabled and `surface` is [`VK_NULL_HANDLE`], the
/// values returned in `present_mode` will only indicate support for [`VkPresentModeKhr::FIFOKhr`],
/// [`VkPresentModeKhr::DemandRefreshKhr`], and [`VkPresentModeKhr::ContinousRefreshKhr`]. To query
/// support for any other present mode, a valid handle must be provided in `surface`.
///
/// # Valid Usage
///  - If the `google_surfaceless_query` extension is not enabled, `surface` must be a valid
///    [`VkSurfaceKhr`] handle
///  - If `surface` is not [`VK_NULL_HANDLE`], `surface` must be supported by `physical_device`, as
///    reported by [`VkGetPhysicalDeviceSurfaceSupportKhr`] or an equivalent platform-specific
///    mechanism
///
/// # Valid Usage (Implicit)
///  - `physical_device` must be a valid [`VkPhysicalDevice`] handle
///  - If `surface` is not [`VK_NULL_HANDLE`], `surface` must be a valid [`VkSurfaceKhr`] handle
///  - `present_mode_count` must be a valid pointer to a [`u32`] value
///  - If the value referenced by `present_mode_count` is not 0, and `present_modes` is not
///    [`null_mut`], `present_modes` must be a valid pointer to an array of `present_mode_count`
///    [`VkPresentModeKhr`] values
///  - Both of `physical_device`, and `surface` that are valid handles of non-ignored parameters
///    must have been created, allocated, or retrieved from the same [`VkInstance`]
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///  - [`VkResult::VkIncomplete`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorSurfaceLostKhr`]
///
/// Provided by [`khr_surface`]
pub type VkGetPhysicalDeviceSurfacePresentModesKhr = unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKhr,
    present_mode_count: *mut u32,
    present_modes: *mut VkPresentModeKhr,
) -> VkResult;

/// The name of [`VkGetPhysicalDeviceSurfacePresentModesKhr`]
pub const VK_GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES_KHR: &CStr =
    c"vkGetPhysicalDeviceSurfacePresentModesKHR";
