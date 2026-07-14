use crate::{VkAllocationCallbacks, VkBuffer, VkDevice};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0, VkBufferView};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy a buffer object
///
/// # Parameters
///  - `device` is the logical device that destroys the buffer.
///  - `buffer` is the buffer to destroy.
///  - `allocator` controls host memory allocation.
///
/// # Valid Usage
///  - All submitted commands that refer to buffer, either directly or via a [`VkBufferView`], must
///    have completed execution
///  - If [`VkAllocationCallbacks`] were provided when buffer was created, a compatible set of
///    callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when buffer was created, `allocator` must be
///    [`null`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `buffer` is not [`VK_NULL_HANDLE`], `buffer` must be a valid [`VkBuffer`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `buffer` is a valid handle, it must have been created, allocated, or retrieved from
///    device
///
/// # Host Synchronization
///  - Host access to `buffer` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyBuffer = unsafe extern "system" fn(
    device: VkDevice,
    buffer: VkBuffer,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroyBuffer`]
pub const VK_DESTROY_BUFFER: &CStr = c"vkDestroyBuffer";
