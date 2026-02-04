use crate::{VkDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkQueue};

/// Wait for a device to become idle
///
/// # Parameters
///  - `device` is the logical device to idle.
///
/// # Description
/// [`VkDeviceWaitIdle`] is equivalent to calling [`VkQueueWaitIdle`] for all queues owned by
/// device.
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///
/// # Host Synchronization
///  - Host access to all [`VkQueue`] objects from `device` must be externally synchronized
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorDeviceLost`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDeviceWaitIdle = unsafe extern "system" fn(device: VkDevice) -> VkResult;

/// The name of [`VkDeviceWaitIdle`]
pub const VK_DEVICE_WAIT_IDLE: &CStr = c"vkDeviceWaitIdle";
