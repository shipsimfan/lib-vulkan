use crate::{VkDevice, VkDeviceGroupPresentModeFlagsKHR, VkResult, VkSurfaceKHR};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VkGetDeviceGroupPresentCapabilitiesKHR, VK_VERSION_1_1};

/// Query present capabilities for a surface
///
/// # Parameters
///  - `device` is the logical device.
///  - `surface` is the surface.
///  - `modes` is a pointer to a [`VkDeviceGroupPresentModeFlagsKHR`] in which the supported device
///    group present modes for the surface are returned.
///
/// # Description
/// The modes returned by this command are not invariant, and may change in response to the surface
/// being moved, resized, or occluded. These modes must be a subset of the modes returned by
/// [`VkGetDeviceGroupPresentCapabilitiesKHR`].
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
pub type VkGetDeviceGroupSurfacePresentModesKHR = extern "system" fn(
    device: VkDevice,
    surface: VkSurfaceKHR,
    modes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult;

/// The name of [`VkAcquireNextImage2KHR`]
pub const VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceGroupSurfacePresentModesKHR\0") };
