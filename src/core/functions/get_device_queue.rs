use crate::{VkDevice, VkQueue, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VkDeviceQueueCreateInfo, VK_VERSION_1_0};

/// Get a queue handle from a device
///
/// # Parameters
///  - `device` is the logical device that owns the queue.
///  - `queue_family_index` is the index of the queue family to which the queue belongs.
///  - `queue_index` is the index within this queue family of the queue to retrieve.
///  - `p_queue` is a pointer to a [`VkQueue`] object that will be filled with the handle for the
///    requested queue.
///
/// # Description
/// [`VkGetDeviceQueue`] must only be used to get queues that were created with the `flags`
/// parameter of [`VkDeviceQueueCreateInfo`] set to zero. To get queues that were created with a
/// non-zero `flags` parameter use [`VkGetDeviceQueue2`].
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkGetDeviceQueue = extern "system" fn(
    device: VkDevice,
    queue_family_index: u32,
    queue_index: u32,
    queue: *mut VkQueue,
) -> VkResult;

/// The name of [`VkGetDeviceQueue`]
pub const VK_GET_DEVICE_QUEUE: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceQueue\0") };
