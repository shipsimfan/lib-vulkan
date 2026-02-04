use crate::{VkBool32, VkPhysicalDevice};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VkGetPhysicalDeviceQueueFamilyProperties, khr_win32_surface};

/// Query queue family support for presentation on a Win32 display
///
/// # Parameters
///  - `physical_device` is the physical device.
///  - `queue_family_index` is the queue family index.
///
/// # Description
/// This platform-specific function can be called prior to creating a surface.
///
/// # Valid Usage
///  - `queue_family_index` must be less than `queue_family_property_count` returned by
///    [`VkGetPhysicalDeviceQueueFamilyProperties`] for the given `physical_device`
///
/// # Valid Usage (Implicit)
///  - `physical_device` must be a valid [`VkPhysicalDevice`] handle
///
/// Provided by [`khr_win32_surface`]
pub type VkGetPhysicalDeviceWin32PresentationSupportKhr =
    unsafe extern "system" fn(physical_device: VkPhysicalDevice, queue_family_index: u32) -> VkBool32;

/// The name of [`VkGetPhysicalDeviceWin32PresentationSupportKhr`]
pub const VK_GET_PHYSICAL_DEVICE_WIN32_PRESENTATION_SUPPORT_KHR: &CStr = unsafe {
    CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceWin32PresentationSupportKHR\0")
};
