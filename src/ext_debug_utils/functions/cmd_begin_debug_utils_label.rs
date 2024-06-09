use crate::{VkCommandBuffer, VkDebugUtilsLabelEXT};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Open a command buffer debug label region
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command is recorded.
///  - `label_info` is a pointer to a [`VkDebugUtilsLabelEXT`] structure specifying parameters of
///    the label region to open.
///
/// Provided by [`ext_debug_utils`]
pub type VkCmdBeginDebugUtilsLabelEXT =
    extern "system" fn(command_buffer: VkCommandBuffer, label_info: *const VkDebugUtilsLabelEXT);

/// The name of [`VkCmdBeginDebugUtilsLabelEXT`]
pub const VK_CMD_BEGIN_DEBUG_UTILS_LABEL_EXT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginDebugUtilsLabelEXT\0") };
