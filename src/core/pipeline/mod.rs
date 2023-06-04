use crate::{
    Device, Loader, NativeLoader, PipelineLayout, RenderPass, Result, VkPipeline, VkResult,
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
        let vk_create_info = create_info.into_binding();

        let mut handle = Vec::with_capacity(1);
        let handle = match (device.pipeline_functions().create_pipeline)(
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
