use crate::{VkAllocationCallbacks, VkDevice, VkPipelineLayout};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy a pipeline layout object
///
/// # Parameters
///  - `device` is the logical device that destroys the pipeline layout.
///  - `pipeline_layout` is the pipeline layout to destroy.
///  - `allocator` controls host memory allocation as described in the Memory Allocation chapter.
///
/// # Valid Usage
///  - If [`VkAllocationCallbacks`] were provided when `pipeline_layout` was created, a compatible
///    set of callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when `pipeline_layout` was created,
///    `allocator` must be [`null`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `pipeline_layout` is not [`VK_NULL_HANDLE`], `pipeline_layout` must be a valid
///    [`VkPipelineLayout`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `pipeline_layout` is a valid handle, it must have been created, allocated, or retrieved
///    from `device`
///
/// # Host Synchronization
///  - Host access to `pipeline_layout` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyPipelineLayout = unsafe extern "system" fn(
    device: VkDevice,
    pipeline_layout: VkPipelineLayout,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroyPipelineLayout`]
pub const VK_DESTROY_PIPELINE_LAYOUT: &CStr = c"vkDestroyPipelineLayout";
