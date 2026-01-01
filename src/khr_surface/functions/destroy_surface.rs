use crate::{VkAllocationCallbacks, VkInstance, khr_surface::VkSurfaceKhr};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, khr_surface};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy a [`VkSurfaceKhr`] object
///
/// # Parameters
///  - `instance` is the instance used to create the surface.
///  - `surface` is the surface to destroy.
///  - `allocator` is the allocator used for host memory allocated for the surface object when
///    there is no more specific allocator available.
///
/// # Description
/// Destroying a [`VkSurfaceKhr`] merely severs the connection between Vulkan and the native
/// surface, and does not imply destroying the native surface, closing a window, or similar
/// behavior.
///
/// # Valid Usage
///  - All [`VkSwapchainKhr`] objects created for surface must have been destroyed prior to
///    destroying surface
///  - If [`VkAllocationCallbacks`] were provided when surface was created, a compatible set of
///    callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when surface was created, `allocator` must be
///    [`null`]
///
/// # Valid Usage (Implicit)
///  - `instance` must be a valid [`VkInstance`] handle
///  - If `surface` is not [`VK_NULL_HANDLE`], `surface` must be a valid [`VkSurfaceKhr`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `surface` is a valid handle, it must have been created, allocated, or retrieved from
///    `instance`
///
/// # Host Synchronization
///  - Host access to `surface` must be externally synchronized
///
/// Provided by [`khr_surface`]
pub type VkDestroySurfaceKhr = extern "system" fn(
    instance: VkInstance,
    surface: VkSurfaceKhr,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroySurfaceKhr`]
pub const VK_DESTROY_SURFACE_KHR: &CStr = c"vkDestroySurfaceKHR";
