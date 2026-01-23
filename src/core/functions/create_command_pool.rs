use crate::{VkAllocationCallbacks, VkCommandPool, VkCommandPoolCreateInfo, VkDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::null;

/// Create a new command pool object
///
/// # Parameters
///  - `device` is the logical device that creates the command pool.
///  - `create_info` is a pointer to a [`VkCommandPoolCreateInfo`] structure specifying the state
///    of the command pool object.
///  - `allocator` controls host memory allocation.
///  - `command_pool` is a pointer to a [`VkCommandPool`] handle in which the created pool is
///    returned.
///
/// # Valid Usage
///  - `create_info.queue_family_index` must be the index of a queue family available in the
///    logical device `device`
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `create_info` must be a valid pointer to a valid [`VkCommandPoolCreateInfo`] structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `command_pool` must be a valid pointer to a [`VkCommandPool`] handle
///  - The device must have been created with at least 1 queue
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCreateCommandPool = extern "system" fn(
    device: VkDevice,
    create_info: *const VkCommandPoolCreateInfo,
    allocator: *const VkAllocationCallbacks,
    command_pool: *mut VkCommandPool,
) -> VkResult;

/// The name of [`VkCreateCommandPool`]
pub const VK_CREATE_COMMAND_POOL: &CStr = c"vkCreateCommandPool";
