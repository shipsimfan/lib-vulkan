use crate::{VkAllocationCallbacks, VkInstance, VkSurfaceKHR};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_surface;

/// Destroy a [`VkSurfaceKHR`] object
///
/// # Parameters
///  - `instance` is the instance used to create the surface.
///  - `surface` is the surface to destroy.
///  - `allocator` is the allocator used for host memory allocated for the surface object when
///    there is no more specific allocator available.
///
/// # Description
/// Destroying a [`VkSurfaceKHR`] merely severs the connection between Vulkan and the native
/// surface, and does not imply destroying the native surface, closing a window, or similar
/// behavior.
///
/// Provided by [`khr_surface`]
pub type VkDestroySurfaceKHR = extern "system" fn(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroySurfaceKHR`]
pub const VK_DESTROY_SURFACE_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkDestroySurfaceKHR\0") };
