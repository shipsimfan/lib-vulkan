use crate::{VkDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Wait for a device to become idle
///
/// # Parameters
///  - `device` is the logical device to idle.
///
/// # Description
/// [`VkDeviceWaitIdle`] is equivalent to calling [`VkQueueWaitIdle`] for all queues owned by
/// device.
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
pub type VkDeviceWaitIdle = extern "system" fn(device: VkDevice) -> VkResult;

/// The name of [`VkDeviceWaitIdle`]
pub const VK_DEVICE_WAIT_IDLE: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkDeviceWaitIdle\0") };
