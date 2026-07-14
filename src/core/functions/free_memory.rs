use crate::{VkAllocationCallbacks, VkDevice, VkDeviceMemory};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null;

/// Free device memory
///
/// # Parameters
///  - `device` is the logical device that owns the memory.
///  - `memory` is the [`VkDeviceMemory`] object to be freed.
///  - `allocator` controls host memory allocation.
///
/// # Description
/// Before freeing a memory object, an application must ensure the memory object is no longer in
/// use by the device — for example by command buffers in the pending state. Memory can be freed
/// whilst still bound to resources, but those resources must not be used afterwards. Freeing a
/// memory object releases the reference it held, if any, to its payload. If there are still any
/// bound images or buffers, the memory object’s payload may not be immediately released by the
/// implementation, but must be released by the time all bound images and buffers have been
/// destroyed. Once all references to a payload are released, it is returned to the heap from which
/// it was allocated.
///
/// If a memory object is mapped at the time it is freed, it is implicitly unmapped.
///
/// # Valid Usage
///  - All submitted commands that refer to `memory` (via images or buffers) must have completed
///    execution
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `memory` is not [`VK_NULL_HANDLE`], memory must be a valid [`VkDeviceMemory`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `memory` is a valid handle, it must have been created, allocated, or retrieved from
///    `device`
///
/// # Host Synchronization
///  - Host access to `memory` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkFreeMemory = unsafe extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkFreeMemory`]
pub const VK_FREE_MEMORY: &CStr = c"vkFreeMemory";
