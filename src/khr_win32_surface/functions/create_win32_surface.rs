use crate::{
    VkAllocationCallbacks, VkInstance, VkResult, VkSurfaceKHR, VkWin32SurfaceCreateInfoKHR,
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_win32_surface;

/// Create a [`VkSurfaceKHR`] object for an Win32 native window
///
/// # Parameters
///  - `instance` is the instance to associate the surface with.
///  - `create_info` is a pointer to a [`VkWin32SurfaceCreateInfoKHR`] structure containing
///    parameters affecting the creation of the surface object.
///  - `allocator` is the allocator used for host memory allocated for the surface object when
///    there is no more specific allocator available.
///  - `surface` is a pointer to a [`VkSurfaceKHR`] handle in which the created surface object is
///    returned.
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
pub type VkCreateWin32SurfaceKHR = extern "system" fn(
    instance: VkInstance,
    create_info: *const VkWin32SurfaceCreateInfoKHR,
    allocator: *const VkAllocationCallbacks,
    surface: *mut VkSurfaceKHR,
) -> VkResult;

/// The name of [`VkCreateWin32SurfaceKHR`]
pub const VK_CREATE_WIN32_SURFACE_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkCreateWin32SurfaceKHR\0") };
