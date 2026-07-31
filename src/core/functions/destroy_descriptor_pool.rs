use crate::{VkAllocationCallbacks, VkDescriptorPool, VkDevice};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy a descriptor pool object
///
/// # Parameters
///  - `device` is the logical device that destroys the descriptor pool.
///  - `descriptor_pool` is the descriptor pool to destroy.
///  - `allocator` controls host memory allocation.
///
/// # Description
/// When a pool is destroyed, all descriptor sets allocated from the pool are implicitly freed and
/// become invalid. Descriptor sets allocated from a given pool do not need to be freed before
/// destroying that descriptor pool.
///
/// # Valid Usage
///  - All submitted commands that refer to `descriptor_pool` (via any allocated descriptor sets)
///    must have completed execution
///  - If [`VkAllocationCallbacks`] were provided when `descriptor_pool` was created, a compatible
///    set of callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when `descriptor_pool` was created,
///    `allocator` must be [`null`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `descriptor_pool` is not [`VK_NULL_HANDLE`], `descriptor_pool` must be a valid
///    [`VkDescriptorPool`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `descriptor_pool` is a valid handle, it must have been created, allocated, or retrieved
///    from `device`
///
/// # Host Synchronization
///  - Host access to `descriptor_pool` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyDescriptorPool = unsafe extern "system" fn(
    device: VkDevice,
    descriptor_pool: VkDescriptorPool,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroyDescriptorPool`]
pub const VK_DESTROY_DESCRIPTOR_POOL: &CStr = c"vkDestroyDescriptorPool";
