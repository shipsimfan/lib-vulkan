use crate::{VkAllocationCallbacks, VkDevice, VkResult, VkShaderModule, VkShaderModuleCreateInfo};

pub(crate) type VkCreateShaderModule = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkShaderModuleCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_shader_module: *mut Option<VkShaderModule>,
) -> VkResult;

pub(crate) type VkDestroyShaderModule = extern "system" fn(
    device: VkDevice,
    shader_module: VkShaderModule,
    p_allocator: *const VkAllocationCallbacks,
);
