use crate::{VkCommandBuffer, VkCommandBufferAllocateInfo, VkDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Allocate command buffers from an existing command pool
///
/// # Parameters
///  - `device` is the logical device that owns the command pool.
///  - `allocate_info` is a pointer to a [`VkCommandBufferAllocateInfo`] structure describing
///    parameters of the allocation. `command_pool` may be accessed any time one of the resulting
///    command buffers is accessed.
///  - `command_buffers` is a pointer to an array of [`VkCommandBuffer`] handles in which the
///    resulting command buffer objects are returned. The array must be at least the length
///    specified by the `command_buffer_count` member of `allocate_info`. Each allocated command
///    buffer begins in the initial state.
///
/// # Description
/// [`VkAllocateCommandBuffers`] can be used to allocate multiple command buffers. If the
/// allocation of any of those command buffers fails, the implementation must free all successfully
/// allocated command buffer objects from this command, set all entries of the `command_buffers`
/// array to [`null_mut`] and return the error.
///
/// When command buffers are first allocated, they are in the initial state.
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `allocate_info` must be a valid pointer to a valid [`VkCommandBufferAllocateInfo`] structure
///  - `command_buffers` must be a valid pointer to an array of
///    `allocate_info.command_buffer_count` [`VkCommandBuffer`] handles
///  - The device must have been created with at least 1 queue
///  - `allocate_info.command_buffer_count` must be greater than 0
///
/// # Return Codes
///
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
pub type VkAllocateCommandBuffers = unsafe extern "system" fn(
    device: VkDevice,
    allocate_info: *const VkCommandBufferAllocateInfo,
    command_buffers: *mut VkCommandBuffer,
) -> VkResult;

/// The name of [`VkAllocateCommandBuffers`]
pub const VK_ALLOCATE_COMMAND_BUFFERS: &CStr = c"vkAllocateCommandBuffers";
