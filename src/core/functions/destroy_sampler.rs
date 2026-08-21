use crate::{VkAllocationCallbacks, VkDevice, VkSampler};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy a sampler object
///
/// # Parameters
///  - `device` is the logical device that destroys the sampler.
///  - `sampler` is the sampler to destroy.
///  - `allocator` controls host memory allocation.
///
/// # Valid Usage
///  - All submitted commands that refer to `sampler` must have completed execution
///  - If [`VkAllocationCallbacks`] were provided when `sampler` was created, a compatible set of
///    callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when sampler was created, `allocator` must be
///    [`null`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `sampler` is not [`VK_NULL_HANDLE`], `sampler` must be a valid [`VkSampler`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `sampler` is a valid handle, it must have been created, allocated, or retrieved from
///    `device`
///
/// # Host Synchronization
///  - Host access to `sampler` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroySampler = unsafe extern "system" fn(
    device: VkDevice,
    sampler: VkSampler,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroySampler`]
pub const VK_DESTROY_SAMPLER: &CStr = c"vkDestroySampler";
