use crate::{VkAllocationCallbacks, VkDevice, VkSemaphore};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy a semaphore object
///
/// # Parameters
///  - `device` is the logical device that destroys the semaphore.
///  - `semaphore` is the handle of the semaphore to destroy.
///  - `allocator` controls host memory allocation.
///
/// # Valid Usage
///  - All submitted batches that refer to semaphore must have completed execution
///  - If [`VkAllocationCallbacks`] were provided when semaphore was created, a compatible set of
///    callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when semaphore was created, `allocator` must
///    be [`null`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `semaphore` is not [`VK_NULL_HANDLE`], `semaphore` must be a valid [`VkSemaphore`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `semaphore` is a valid handle, it must have been created, allocated, or retrieved from
///    `device`
///
/// # Host Synchronization
///  - Host access to semaphore must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroySemaphore = unsafe extern "system" fn(
    device: VkDevice,
    semaphore: VkSemaphore,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroySemaphore`]
pub const VK_DESTROY_SEMAPHORE: &CStr = c"vkDestroySemaphore";
