use crate::{
    VkAllocationCallbacks, VkDescriptorPool, VkDescriptorPoolCreateInfo, VkDevice, VkResult,
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::null;

/// Creates a descriptor pool object
///
/// # Parameters
///  - `device` is the logical device that creates the descriptor pool.
///  - `create_info` is a pointer to a [`VkDescriptorPoolCreateInfo`] structure specifying the
///    state of the descriptor pool object.
///  - `allocator` controls host memory allocation.
///  - `descriptor_pool` is a pointer to a [`VkDescriptorPool`] handle in which the resulting
///    descriptor pool object is returned.
///
/// # Description
/// The created descriptor pool is returned in `descriptor_pool`.
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `create_info` must be a valid pointer to a valid [`VkDescriptorPoolCreateInfo`] structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `descriptor_pool` must be a valid pointer to a [`VkDescriptorPool`] handle
///  - The device must have been created with at least 1 queue
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorFragmentation`]
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCreateDescriptorPool = unsafe extern "system" fn(
    device: VkDevice,
    create_info: *const VkDescriptorPoolCreateInfo,
    allocator: *const VkAllocationCallbacks,
    descriptor_pool: *mut VkDescriptorPool,
) -> VkResult;

/// The name of [`VkCreateDescriptorPool`]
pub const VK_CREATE_DESCRIPTOR_POOL: &CStr = c"vkCreateDescriptorPool";
