use crate::{VkCommandBuffer, VkDebugUtilsLabelEXT};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Insert a label into a command buffer
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command is recorded.
///  - `label_info` is a pointer to a [`VkDebugUtilsLabelEXT`] structure specifying parameters of
///    the label to insert.
///
/// Provided by [`ext_debug_utils`]
pub type VkCmdInsertDebugUtilsLabelEXT =
    extern "system" fn(command_buffer: VkCommandBuffer, label_info: *const VkDebugUtilsLabelEXT);

/// The name of [`VkCmdInsertDebugUtilsLabelEXT`]
pub const VK_CMD_INSERT_DEBUG_UTILS_LABEL_EXT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkCmdInsertDebugUtilsLabelEXT\0") };
