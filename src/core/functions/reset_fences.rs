use crate::{VkDevice, VkFence, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Resets one or more fence objects
///
/// # Parameters
///  - `device` is the logical device that owns the fences.
///  - `fence_count` is the number of fences to reset.
///  - `fences` is a pointer to an array of fence handles to reset.
///
/// # Description
/// If any member of `fences` currently has its payload imported with temporary permanence, that
/// fence’s prior permanent payload is first restored. The remaining operations described therefore
/// operate on the restored payload.
///
/// When [`VkResetFences`] is executed on the host, it defines a fence unsignal operation for each
/// fence, which resets the fence to the unsignaled state.
///
/// If any member of `fences` is already in the unsignaled state when [`VkResetFences`] is
/// executed, then [`VkResetFences`] has no effect on that fence.
///
/// # Valid Usage
///  - Each element of `fences` must not be currently associated with any queue command that has
///    not yet completed execution on that queue
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `fences` must be a valid pointer to an array of `fence_count` valid [`VkFence`] handles
///  - `fence_count` must be greater than 0
///  - Each element of `fences` must have been created, allocated, or retrieved from `device`
///
/// # Host Synchronization
///  - Host access to each member of `fences` must be externally synchronized
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
pub type VkResetFences = unsafe extern "system" fn(
    device: VkDevice,
    fence_count: u32,
    fences: *const VkFence,
) -> VkResult;

/// The name of [`VkResetFences`]
pub const VK_RESET_FENCES: &CStr = c"vkResetFences";
