use crate::{VkAllocationCallbacks, VkDevice, VkResult, VkShaderModule, VkShaderModuleCreateInfo};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::null;

/// Creates a new shader module object
///
/// # Parameters
///  - `device` is the logical device that creates the shader module.
///  - `create_info` is a pointer to a [`VkShaderModuleCreateInfo`] structure.
///  - `allocator` controls host memory allocation as described in the Memory Allocation chapter.
///  - `shader_module` is a pointer to a [`VkShaderModule`] handle in which the resulting shader
///    module object is returned.
///
/// # Description
/// Once a shader module has been created, any entry points it contains can be used in pipeline
/// shader stages as described in Compute Pipelines and Graphics Pipelines.
///
/// # Valid Usage
///  - If `create_info` is not [`null`], `create_info->next` must be [`null`] or a pointer to a
///    valid instance of
///    - [`VkShaderModuleValidationCacheCreateInfoExt`]
///    - [`VkValidationFeaturesExt`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `create_info` must be a valid pointer to a valid [`VkShaderModuleCreateInfo`] structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `shader_module` must be a valid pointer to a [`VkShaderModule`] handle
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorInvalidShaderNv`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCreateShaderModule = unsafe extern "system" fn(
    device: VkDevice,
    create_info: *const VkShaderModuleCreateInfo,
    allocator: *const VkAllocationCallbacks,
    shader_module: *mut VkShaderModule,
) -> VkResult;

/// The name of [`VkCreateShaderModule`]
pub const VK_CREATE_SHADER_MODULE: &CStr = c"vkCreateShaderModule";
