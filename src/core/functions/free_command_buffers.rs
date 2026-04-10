use crate::{VkCommandBuffer, VkCommandPool, VkDevice};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::null;

/// Free command buffers
///
/// # Parameters
///  - `device` is the logical device that owns the command pool.
///  - `command_pool` is the command pool from which the command buffers were allocated.
///  - `command_buffer_count` is the length of the `command_buffers` array.
///  - `command_buffers` is a pointer to an array of handles of command buffers to free.
///
/// # Description
/// Any primary command buffer that is in the recording or executable state and has any element of
/// `command_buffers` recorded into it, becomes invalid.
///
/// # Valid Usage
///  - All elements of `command_buffers` must not be in the pending state
///  - `command_buffers` must be a valid pointer to an array of `command_buffer_count`
///    [`VkCommandBuffer`] handles, each element of which must either be a valid handle or [`null`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `command_pool` must be a valid [`VkCommandPool`] handle
///  - `command_buffer_count` must be greater than 0
///  - `command_pool` must have been created, allocated, or retrieved from `device`
///  - Each element of `command_buffers` that is a valid handle must have been created, allocated,
///    or retrieved from `command_pool`
///
/// # Host Synchronization
///  - Host access to `command_pool` must be externally synchronized
///  - Host access to each member of `command_buffers` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkFreeCommandBuffers = unsafe extern "system" fn(
    device: VkDevice,
    command_pool: VkCommandPool,
    command_buffer_count: u32,
    command_buffers: *const VkCommandBuffer,
);

/// The name of [`VkFreeCommandBuffers`]
pub const VK_FREE_COMMAND_BUFFERS: &CStr = c"vkFreeCommandBuffers";
