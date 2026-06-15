use crate::{VkCommandBuffer, VkViewport};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkCommandPool, VkDynamicState, VkPhysicalDeviceLimits,
    VkPipelineDynamicStateCreateInfo, VkPipelineViewportStateCreateInfo, VkQueueFlag,
};

/// Set the viewport dynamically for a command buffer
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command will be recorded.
///  - `first_viewport` is the index of the first viewport whose parameters are updated by the
///    command.
///  - `viewport_count` is the number of viewports whose parameters are updated by the command.
///  - `viewports` is a pointer to an array of [`VkViewport`] structures specifying viewport
///    parameters.
///
/// # Description
/// This command sets the viewport transformation parameters state for subsequent drawing commands
/// when drawing using shader objects, or when the graphics pipeline is created with
/// [`VkDynamicState::Viewport`] set in [`VkPipelineDynamicStateCreateInfo::dynamic_states`].
/// Otherwise, this state is specified by the [`VkPipelineViewportStateCreateInfo::viewports`]
/// values used to create the currently active pipeline.
///
/// The viewport parameters taken from element i of `viewports` replace the current state for the
/// viewport index `first_viewport + i`, for `i` in `[0, viewport_count)`.
///
/// # Valid Usage
///  - The sum of `first_viewport` and `viewport_count` must be between 1 and
///    [`VkPhysicalDeviceLimits::max_viewports`], inclusive
///  - If the `multi_viewport` feature is not enabled, `first_viewport` must be 0
///  - If the `multi_viewport` feature is not enabled, `viewport_count` must be 1
///  - `command_buffer` must not have
///    [`VkCommandBufferInheritanceViewportScissorInfoNv::viewport_scissor_2d`] enabled
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `viewports` must be a valid pointer to an array of `viewport_count` valid [`VkViewport`]
///    structures
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Graphics`] operations
///  - This command must only be called outside of a video coding scope
///  - `viewport_count` must be greater than 0
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCmdSetViewport = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    viewports: *const VkViewport,
);

/// The name of [`VkCmdSetViewport`]
pub const VK_CMD_SET_VIEWPORT: &CStr = c"vkCmdSetViewport";
