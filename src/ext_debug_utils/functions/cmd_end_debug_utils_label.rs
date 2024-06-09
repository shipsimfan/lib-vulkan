use crate::{VkCommandBuffer, VkDebugUtilsLabelEXT};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{ext_debug_utils, VkCmdBeginDebugUtilsLabelEXT};

/// Close a command buffer label region
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command is recorded.
///
/// # Description
/// An application may open a debug label region in one command buffer and close it in another, or
/// otherwise split debug label regions across multiple command buffers or multiple queue
/// submissions. When viewed from the linear series of submissions to a single queue, the calls to
/// [`VkCmdBeginDebugUtilsLabelEXT`] and [`VkCmdEndDebugUtilsLabelEXT`] must be matched and
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
/// Provided by [`ext_debug_utils`]
pub type VkCmdEndDebugUtilsLabelEXT =
    extern "system" fn(command_buffer: VkCommandBuffer, label_info: *const VkDebugUtilsLabelEXT);

/// The name of [`VkCmdEndDebugUtilsLabelEXT`]
pub const VK_CMD_END_DEBUG_UTILS_LABEL_EXT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkCmdEndDebugUtilsLabelEXT\0") };
