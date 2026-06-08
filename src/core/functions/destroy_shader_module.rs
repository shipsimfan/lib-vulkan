use crate::{VkAllocationCallbacks, VkDevice, VkShaderModule};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy a shader module
///
/// # Parameters
///  - `device` is the logical device that destroys the shader module.
///  - `shader_module` is the handle of the shader module to destroy.
///  - `allocator` controls host memory allocation as described in the Memory Allocation chapter.
///
/// # Description
/// A shader module can be destroyed while pipelines created using its shaders are still in use.
///
/// # Valid Usage
///  - If [`VkAllocationCallbacks`] were provided when `shader_module` was created, a compatible
///    set of callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when `shader_module` was created, `allocator`
///    must be [`null`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `shader_module` is not [`VK_NULL_HANDLE`], `shader_module` must be a valid
///    [`VkShaderModule`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `shader_module` is a valid handle, it must have been created, allocated, or retrieved
///    from `device`
///
/// # Host Synchronization
///  - Host access to `shader_module` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyShaderModule = unsafe extern "system" fn(
    device: VkDevice,
    shader_module: VkShaderModule,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroyShaderModule`]
pub const VK_DESTROY_SHADER_MODULE: &CStr = c"vkDestroyShaderModule";
