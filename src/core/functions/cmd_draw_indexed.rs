use crate::VkCommandBuffer;
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkCmdBindIndexBuffer, VkCommandPool, VkIndexType, VkQueueFlag};

/// Draw primitives with indexed vertices
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command is recorded.
///  - `index_count` is the number of vertices to draw.
///  - `instance_count` is the number of instances to draw.
///  - `first_index` is the base index within the index buffer.
///  - `vertex_offset` is the value added to the vertex index before indexing into the vertex
///    buffer.
///  - `first_instance` is the instance ID of the first instance to draw.
///
/// # Description
/// When the command is executed, primitives are assembled using the current primitive topology and
/// `index_count` vertices whose indices are retrieved from the index buffer. The index buffer is
/// treated as an array of tightly packed unsigned integers of size defined by the
/// `VkCmdBindIndexBuffer2::index_type` or the `VkCmdBindIndexBuffer::index_type` parameter with
/// which the buffer was bound.
///
/// The first vertex index is read from `index_address + (first_index × indexSize)`, where
/// `index_address` is the effective address specified for the index buffer by
/// [`VkCmdBindIndexBuffer3Khr`], [`VkCmdBindIndexBuffer2`], or [`VkCmdBindIndexBuffer`], and
/// `index_size` is the byte size of the type specified by `index_type`. Subsequent index values
/// are retrieved from consecutive locations in the index buffer. Indices are first compared to the
/// primitive restart value, then zero extended to 32 bits (if the `index_type` is
/// [`VkIndexType::Uint8`] or [`VkIndexType::Uint16`]) and have `vertex_offset` added to them,
/// before being supplied as the `vertex_index` value.
///
/// The primitives are drawn `instance_count` times with `instance_index` starting with
/// `first_instance` and increasing sequentially for each instance. The assembled primitives
/// execute the bound graphics pipeline.
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Graphics`] operations
///  - This command must only be called inside of a render pass instance
///  - This command must not be called between suspended render pass instances
///  - This command must only be called outside of a video coding scope
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCmdDrawIndexed = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
);

/// The name of [`VkCmdDrawIndexed`]
pub const VK_CMD_DRAW_INDEXED: &CStr = c"vkCmdDrawIndexed";
