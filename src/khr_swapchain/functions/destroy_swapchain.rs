use crate::{VkAllocationCallbacks, VkDevice, VkSwapchainKHR};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VkSurfaceKHR};

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
/// [`VkSurfaceKHR`], and a new swapchain can be created with it.
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
/// Provided by [`khr_swapchain`]
pub type VkDestroySwapchainKHR = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroySwapchainKHR`]
pub const VK_DESTROY_SWAPCHAIN_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkDestroySwapchainKHR\0") };
