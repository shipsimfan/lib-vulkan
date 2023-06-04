use crate::{
    VkPipeline, VkPipelineColorBlendStateCreateInfo, VkPipelineCreateFlags,
    VkPipelineDepthStencilStateCreateInfo, VkPipelineDynamicStateCreateInfo,
    VkPipelineInputAssemblyStateCreateInfo, VkPipelineLayout, VkPipelineMultisampleStateCreateInfo,
    VkPipelineRasterizationStateCreateInfo, VkPipelineShaderStageCreateInfo,
    VkPipelineTessellationStateCreateInfo, VkPipelineVertexInputStateCreateInfo,
    VkPipelineViewportStateCreateInfo, VkRenderPass, VkStructureType,
};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkGraphicsPipelineCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkPipelineCreateFlags,
    pub(crate) stage_count: u32,
    pub(crate) p_stages: *const VkPipelineShaderStageCreateInfo,
    pub(crate) p_vertex_input_state: *const VkPipelineVertexInputStateCreateInfo,
    pub(crate) p_input_assembly_state: *const VkPipelineInputAssemblyStateCreateInfo,
    pub(crate) p_tessellation_state: *const VkPipelineTessellationStateCreateInfo,
    pub(crate) p_viewport_state: *const VkPipelineViewportStateCreateInfo,
    pub(crate) p_rasterization_state: *const VkPipelineRasterizationStateCreateInfo,
    pub(crate) p_multisample_state: *const VkPipelineMultisampleStateCreateInfo,
    pub(crate) p_depth_stencil_state: *const VkPipelineDepthStencilStateCreateInfo,
    pub(crate) p_color_blend_state: *const VkPipelineColorBlendStateCreateInfo,
    pub(crate) p_dynamic_state: *const VkPipelineDynamicStateCreateInfo,
    pub(crate) layout: VkPipelineLayout,
    pub(crate) render_pass: VkRenderPass,
    pub(crate) subpass: u32,
    pub(crate) base_pipeline_handle: Option<VkPipeline>,
    pub(crate) base_pipeline_index: u32,
}
