use crate::{VkAllocationCallbacks, VkDevice, VkFence};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy a fence object
///
/// # Parameters
///  - `device` is the logical device that destroys the fence.
///  - `fence` is the handle of the fence to destroy.
///  - `allocator` controls host memory allocation.
///
/// # Valid Usage
///  - All queue submission commands that refer to fence must have completed execution
///  - If [`VkAllocationCallbacks`] were provided when fence was created, a compatible set of
///    callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when fence was created, `allocator` must be
///    [`null`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `fence` is not [`VK_NULL_HANDLE`], fence must be a valid [`VkFence`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `fence` is a valid handle, it must have been created, allocated, or retrieved from \
///    `device`
///
/// # Host Synchronization
///  - Host access to fence must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyFence = unsafe extern "system" fn(
    device: VkDevice,
    fence: VkFence,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroyFence`]
pub const VK_DESTROY_FENCE: &CStr = c"vkDestroyFence";
