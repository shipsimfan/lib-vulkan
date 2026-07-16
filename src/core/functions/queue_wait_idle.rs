use crate::{VkQueue, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_0, VkDeviceQueueCreateFlag, VkWaitForFences};

/// Wait for a queue to become idle
///
/// # Parameters
///  - `queue` is the queue on which to wait.
///
/// # Description
/// [`VkQueueWaitIdle`] is equivalent to having submitted a valid fence to every previously
/// executed queue submission command that accepts a fence, then waiting for all of those fences to
/// signal using [`VkWaitForFences`] with an infinite `timeout` and `wait_all` set to [`VK_TRUE`].
///
/// # Valid Usage (Implicit)
///  - `queue` must be a valid [`VkQueue`] handle
///
/// # Host Synchronization
///  - Host access to `queue` must be externally synchronized if it was not created with
///    [`VkDeviceQueueCreateFlag::InternallySynchronizedKhr`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkQueueWaitIdle = unsafe extern "system" fn(queue: VkQueue) -> VkResult;

/// The name of [`VkQueueWaitIdle`]
pub const VK_QUEUE_WAIT_IDLE: &CStr = c"vkQueueWaitIdle";
