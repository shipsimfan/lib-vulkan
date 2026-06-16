use crate::VkCommandBuffer;
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkCommandPool, VkQueueFlag};

/// Draw primitives
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command is recorded.
///  - `vertex_count` is the number of vertices to draw.
///  - `instance_count` is the number of instances to draw.
///  - `first_vertex` is the index of the first vertex to draw.
///  - `first_instance` is the instance ID of the first instance to draw.
///
/// # Description
/// When the command is executed, primitives are assembled using the current primitive topology and
/// `vertex_count` consecutive vertex indices with the first vertexIndex value equal to
/// `first_vertex`. The primitives are drawn `instance_count` times with `instance_index` starting
/// with `first_instance` and increasing sequentially for each instance. The assembled primitives
/// execute the bound graphics pipeline.
///
/// TODO: Add valid usage
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
pub type VkCmdDraw = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
);

/// The name of [`VkCmdDraw`]
pub const VK_CMD_DRAW: &CStr = c"vkCmdDraw";
