use crate::{VkQueue, ext_debug_utils::VkDebugUtilsLabelExt};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Open a queue debug label region
///
/// # Parameters
///  - `queue` is the queue in which to start a debug label region.
///  - `label_info` is a pointer to a [`VkDebugUtilsLabelExt`] structure specifying parameters of
///    the label region to open.
///
/// # Valid Usage (Implicit)
///  - `queue` must be a valid [`VkQueue`] handle
///  - `label_info` must be a pointer to a valid [`VkDebugUtilsLabelExt`] structure
///
/// # Host Synchronization
///  - Host access to `queue` must be externally synchronized
///
/// Provided by [`ext_debug_utils`]
pub type VkQueueBeginDebugUtilsLabelExt =
    unsafe extern "system" fn(queue: VkQueue, label_info: *const VkDebugUtilsLabelExt);

/// The name of [`VkQueueBeginDebugUtilsLabelExt`]
pub const VK_QUEUE_BEGIN_DEBUG_UTILS_LABEL_EXT: &CStr = c"vkQueueBeginDebugUtilsLabelEXT";
