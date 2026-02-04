use crate::{VkAllocationCallbacks, VkDevice, khr_swapchain::VkSwapchainKhr};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, khr_surface::VkSurfaceKhr, khr_swapchain};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy a swapchain object
///
/// # Parameters
///  - `device` is the [`VkDevice`] associated with `swapchain`
///  - `swapchain` is the swapchain to destroy.
///  - `allocator` is the allocator used for host memory allocated for the swapchain object when
///    there is no more specific allocator available.
///
/// # Description
/// The application must not destroy a swapchain until after completion of all outstanding
/// operations on images that were acquired from the swapchain. `swapchain` and all associated
/// VkImage handles are destroyed, and must not be acquired or used any more by the application.
/// The memory of each VkImage will only be freed after that image is no longer used by the
/// presentation engine. For example, if one image of the swapchain is being displayed in a window,
/// the memory for that image may not be freed until the window is destroyed, or another swapchain
/// is created for the window. Destroying the swapchain does not invalidate the parent
/// [`VkSurfaceKhr`], and a new swapchain can be created with it.
///
/// When a swapchain associated with a display surface is destroyed, if the image most recently
/// presented to the display surface is from the swapchain being destroyed, then either any display
/// resources modified by presenting images from any swapchain associated with the display surface
/// must be reverted by the implementation to their state prior to the first present performed on
/// one of these swapchains, or such resources must be left in their current state.
///
/// If swapchain has exclusive full-screen access, it is released before the swapchain is
/// destroyed.
///
/// # Valid Usage
///  - All uses of presentable images acquired from `swapchain` must have completed execution
///  - If [`VkAllocationCallbacks`] were provided when `swapchain` was created, a compatible set of
///    callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when swapchain was created, `allocator` must
///    be [`null`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `swapchain` is not [`VK_NULL_HANDLE`], `swapchain` must be a valid [`VkSwapchainKhr`]
///    handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `swapchain` is a valid handle, it must have been created, allocated, or retrieved from
///    `device`
///
/// # Host Synchronization
///  - Host access to `swapchain` must be externally synchronized
///
/// Provided by [`khr_swapchain`]
pub type VkDestroySwapchainKhr = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKhr,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroySwapchainKhr`]
pub const VK_DESTROY_SWAPCHAIN_KHR: &CStr = c"vkDestroySwapchainKHR";
