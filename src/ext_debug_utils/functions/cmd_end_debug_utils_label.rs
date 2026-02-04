use crate::{VkCommandBuffer, ext_debug_utils::VkDebugUtilsLabelExt};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VkQueueFlag,
    ext_debug_utils::{self, VkCmdBeginDebugUtilsLabelExt},
};

/// Close a command buffer label region
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command is recorded.
///
/// # Description
/// An application may open a debug label region in one command buffer and close it in another, or
/// otherwise split debug label regions across multiple command buffers or multiple queue
/// submissions. When viewed from the linear series of submissions to a single queue, the calls to
/// [`VkCmdBeginDebugUtilsLabelExt`] and [`VkCmdEndDebugUtilsLabelExt`] must be matched and
/// balanced.
///
/// There can be problems reporting command buffer debug labels during the recording process
/// because command buffers may be recorded out of sequence with the resulting execution order.
/// Since the recording order may be different, a solitary command buffer may have an inconsistent
/// view of the debug label regions by itself. Therefore, if an issue occurs during the recording
/// of a command buffer, and the environment requires returning debug labels, the implementation
/// may return only those labels it is aware of. This is true even if the implementation is aware
/// of only the debug labels within the command buffer being actively recorded.
///
/// # Valid Usage
///  - There must be an outstanding [`VkCmdBeginDebugUtilsLabelExt`] command prior to the
///    [`VkCmdEndDebugUtilsLabelExt`] on the queue that `command_buffer` is submitted to
///  - If `command_buffer` is a secondary command buffer, there must be an outstanding
///    [`VkCmdBeginDebugUtilsLabelExt`] command recorded to `command_buffer` that has not
///    previously been ended by a call to [`VkCmdEndDebugUtilsLabelExt`]
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::ComputeBit`], [`VkQueueFlag::GraphicsBit`],
///    [`VkQueueFlag::OpticalFlowBitNv`], [`VkQueueFlag::TransferBit`],
///    [`VkQueueFlag::VideoDecodeBitKhr`], or [`VkQueueFlag::VideoEncodeBitKhr`] operations
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`ext_debug_utils`]
pub type VkCmdEndDebugUtilsLabelExt =
    unsafe extern "system" fn(command_buffer: VkCommandBuffer, label_info: *const VkDebugUtilsLabelExt);

/// The name of [`VkCmdEndDebugUtilsLabelExt`]
pub const VK_CMD_END_DEBUG_UTILS_LABEL_EXT: &CStr = c"vkCmdEndDebugUtilsLabelEXT";
