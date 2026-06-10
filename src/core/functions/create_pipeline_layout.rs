use crate::{
    VkAllocationCallbacks, VkDevice, VkPipelineLayout, VkPipelineLayoutCreateInfo, VkResult,
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::null;

/// Creates a new pipeline layout object
///
/// # Parameters
///  - `device` is the logical device that creates the pipeline layout.
///  - `create_info` is a pointer to a [`VkPipelineLayoutCreateInfo`] structure specifying the
///    state of the pipeline layout object.
///  - `allocator` controls host memory allocation.
///  - `pipeline_layout` is a pointer to a [`VkPipelineLayout`] handle in which the resulting
///    pipeline layout object is returned.
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `create_info` must be a valid pointer to a valid [`VkPipelineLayoutCreateInfo`] structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `pipeline_layout` must be a valid pointer to a [`VkPipelineLayout`] handle
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
/// Provided by [`VK_VERSION_1_0`]
pub type VkCreatePipelineLayout = unsafe extern "system" fn(
    device: VkDevice,
    create_info: *const VkPipelineLayoutCreateInfo,
    allocator: *const VkAllocationCallbacks,
    pipeline_layout: *mut VkPipelineLayout,
) -> VkResult;

/// The name of [`VkCreatePipelineLayout`]
pub const VK_CREATE_PIPELINE_LAYOUT: &CStr = c"vkCreatePipelineLayout";
