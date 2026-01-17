use crate::{VkBool32, VkPhysicalDevice};
use std::ffi::CStr;
use wayland::wl_display;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VkGetPhysicalDeviceQueueFamilyProperties, khr_wayland_surface};
#[allow(unused_imports)]
use std::ptr::null;

/// Query physical device for presentation to Wayland
///
/// # Parameters
///  - `physical_device` is the physical device.
///  - `queue_family_index` is the queue family index.
///  - `display` is a pointer to the [`wl_display`] associated with a Wayland compositor.
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
///  - `display` must be a valid pointer to a [`wl_display`] value
///
/// Provided by [`khr_wayland_surface`]
pub type VkGetPhysicalDeviceWaylandPresentationSupportKhr = extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    display: *mut wl_display,
) -> VkBool32;

/// The name of [`VkGetPhysicalDeviceWaylandPresentationSupportKhr`]
pub const VK_GET_PHYSICAL_DEVICE_WAYLAND_PRESENTATION_SUPPORT_KHR: &CStr =
    c"vkGetPhysicalDeviceWaylandPresentationSupportKHR";
