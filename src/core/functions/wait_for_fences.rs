use crate::{VkBool32, VkDevice, VkFence, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_0};

/// Wait for one or more fences to become signaled
///
/// # Parameters
///  - `device` is the logical device that owns the fences.
///  - `fence_count` is the number of fences to wait on.
///  - `fences` is a pointer to an array of `fence_count` fence handles.
///  - `wait_all` is the condition that must be satisfied to successfully unblock the wait. If
///    `wait_all` is [`VK_TRUE`], then the condition is that all fences in `fences` are signaled.
///    Otherwise, the condition is that at least one fence in `fences` is signaled.
///  - `timeout` is the timeout period in units of nanoseconds. `timeout` is adjusted to the
///    closest value allowed by the implementation-dependent timeout accuracy, which may be
///    substantially longer than one nanosecond, and may be longer than the requested period.
///
/// # Description
/// If the condition is satisfied when [`VkWaitForFences`] is called, then [`VkWaitForFences`]
/// returns immediately. If the condition is not satisfied at the time [`VkWaitForFences`] is
/// called, then [`VkWaitForFences`] will block and wait until the condition is satisfied or the
/// timeout has expired, whichever is sooner.
///
/// If `timeout` is zero, then [`VkWaitForFences`] does not wait, but simply returns the current
/// state of the fences. [`VkResult::VkTimeout`] will be returned in this case if the condition is
/// not satisfied, even though no actual wait was performed.
///
/// If the condition is satisfied before the timeout has expired, [`VkWaitForFences`] returns
/// [`VkResult::VkSuccess`]. Otherwise, [`VkWaitForFences`] returns [`VkResult::VkTimeout`] after
/// the timeout has expired.
///
/// If device loss occurs before the timeout has expired, [`VkWaitForFences`] must return in finite
/// time with either [`VkResult::VkSuccess`] or [`VkResult::VkErrorDeviceLost`].
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `fences` must be a valid pointer to an array of `fence_count` valid [`VkFence`] handles
///  - `fence_count` must be greater than 0
///  - Each element of `fences` must have been created, allocated, or retrieved from device
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///  - [`VkResult::VkTimeout`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorDeviceLost`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkWaitForFences = unsafe extern "system" fn(
    device: VkDevice,
    fence_count: u32,
    fences: *const VkFence,
    wait_all: VkBool32,
    timeout: u64,
) -> VkResult;

/// The name of [`VkWaitForFences`]
pub const VK_WAIT_FOR_FENCES: &CStr = c"vkWaitForFences";
