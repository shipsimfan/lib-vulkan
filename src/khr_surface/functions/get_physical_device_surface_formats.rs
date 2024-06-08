use crate::{VkPhysicalDevice, VkResult, VkSurfaceFormatKHR, VkSurfaceKHR};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_surface, VkColorSpaceKHR, VkFormat, VK_NULL_HANDLE};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Query color formats supported by surface
///
/// # Parameters
///  - `physical_device` is the physical device that will be associated with the swapchain to be
///    created, as described for [`VkCreateSwapchainKHR`].
///  - `surface` is the surface that will be associated with the swapchain.
///  - `surface_format_count` is a pointer to an integer related to the number of format pairs
///    available or queried, as described below.
///  - `surface_formats` is either [`null_mut`] or a pointer to an array of [`VkSurfaceFormatKHR`]
///    structures.
///
/// # Description
/// If `surface_formats` is [`null_mut`], then the number of format pairs supported for the given
/// surface is returned in `surface_format_count`. Otherwise, `surface_format_count` must point to
/// a variable set by the user to the number of elements in the `surface_formats` array, and on
/// return the variable is overwritten with the number of structures actually written to
/// `surface_formats`. If the value of `surface_format_count` is less than the number of format
/// pairs supported, at most `surface_format_count` structures will be written, and
/// [`VkResult::VkIncomplete`] will be returned instead of [`VkResult::VkSuccess`], to indicate
/// that not all the available format pairs were returned.
///
/// The number of format pairs supported must be greater than or equal to 1. `surface_formats` must
/// not contain an entry whose value for format is [`VkFormat::Undefined`].
///
/// If `surface_formats` includes an entry whose value for `color_space` is  
/// [`VkColorSpaceKHR::SRGBNonlinearKHR`] and whose value for format is a `UNORM` (or `SRGB`)
/// format and the corresponding `SRGB` (or `UNORM`) format is a color renderable format for
/// [`VkImageTiling::Optimal`], then `surface_formats` must also contain an entry with the same
/// value for `color_space` and format equal to the corresponding `SRGB` (or `UNORM`) format.
///
/// If the [`google_surfaceless_query`] extension is enabled, the values returned in
/// `surface_formats` will be identical for every valid surface created on this physical device,
/// and so surface can be [`VK_NULL_HANDLE`].
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
pub type VkGetPhysicalDeviceSurfaceFormatsKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    surface_format_count: *mut u32,
    surface_formats: *mut VkSurfaceFormatKHR,
) -> VkResult;

/// The name of [`VkGetPhysicalDeviceSurfaceFormatsKHR`]
pub const VK_GET_PHYSICAL_DEVICE_SURFACE_FORMAT_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceSurfaceFormatsKHR\0") };
