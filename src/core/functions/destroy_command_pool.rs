use crate::{VkAllocationCallbacks, VkCommandPool, VkDevice};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0, VkCommandBuffer};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy a command pool object
///
/// # Parameters
///  - `device` is the logical device that destroys the command pool.
///  - `command_pool` is the handle of the command pool to destroy.
///  - `allocator` controls host memory allocation.
///
/// # Description
/// When a pool is destroyed, all command buffers allocated from the pool are freed.
///
/// Any primary command buffer allocated from another [`VkCommandPool`] that is in the recording or
/// executable state and has a secondary command buffer allocated from `command_pool` recorded into
/// it, becomes invalid.
///
/// # Valid Usage
///  - All [`VkCommandBuffer`] objects allocated from `command_pool` must not be in the pending
///    state
///  - If [`VkAllocationCallbacks`] were provided when `command_pool` was created, a compatible set
///    of callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when `command_pool` was created, `allocator`
///    must be [`null`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `command_pool` is not [`VK_NULL_HANDLE`], `command_pool` must be a valid
///    [`VkCommandPool`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `command_pool` is a valid handle, it must have been created, allocated, or retrieved from
///    device
///
/// # Host Synchronization
///  - Host access to `command_pool` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyCommandPool = extern "system" fn(
    device: VkDevice,
    command_pool: VkCommandPool,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroyCommandPool`]
pub const VK_DESTROY_COMMAND_POOL: &CStr = c"vkDestroyCommandPool";
