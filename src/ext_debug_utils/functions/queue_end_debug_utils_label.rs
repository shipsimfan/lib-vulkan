use crate::VkQueue;
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{ext_debug_utils, VkQueueBeginDebugUtilsLabelEXT};

/// Close a queue debug label region
///
/// # Parameters
///  - `queue` is the queue in which a debug label region should be closed.
///
/// # Description
/// The calls to [`VkQueueBeginDebugUtilsLabelEXT`] and [`VkQueueEndDebugUtilsLabelEXT`] must be
/// matched and balanced.
///
/// Provided by [`ext_debug_utils`]
pub type VkQueueEndDebugUtilsLabelEXT = extern "system" fn(queue: VkQueue);

/// The name of [`VkQueueEndDebugUtilsLabelEXT`]
pub const VK_QUEUE_END_DEBUG_UTILS_LABEL_EXT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkQueueEndDebugUtilsLabelEXT\0") };
