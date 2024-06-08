use crate::{VkPhysicalDevice, VkPresentModeKHR, VkResult, VkSurfaceKHR};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_surface, VK_NULL_HANDLE};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Query supported presentation modes
///
/// # Parameters
///  - `physical_device` is the physical device that will be associated with the swapchain to be
///    created, as described for [`VkCreateSwapchainKHR`].
///  - `surface` is the surface that will be associated with the swapchain.
///  - `present_mode_count` is a pointer to an integer related to the number of presentation modes
///    available or queried, as described below.
///  - `present_modes` is either [`null_mut`] or a pointer to an array of [`VkPresentModeKHR`]
///    values, indicating the supported presentation modes.
///
/// # Description
/// If `present_modes` is [`null_mut`], then the number of presentation modes supported for the
/// given surface is returned in `present_mode_count`. Otherwise, `present_mode_count` must point
/// to a variable set by the user to the number of elements in the `present_modes` array, and on
/// return the variable is overwritten with the number of values actually written to
/// `present_modes`. If the value of `present_mode_count` is less than the number of presentation
/// modes supported, at most `present_mode_count` values will be written, and
/// [`VkResult::VkIncomplete`] will be returned instead of [`VkResult::VkSuccess`], to indicate
/// that not all the available modes were returned.
///
/// If the [`google_surfaceless_query`] extension is enabled and surface is [`VK_NULL_HANDLE`], the
/// values returned in `present_mode` will only indicate support for [`VkPresentModeKHR::FIFOKHR`],
/// [`VkPresentModeKHR::DemandRefreshKHR`], and [`VkPresentModeKHR::ContinousRefreshKHR`]. To query
/// support for any other present mode, a valid handle must be provided in surface.
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///  - [`VkResult::VkIncomplete`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorSurfaceLostKHR`]
///
/// Provided by [`khr_surface`]
pub type VkGetPhysicalDeviceSurfacePresentModesKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    present_mode_count: *mut u32,
    present_modes: *mut VkPresentModeKHR,
) -> VkResult;

/// The name of [`VkGetPhysicalDeviceSurfacePresentModesKHR`]
pub const VK_GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceSurfacePresentModesKHR\0") };
