use crate::{
    VkAllocationCallbacks, VkDescriptorSetLayout, VkDescriptorSetLayoutCreateInfo, VkDevice,
    VkResult,
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null;

/// Create a new descriptor set layout
///
/// # Parameters
///  - `device` is the logical device that creates the descriptor set layout.
///  - `create_info` is a pointer to a [`VkDescriptorSetLayoutCreateInfo`] structure specifying the
///    state of the descriptor set layout object.
///  - `allocator` controls host memory allocation.
///  - `set_layout` is a pointer to a [`VkDescriptorSetLayout`] handle in which the resulting
///    descriptor set layout object is returned.
///
/// # Valid Usage
///  - If the descriptor layout exceeds the limits reported through the physical device limits,
///    then [`VkGetDescriptorSetLayoutSupport`] must have returned [`VkDescriptorSetLayoutSupport`]
///    with `support` equal to [`VK_TRUE`] for `create_info`
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `create_info` must be a valid pointer to a valid [`VkDescriptorSetLayoutCreateInfo`]
///    structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `set_layout` must be a valid pointer to a [`VkDescriptorSetLayout`] handle
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
pub type VkCreateDescriptorSetLayout = unsafe extern "system" fn(
    device: VkDevice,
    create_info: *const VkDescriptorSetLayoutCreateInfo,
    allocator: *const VkAllocationCallbacks,
    set_layout: *mut VkDescriptorSetLayout,
) -> VkResult;

/// The name of [`VkCreateDescriptorSetLayout`]
pub const VK_CREATE_DESCRIPTOR_SET_LAYOUT: &CStr = c"vkCreateDescriptorSetLayout";
