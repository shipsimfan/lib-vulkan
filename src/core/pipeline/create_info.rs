use crate::{
    DynamicState, Loader, NativeLoader, PipelineColorBlendStateCreateInfo, PipelineCreateFlags,
    PipelineDepthStencilStateCreateInfo, PipelineInputAssemblyStateCreateInfo, PipelineLayout,
    PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo,
    PipelineShaderStageCreateInfo, PipelineTessellationStateCreateInfo,
    PipelineVertexInputStateCreateInfo, PipelineViewportStateCreateInfo, RenderPass,
    VkGraphicsPipelineCreateInfo, VkPipelineDynamicStateCreateFlags,
    VkPipelineDynamicStateCreateInfo, VkStructureType,
};
use std::{ptr::null, sync::Arc};

pub struct GraphicsPipelineCreateInfo<L: Loader = NativeLoader> {
    pub flags: PipelineCreateFlags,
    pub stages: Vec<PipelineShaderStageCreateInfo>,
    pub vertex_input_state: Option<PipelineVertexInputStateCreateInfo>,
    pub input_assembly_state: Option<PipelineInputAssemblyStateCreateInfo>,
    pub tessellation_state: Option<PipelineTessellationStateCreateInfo>,
    pub viewport_state: Option<PipelineViewportStateCreateInfo>,
    pub rasterization_state: Option<PipelineRasterizationStateCreateInfo>,
    pub multisample_state: Option<PipelineMultisampleStateCreateInfo>,
    pub depth_stencil_state: Option<PipelineDepthStencilStateCreateInfo>,
    pub color_blend_state: Option<PipelineColorBlendStateCreateInfo>,
    pub dynamic_state: Vec<DynamicState>,
    pub layout: Arc<PipelineLayout<L>>,
    pub render_pass: Arc<RenderPass<L>>,
    pub subpass: u32,
}

impl<L: Loader> GraphicsPipelineCreateInfo<L> {
    pub(super) fn arcs(self) -> (Arc<PipelineLayout<L>>, Arc<RenderPass<L>>) {
        (self.layout, self.render_pass)
    }
}
