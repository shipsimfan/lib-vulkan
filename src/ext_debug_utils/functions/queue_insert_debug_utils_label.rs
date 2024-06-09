use crate::{VkDebugUtilsLabelEXT, VkQueue};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Insert a label into a queue
///
/// # Parameters
///  - `queue` is the queue in which to start a debug label region.
///  - `label_info` is a pointer to a [`VkDebugUtilsLabelEXT`] structure specifying parameters of
///    the label to insert.
///
/// Provided by [`ext_debug_utils`]
pub type VkQueueInsertDebugUtilsLabelEXT =
    extern "system" fn(queue: VkQueue, label_info: *const VkDebugUtilsLabelEXT);

/// The name of [`VkQueueInsertDebugUtilsLabelEXT`]
pub const VK_QUEUE_INSERT_DEBUG_UTILS_LABEL_EXT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkQueueInsertDebugUtilsLabelEXT\0") };
