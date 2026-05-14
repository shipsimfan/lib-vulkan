use crate::{VkCommandBuffer, ext_debug_utils::VkDebugUtilsLabelExt};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VkQueueFlag, ext_debug_utils};

/// Insert a label into a command buffer
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command is recorded.
///  - `label_info` is a pointer to a [`VkDebugUtilsLabelExt`] structure specifying parameters of
///    the label to insert.
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `label_info` must be a valid pointer to a valid [`VkDebugUtilsLabelExt`] structure
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Compute`], [`VkQueueFlag::Graphics`],
///    [`VkQueueFlag::OpticalFlowNv`], [`VkQueueFlag::Transfer`],
///    [`VkQueueFlag::VideoDecodeKhr`], or [`VkQueueFlag::VideoEncodeKhr`] operations
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`ext_debug_utils`]
pub type VkCmdInsertDebugUtilsLabelExt = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    label_info: *const VkDebugUtilsLabelExt,
);

/// The name of [`VkCmdInsertDebugUtilsLabelExt`]
pub const VK_CMD_INSERT_DEBUG_UTILS_LABEL_EXT: &CStr = c"vkCmdInsertDebugUtilsLabelEXT";
