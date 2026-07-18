use crate::{VkBuffer, VkCommandBuffer, VkDeviceSize, VkIndexType};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_VERSION_1_0, VkBufferUsageFlag, VkCommandPool, VkDevice, VkDeviceMemory,
    VkQueueFlag,
};

/// Bind an index buffer to a command buffer
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command is recorded.
///  - `buffer` is the buffer being bound.
///  - `offset` is the starting offset in bytes within buffer used in index buffer address
///    calculations.
///  - `index_type` is a [`VkIndexType`] value specifying the size of the indices.
///
/// # Description
/// `buffer` and `offset` specify the bound index buffer range, with a range of memory bound from
/// `[base + offset, base + offset + size)`, where `size` is from `offset` to the end of the
/// buffer.
///
/// If the `maintenance6` feature is enabled, buffer can be [`VK_NULL_HANDLE`]. If `buffer` is
/// [`VK_NULL_HANDLE`] and the `null_descriptor` feature is enabled, every index fetched results in
/// a value of zero.
///
/// # Valid Usage
///  - `offset` must be less than the size of `buffer`
///  - The sum of `offset` and the base address of the range of [`VkDeviceMemory`] object that is
///    backing buffer, must be a multiple of the size of the type indicated by `index_type`
///  - `buffer` must have been created with the [`VkBufferUsageFlag::IndexBuffer`] usage flag set
///  - If `buffer` is non-sparse then it must be bound completely and contiguously to a single
///    [`VkDeviceMemory`] object
///  - `index_type` must not be [`VkIndexType::NoneKhr`]
///  - If `index_type` is [`VkIndexType::Uint8`], the `index_type_uint8` feature must be enabled
///  - If the `maintenance6` feature is not enabled, `buffer` must not be [`VK_NULL_HANDLE`]
///  - If `buffer` is [`VK_NULL_HANDLE`], `offset` must be zero
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - If `buffer` is not [`VK_NULL_HANDLE`], `buffer` must be a valid [`VkBuffer`] handle
///  - `index_type` must be a valid [`VkIndexType`] value
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Graphics`] operations
///  - This command must only be called outside of a video coding scope
///  - Both of `buffer`, and `command_buffer` that are valid handles of non-ignored parameters must
///    have been created, allocated, or retrieved from the same [`VkDevice`]
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCmdBindIndexBuffer = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    index_type: VkIndexType,
);

/// The name of [`VkCmdBindIndexBuffer`]
pub const VK_CMD_BIND_INDEX_BUFFER: &CStr = c"vkCmdBindIndexBuffer";
