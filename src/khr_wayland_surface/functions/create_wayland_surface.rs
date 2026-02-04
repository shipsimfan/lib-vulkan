use crate::{
    VkAllocationCallbacks, VkInstance, VkResult, khr_surface::VkSurfaceKhr,
    khr_wayland_surface::VkWaylandSurfaceCreateInfoKhr,
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_wayland_surface;
#[allow(unused_imports)]
use std::ptr::null;

/// Create a [`VkSurfaceKhr`] object for a Wayland window
///
/// # Parameters
///  - `instance` is the instance to associate the surface with.
///  - `create_info` is a pointer to a [`VkWaylandSurfaceCreateInfoKhr`] structure containing
///    parameters affecting the creation of the surface object.
///  - `allocator` is the allocator used for host memory allocated for the surface object when
///    there is no more specific allocator available.
///  - `surface` is a pointer to a [`VkSurfaceKhr`] handle in which the created surface object is
///    returned.
///
/// # Valid Usage (Implicit)
///  - `instance` must be a valid [`VkInstance`] handle
///  - `create_info` must be a valid pointer to a valid [`VkWaylandSurfaceCreateInfoKhr`] structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `surface` must be a valid pointer to a [`VkSurfaceKhr`] handle
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`khr_wayland_surface`]
pub type VkCreateWaylandSurfaceKhr = unsafe extern "system" fn(
    instance: VkInstance,
    create_info: *const VkWaylandSurfaceCreateInfoKhr,
    allocatior: *const VkAllocationCallbacks,
    surface: *mut VkSurfaceKhr,
) -> VkResult;

/// The name of [`VkCreateWaylandSurfaceKhr`]
pub const VK_CREATE_WAYLAND_SURFACE_KHR: &CStr = c"vkCreateWaylandSurfaceKHR";
