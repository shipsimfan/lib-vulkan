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
    pub(super) fn into_binding(&self) -> VkGraphicsPipelineCreateInfo {
        let mut strings = Vec::with_capacity(self.stages.len());
        let stages: Vec<_> = self
            .stages
            .iter()
            .map(|stage| {
                let (stage, string) = stage.into_binding();
                strings.push(string);
                stage
            })
            .collect();

        let vertex_input_state = self
            .vertex_input_state
            .as_ref()
            .map(|vertex_input_state| vertex_input_state.into_binding());
        let input_assembly_state = self
            .input_assembly_state
            .as_ref()
            .map(|input_assembly_state| input_assembly_state.into_binding());
        let tessellation_state = self
            .tessellation_state
            .as_ref()
            .map(|tessellation_state| tessellation_state.into_binding());
        let viewport_state = self
            .viewport_state
            .as_ref()
            .map(|viewport_state| viewport_state.into_binding());
        let rasterization_state = self
            .rasterization_state
            .as_ref()
            .map(|rasterization_state| rasterization_state.into_binding());
        let multisample_state = self
            .multisample_state
            .as_ref()
            .map(|multisample_state| multisample_state.into_binding());
        let depth_stencil_state = self
            .depth_stencil_state
            .as_ref()
            .map(|depth_stencil_state| depth_stencil_state.into_binding());
        let color_blend_state = self
            .color_blend_state
            .as_ref()
            .map(|color_blend_state| color_blend_state.into_binding());
        let dynamic_state = VkPipelineDynamicStateCreateInfo {
            s_type: VkStructureType::PipelineDynamicStateCreateInfo,
            p_next: null(),
            flags: VkPipelineDynamicStateCreateFlags::default(),
            dynamic_state_count: self.dynamic_state.len() as u32,
            p_dynamic_states: if self.dynamic_state.len() == 0 {
                null()
            } else {
                self.dynamic_state.as_ptr()
            },
        };

        VkGraphicsPipelineCreateInfo {
            s_type: VkStructureType::GraphicsPipelineCreateInfo,
            p_next: null(),
            flags: self.flags,
            stage_count: stages.len() as u32,
            p_stages: if stages.len() == 0 {
                null()
            } else {
                stages.as_ptr()
            },
            p_vertex_input_state: vertex_input_state
                .as_ref()
                .map(|vertex_input_state| vertex_input_state as *const _)
                .unwrap_or(null()),
            p_input_assembly_state: input_assembly_state
                .as_ref()
                .map(|input_assembly_state| input_assembly_state as *const _)
                .unwrap_or(null()),
            p_tessellation_state: tessellation_state
                .as_ref()
                .map(|tessellation_state| tessellation_state as *const _)
                .unwrap_or(null()),
            p_viewport_state: viewport_state
                .as_ref()
                .map(|viewport_state| viewport_state as *const _)
                .unwrap_or(null()),
            p_rasterization_state: rasterization_state
                .as_ref()
                .map(|rasterization_state| rasterization_state as *const _)
                .unwrap_or(null()),
            p_multisample_state: multisample_state
                .as_ref()
                .map(|multisample_state| multisample_state as *const _)
                .unwrap_or(null()),
            p_depth_stencil_state: depth_stencil_state
                .as_ref()
                .map(|depth_stencil_state| depth_stencil_state as *const _)
                .unwrap_or(null()),
            p_color_blend_state: color_blend_state
                .as_ref()
                .map(|color_blend_state| &color_blend_state.0 as *const _)
                .unwrap_or(null()),
            p_dynamic_state: &dynamic_state,
            layout: self.layout.handle(),
            render_pass: self.render_pass.handle(),
            subpass: self.subpass,
            base_pipeline_handle: None,
            base_pipeline_index: 0,
        }
    }

    pub(super) fn arcs(self) -> (Arc<PipelineLayout<L>>, Arc<RenderPass<L>>) {
        (self.layout, self.render_pass)
    }
}
