use crate::{
    VkAllocationCallbacks, VkDevice, VkGraphicsPipelineCreateInfo, VkPipeline, VkPipelineCache,
    VkResult,
};

pub(crate) type VkCreateGraphicsPipeline = extern "system" fn(
    device: VkDevice,
    pipeline_cache: Option<VkPipelineCache>,
    create_info_count: u32,
    p_create_infos: *const VkGraphicsPipelineCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_pipelines: *mut VkPipeline,
) -> VkResult;

pub(crate) type VkDestroyPipeline = extern "system" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    p_allocator: *const VkAllocationCallbacks,
);
