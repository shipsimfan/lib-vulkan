use crate::{VkDebugUtilsLabelEXT, VkQueue};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Open a queue debug label region
///
/// # Parameters
///  - `queue` is the queue in which to start a debug label region.
///  - `label_info` is a pointer to a [`VkDebugUtilsLabelEXT`] structure specifying parameters of
///    the label region to open.
///
/// Provided by [`ext_debug_utils`]
pub type VkQueueBeginDebugUtilsLabelEXT =
    extern "system" fn(queue: VkQueue, label_info: *const VkDebugUtilsLabelEXT);

/// The name of [`VkQueueBeginDebugUtilsLabelEXT`]
pub const VK_QUEUE_BEGIN_DEBUG_UTILS_LABEL_EXT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkQueueBeginDebugUtilsLabelEXT\0") };
