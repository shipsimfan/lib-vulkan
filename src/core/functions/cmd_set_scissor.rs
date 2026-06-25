use crate::{VkCommandBuffer, VkRect2D};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkCommandPool, VkDynamicState, VkPhysicalDeviceLimits,
    VkPipelineDynamicStateCreateInfo, VkPipelineViewportStateCreateInfo, VkQueueFlag,
};

/// Set scissor rectangles dynamically for a command buffer
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command will be recorded.
///  - `first_scissor` is the index of the first scissor whose state is updated by the command.
///  - `scissor_count` is the number of scissors whose rectangles are updated by the command.
///  - `scissors` is a pointer to an array of [`VkRect2D`] structures defining scissor rectangles.
///
/// # Description
/// The scissor rectangles taken from element `i` of `scissors` replace the current state for the
/// scissor index `first_scissor + i`, for `i` in `[0, scissor_count)`.
///
/// This command sets the scissor rectangles for subsequent drawing commands when drawing using
/// shader objects, or when the graphics pipeline is created with [`VkDynamicState::Scissor`] set
/// in [`VkPipelineDynamicStateCreateInfo::dynamic_states`]. Otherwise, this state is specified by
/// the [`VkPipelineViewportStateCreateInfo::scissors`] values used to create the currently active
/// pipeline.
///
/// # Valid Usage
///  - The sum of `first_scissor` and `scissor_count` must be between 1 and
///    [`VkPhysicalDeviceLimits::max_viewports`], inclusive
///  - If the `multi_viewport` feature is not enabled, `first_scissor` must be 0
///  - If the `multi_viewport` feature is not enabled, `scissor_count` must be 1
///  - The `x` and `y` members of `offset` member of any element of `scissors` must be greater than
///    or equal to 0
///  - Evaluation of `(offset.x + extent.width)` must not cause a signed integer addition overflow
///    for any element of `scissors`
///  - Evaluation of `(offset.y + extent.height)` must not cause a signed integer addition overflow
///    for any element of `scissors`
///  - If this command is recorded in a secondary command buffer with
///    [`VkCommandBufferInheritanceViewportScissorInfoNv::viewport_scissor_2d`] enabled, then this
///    function must not be called
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `scissors` must be a valid pointer to an array of `scissor_count` [`VkRect2D`] structures
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Graphics`] operations
///  - This command must only be called outside of a video coding scope
///  - `scissor_count` must be greater than 0
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCmdSetScissor = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    first_scissor: u32,
    scissor_count: u32,
    scissors: *const VkRect2D,
);

/// The name of [`VkCmdSetScissor`]
pub const VK_CMD_SET_SCISSOR: &CStr = c"vkCmdSetScissor";
