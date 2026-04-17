use crate::{VkAllocationCallbacks, VkDevice, VkResult, VkSemaphore, VkSemaphoreCreateInfo};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::null;

/// Create a new queue semaphore object
///
/// # Parameters
///  - `device` is the logical device that creates the semaphore.
///  - `create_info` is a pointer to a [`VkSemaphoreCreateInfo`] structure containing information
///    about how the semaphore is to be created.
///  - `allocator` controls host memory allocation.
///  - `semaphore` is a pointer to a handle in which the resulting semaphore object is returned.
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `create_info` must be a valid pointer to a valid [`VkSemaphoreCreateInfo`] structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `semaphore` must be a valid pointer to a [`VkSemaphore`] handle
///  - The device must have been created with at least 1 queue
///
/// # Return Codes
///
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCreateSemaphore = unsafe extern "system" fn(
    device: VkDevice,
    create_info: *const VkSemaphoreCreateInfo,
    allocator: *const VkAllocationCallbacks,
    semaphore: *mut VkSemaphore,
) -> VkResult;

/// The name of [`VkCreateSemaphore`]
pub const VK_CREATE_SEMAPHORE: &CStr = c"vkCreateSemaphore";
