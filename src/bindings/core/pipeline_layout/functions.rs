use crate::{
    VkAllocationCallbacks, VkDevice, VkPipelineLayout, VkPipelineLayoutCreateInfo, VkResult,
};

pub(crate) type VkCreatePipelineLayout = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkPipelineLayoutCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_pipeline_layout: *mut Option<VkPipelineLayout>,
) -> VkResult;

pub(crate) type VkDestroyPipelineLayout = extern "system" fn(
    device: VkDevice,
    pipeline_layout: VkPipelineLayout,
    p_allocator: *const VkAllocationCallbacks,
) -> VkResult;
