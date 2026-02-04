use crate::VkQueue;
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils::{self, VkQueueBeginDebugUtilsLabelExt};

/// Close a queue debug label region
///
/// # Parameters
///  - `queue` is the queue in which a debug label region should be closed.
///
/// # Description
/// The calls to [`VkQueueBeginDebugUtilsLabelExt`] and [`VkQueueEndDebugUtilsLabelExt`] must be
/// matched and balanced.
///
/// # Valid Usage
///  - There must be an outstanding [`VkQueueBeginDebugUtilsLabelExt`] command prior to the
///    [`VkQueueEndDebugUtilsLabelExt`] on the queue
///
/// # Valid Usage (Implicit)
///  - `queue` must be a valid [`VkQueue`] handle
///
/// # Host Synchronization
///  - Host access to `queue` must be externally synchronized
///
/// Provided by [`ext_debug_utils`]
pub type VkQueueEndDebugUtilsLabelExt = unsafe extern "system" fn(queue: VkQueue);

/// The name of [`VkQueueEndDebugUtilsLabelExt`]
pub const VK_QUEUE_END_DEBUG_UTILS_LABEL_EXT: &CStr = c"vkQueueEndDebugUtilsLabelEXT";
