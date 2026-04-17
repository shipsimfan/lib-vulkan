use crate::{VkAllocationCallbacks, VkDevice, VkImageView};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy an image view object
///
/// # Parameters
///  - `device` is the logical device that destroys the image view.
///  - `image_view` is the image view to destroy.
///  - `allocator` controls host memory allocation.
///
/// # Valid Usage
///  - All submitted commands that refer to `image_view` must have completed execution
///  - If [`VkAllocationCallbacks`] were provided when `image_view` was created, a compatible set
///    of callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when `image_view` was created, `allocator`
///    must be [`null`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `image_view` is not [`VK_NULL_HANDLE`], `image_view` must be a valid [`VkImageView`]
///    handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `image_view` is a valid handle, it must have been created, allocated, or retrieved from
///    `device`
///
/// # Host Synchronization
///  - Host access to `image_view` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyImageView = unsafe extern "system" fn(
    device: VkDevice,
    image_view: VkImageView,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroyImageView`]
pub const VK_DESTROY_IMAGE_VIEW: &CStr = c"vkDestroyImageView";
