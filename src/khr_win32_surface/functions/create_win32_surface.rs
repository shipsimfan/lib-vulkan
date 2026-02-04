use crate::{
    VkAllocationCallbacks, VkInstance, VkResult, khr_surface::VkSurfaceKhr,
    khr_win32_surface::VkWin32SurfaceCreateInfoKhr,
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_win32_surface;
#[allow(unused_imports)]
use std::ptr::null;

/// Create a [`VkSurfaceKhr`] object for a Win32 native window
///
/// # Parameters
///  - `instance` is the instance to associate the surface with.
///  - `create_info` is a pointer to a [`VkWin32SurfaceCreateInfoKhr`] structure containing
///    parameters affecting the creation of the surface object.
///  - `allocator` is the allocator used for host memory allocated for the surface object when
///    there is no more specific allocator available.
///  - `surface` is a pointer to a [`VkSurfaceKhr`] handle in which the created surface object is
///    returned.
///
/// # Valid Usage (Implicit)
///  - `instance` must be a valid [`VkInstance`] handle
///  - `create_info` must be a valid pointer to a valid [`VkWin32SurfaceCreateInfoKhr`] structure
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
///
/// Provided by [`khr_win32_surface`]
pub type VkCreateWin32SurfaceKhr = unsafe extern "system" fn(
    instance: VkInstance,
    create_info: *const VkWin32SurfaceCreateInfoKhr,
    allocator: *const VkAllocationCallbacks,
    surface: *mut VkSurfaceKhr,
) -> VkResult;

/// The name of [`VkCreateWin32SurfaceKhr`]
pub const VK_CREATE_WIN32_SURFACE_KHR: &CStr = c"vkCreateWin32SurfaceKHR";
