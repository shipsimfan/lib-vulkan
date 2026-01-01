use crate::{
    VkDevice, VkResult, khr_surface::VkSurfaceKhr, khr_swapchain::VkDeviceGroupPresentModeFlagsKhr,
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_1, VkInstance,
    khr_swapchain::{self, VkGetDeviceGroupPresentCapabilitiesKhr},
};

/// Query present capabilities for a surface
///
/// # Parameters
///  - `device` is the logical device.
///  - `surface` is the surface.
///  - `modes` is a pointer to a [`VkDeviceGroupPresentModeFlagsKhr`] in which the supported device
///    group present modes for the surface are returned.
///
/// # Description
/// The modes returned by this command are not invariant, and may change in response to the surface
/// being moved, resized, or occluded. These modes must be a subset of the modes returned by
/// [`VkGetDeviceGroupPresentCapabilitiesKhr`].
///
/// # Valid Usage
///  - `surface` must be supported by all physical devices associated with `device`, as reported by
///    [`VkGetPhysicalDeviceSurfaceSupportKhr`] or an equivalent platform-specific mechanism
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `surface` must be a valid [`VkSurfaceKhr`] handle
///  - `modes` must be a valid pointer to a [`VkDeviceGroupPresentModeFlagsKhr`] value
///  - Both of `device`, and `surface` must have been created, allocated, or retrieved from the
///    same [`VkInstance`]
///
/// # Host Synchronization
///  - Host access to `surface` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
pub type VkGetDeviceGroupSurfacePresentModesKhr = extern "system" fn(
    device: VkDevice,
    surface: VkSurfaceKhr,
    modes: *mut VkDeviceGroupPresentModeFlagsKhr,
) -> VkResult;

/// The name of [`VkGetDeviceGroupSurfacePresentModesKhr`]
pub const VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES_KHR: &CStr =
    c"vkGetDeviceGroupSurfacePresentModesKHR";
