use crate::{VkAllocationCallbacks, VkDevice, VkImage};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0, VkImageView, khr_swapchain::VkGetSwapchainImagesKhr};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy an image object
///
/// # Parameters
///  - `device` is the logical device that destroys the image.
///  - `image` is the image to destroy.
///  - `allocator` controls host memory allocation.
///
/// # Valid Usage
///  - All submitted commands that refer to `image`, either directly or via a [`VkImageView`], must
///    have completed execution
///  - If [`VkAllocationCallbacks`] were provided when `image` was created, a compatible set of
///    callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when `image` was created, `allocator` must be
///    [`null`]
///  - `image` must not have been acquired from [`VkGetSwapchainImagesKhr`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `image` is not [`VK_NULL_HANDLE`], image must be a valid [`VkImage`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `image` is a valid handle, it must have been created, allocated, or retrieved from
///    `device`
///
/// # Host Synchronization
///  - Host access to image must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyImage = unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroyImage`]
pub const VK_DESTROY_IMAGE: &CStr = c"vkDestroyImage";
