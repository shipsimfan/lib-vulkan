use crate::{VkAllocationCallbacks, VkDevice, VkFence, VkFenceCreateInfo, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::null;

/// Create a new fence object
///
/// # Parameters
///  - `device` is the logical device that creates the fence.
///  - `create_info` is a pointer to a [`VkFenceCreateInfo`] structure containing information about
///    how the fence is to be created.
///  - `allocator` controls host memory allocation.
///  - `fence` is a pointer to a handle in which the resulting fence object is returned.
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `create_info` must be a valid pointer to a valid [`VkFenceCreateInfo`] structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `fence` must be a valid pointer to a [`VkFence`] handle
///  - The device must have been created with at least 1 queue
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
///  Provided by [`VK_VERSION_1_0`]
pub type VkCreateFence = unsafe extern "system" fn(
    device: VkDevice,
    create_info: *const VkFenceCreateInfo,
    allocator: *const VkAllocationCallbacks,
    fence: *mut VkFence,
) -> VkResult;

/// The name of [`VkCreateFence`]
pub const VK_CREATE_FENCE: &CStr = c"vkCreateFence";
