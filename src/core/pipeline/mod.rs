use crate::{
    Device, Loader, NativeLoader, PipelineLayout, RenderPass, Result, VkGraphicsPipelineCreateInfo,
    VkPipeline, VkPipelineDynamicStateCreateFlags, VkPipelineDynamicStateCreateInfo, VkResult,
    VkStructureType,
};
use std::{ptr::null, sync::Arc};

mod color_blend_attachment_state;
mod color_blend_state_crate_info;
mod create_info;
mod depth_stencil_state_create_info;
mod functions;
mod input_assembly_state_create_info;
mod multisample_state_create_info;
mod rasterization_state_create_info;
mod shader_stage_create_info;
mod tessellation_state_create_info;
mod vertex_input_state_create_info;
mod viewport_state_create_info;

pub use color_blend_attachment_state::PipelineColorBlendAttachmentState;
pub use color_blend_state_crate_info::PipelineColorBlendStateCreateInfo;
pub use create_info::GraphicsPipelineCreateInfo;
pub use depth_stencil_state_create_info::PipelineDepthStencilStateCreateInfo;
pub use input_assembly_state_create_info::PipelineInputAssemblyStateCreateInfo;
pub use multisample_state_create_info::PipelineMultisampleStateCreateInfo;
pub use rasterization_state_create_info::PipelineRasterizationStateCreateInfo;
pub use shader_stage_create_info::PipelineShaderStageCreateInfo;
pub use tessellation_state_create_info::PipelineTessellationStateCreateInfo;
pub use vertex_input_state_create_info::PipelineVertexInputStateCreateInfo;
pub use viewport_state_create_info::{PipelineViewportState, PipelineViewportStateCreateInfo};

pub(crate) use functions::PipelineFunctions;

pub struct Pipeline<L: Loader = NativeLoader> {
    handle: VkPipeline,
    device: Arc<Device<L>>,
    pipeline_layout: Arc<PipelineLayout<L>>,
    render_pass: Arc<RenderPass<L>>,
}

impl<L: Loader> Pipeline<L> {
    pub(crate) fn create(
        device: Arc<Device<L>>,
        create_info: GraphicsPipelineCreateInfo<L>,
    ) -> Result<Self> {
        let mut strings = Vec::with_capacity(create_info.stages.len());
        let stages: Vec<_> = create_info
            .stages
            .iter()
            .map(|stage| {
                let (stage, string) = stage.into_binding();
                strings.push(string);
                stage
            })
            .collect();

        let vertex_input_state = create_info
            .vertex_input_state
            .as_ref()
            .map(|vertex_input_state| vertex_input_state.into_binding());
        let input_assembly_state = create_info
            .input_assembly_state
            .as_ref()
            .map(|input_assembly_state| input_assembly_state.into_binding());
        let tessellation_state = create_info
            .tessellation_state
            .as_ref()
            .map(|tessellation_state| tessellation_state.into_binding());
        let viewport_state = create_info
            .viewport_state
            .as_ref()
            .map(|viewport_state| viewport_state.into_binding());
        let rasterization_state = create_info
            .rasterization_state
            .as_ref()
            .map(|rasterization_state| rasterization_state.into_binding());
        let multisample_state = create_info
            .multisample_state
            .as_ref()
            .map(|multisample_state| multisample_state.into_binding());
        let depth_stencil_state = create_info
            .depth_stencil_state
            .as_ref()
            .map(|depth_stencil_state| depth_stencil_state.into_binding());
        let color_blend_state = create_info
            .color_blend_state
            .as_ref()
            .map(|color_blend_state| color_blend_state.into_binding());
        let dynamic_state = VkPipelineDynamicStateCreateInfo {
            s_type: VkStructureType::PipelineDynamicStateCreateInfo,
            p_next: null(),
            flags: VkPipelineDynamicStateCreateFlags::default(),
            dynamic_state_count: create_info.dynamic_state.len() as u32,
            p_dynamic_states: if create_info.dynamic_state.len() == 0 {
                null()
            } else {
                create_info.dynamic_state.as_ptr()
            },
        };

        let vk_create_info = VkGraphicsPipelineCreateInfo {
            s_type: VkStructureType::GraphicsPipelineCreateInfo,
            p_next: null(),
            flags: create_info.flags,
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
            layout: create_info.layout.handle(),
            render_pass: create_info.render_pass.handle(),
            subpass: create_info.subpass,
            base_pipeline_handle: None,
            base_pipeline_index: 0,
        };

        let mut handle = Vec::with_capacity(1);
        let handle = match (device.pipeline_functions().create_pipelines)(
            device.handle(),
            None,
            1,
            &vk_create_info,
            null(),
            handle.as_mut_ptr(),
        ) {
            VkResult::Success => {
                unsafe { handle.set_len(1) };
                handle.pop().unwrap()
            }
            result => return Err(result),
        };

        let (pipeline_layout, render_pass) = create_info.arcs();

        Ok(Pipeline {
            handle,
            device,
            pipeline_layout,
            render_pass,
        })
    }
}

impl<L: Loader> Drop for Pipeline<L> {
    fn drop(&mut self) {
        (self.device.pipeline_functions().destroy_pipeline)(
            self.device.handle(),
            self.handle,
            null(),
        )
    }
}
