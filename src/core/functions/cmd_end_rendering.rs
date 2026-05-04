use crate::VkCommandBuffer;
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkCmdBeginRendering, VkCommandPool, VkQueueFlag, VkRenderingFlag};

/// End a dynamic render pass instance
///
/// # Parameters
///  - `command_buffer` is the command buffer in which to record the command.
///
/// # Description
/// If the value of `rendering_info.flags` used to begin this render pass instance included
/// [`VkRenderingFlag::SuspendingBit`], then this render pass is suspended and will be resumed
/// later in submission order.
///
/// # Valid Usage
///  - The current render pass instance must have been begun with [`VkCmdBeginRendering`]
///  - The current render pass instance must have been begun in `command_buffer`
///  - This command must not be recorded when transform feedback is active
///  - If `VkCmdBeginQuery*` was called within the render pass, the corresponding `VkCmdEndQuery*`
///    must have been called subsequently within the same subpass
///  - This command must not be recorded when per-tile execution model is enabled
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::GraphicsBit`] operations
///  - This command must only be called inside of a render pass instance
///  - This command must not be called between suspended render pass instances
///  - This command must only be called outside of a video coding scope
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_3`]
pub type VkCmdEndRendering = unsafe extern "system" fn(command_buffer: VkCommandBuffer);

/// The name of [`VkCmdEndRendering`]
pub const VK_CMD_END_RENDERING: &CStr = c"vkCmdEndRendering";
