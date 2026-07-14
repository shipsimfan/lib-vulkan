use crate::{VkBuffer, VkCommandBuffer, VkDeviceSize};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_VERSION_1_0, VkBufferUsageFlag, VkCommandPool, VkDevice, VkDeviceMemory,
    VkPhysicalDeviceLimits, VkQueueFlag,
};

/// Bind vertex buffers to a command buffer
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command is recorded.
///  - `first_binding` is the index of the first vertex input binding whose state is updated by the
///    command.
///  - `binding_count` is the number of vertex input bindings whose state is updated by the
///    command.
///  - `buffers` is a pointer to an array of buffer handles.
///  - `offsets` is a pointer to an array of buffer offsets.
///
/// # Description
/// The values taken from elements `i` of `buffers` and `offsets` replace the current state for the
/// vertex input binding `first_binding + i`, for `i` in `[0, binding_count)`. The vertex input
/// binding is updated to start at the offset indicated by `offsets[i]` from the start of the
/// buffer `buffers[i]`. All vertex input attributes that use each of these bindings will use these
/// updated addresses in their address calculations for subsequent drawing commands. If the
/// `null_descriptor` feature is enabled, elements of `buffers` can be [`VK_NULL_HANDLE`], and can
/// be used by the vertex shader. If a vertex input attribute is bound to a vertex input binding
/// that is [`VK_NULL_HANDLE`], the values taken from memory are considered to be zero, and missing
/// G, B, or A components are filled with 0.
///
/// # Valid Usage
///  - `first_binding` must be less than [`VkPhysicalDeviceLimits::max_vertex_input_bindings`]
///  - The sum of `first_binding` and `binding_count` must be less than or equal to
///    [`VkPhysicalDeviceLimits::max_vertex_input_bindings`]
///  - All elements of `offsets` must be less than the size of the corresponding element in
///    `buffers`
///  - All elements of `buffers` must have been created with the
///    [`VkBufferUsageFlag::VertexBuffer`] flag
///  - Each element of `buffers` that is non-sparse must be bound completely and contiguously to a
///    single [`VkDeviceMemory`] object
///  - If the `null_descriptor` feature is not enabled, all elements of `buffers` must not be
///    [`VK_NULL_HANDLE`]
///  - If an element of `buffers` is [`VK_NULL_HANDLE`], then the corresponding element of
///    `offsets` must be zero
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `buffers` must be a valid pointer to an array of `binding_count` valid or [`VK_NULL_HANDLE`]
///    [`VkBuffer`] handles
///  - `offsets` must be a valid pointer to an array of `binding_count` [`VkDeviceSize`] values
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Graphics`] operations
///  - This command must only be called outside of a video coding scope
///  - `binding_count` must be greater than 0
///  - Both of `command_buffer`, and the elements of `buffers` that are valid handles of
///    non-ignored parameters must have been created, allocated, or retrieved from the same
///    [`VkDevice`]
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCmdBindVertexBuffers = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    first_binding: u32,
    binding_count: u32,
    buffers: *const VkBuffer,
    offsets: *const VkDeviceSize,
);

/// The name of [`VkCmdBindVertexBuffers`]
pub const VK_CMD_BIND_VERTEX_BUFFERS: &CStr = c"vkCmdBindVertexBuffers";
