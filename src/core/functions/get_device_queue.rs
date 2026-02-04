use crate::{VkDevice, VkQueue, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkDeviceQueueCreateInfo};

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
/// # Valid Usage
///  - `queue_family_index` must be one of the queue family indices specified when device was
///    created, via the [`VkDeviceQueueCreateInfo`] structure
///  - `queue_index` must be less than the value of [`VkDeviceQueueCreateInfo::queue_count`] for
///    the queue family indicated by `queue_family_index` when device was created
///  - [`VkDeviceQueueCreateInfo::flags`] must have been zero when device was created
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `queue` must be a valid pointer to a [`VkQueue`] handle
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkGetDeviceQueue = unsafe extern "system" fn(
    device: VkDevice,
    queue_family_index: u32,
    queue_index: u32,
    queue: *mut VkQueue,
) -> VkResult;

/// The name of [`VkGetDeviceQueue`]
pub const VK_GET_DEVICE_QUEUE: &CStr = c"vkGetDeviceQueue";
