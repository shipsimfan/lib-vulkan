use crate::{VkAllocationCallbacks, VkDevice, VkPipeline};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy a pipeline object
///
/// # Parameters
///  - `device` is the logical device that destroys the pipeline.
///  - `pipeline` is the handle of the pipeline to destroy.
///  - `allocator` controls host memory allocation as described in the Memory Allocation chapter.
///
/// # Valid Usage
///  - All submitted commands that refer to `pipeline` must have completed execution
///  - If [`VkAllocationCallbacks`] were provided when `pipeline` was created, a compatible set of
///    callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when `pipeline` was created, `allocator` must
///    be [`null`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `pipeline` is not [`VK_NULL_HANDLE`], `pipeline` must be a valid [`VkPipeline`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `pipeline` is a valid handle, it must have been created, allocated, or retrieved from
///    `device`
///
/// # Host Synchronization
///  - Host access to `pipeline` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyPipeline = unsafe extern "system" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroyPipeline`]
pub const VK_DESTROY_PIPELINE: &CStr = c"vkDestroyPipeline";
