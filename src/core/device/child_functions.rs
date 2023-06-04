use crate::{
    ImageViewFunctions, Instance, Loader, PipelineLayoutFunctions, RenderPassFunctions, Result,
    ShaderModuleFunctions, SwapchainFunctions, VkDevice,
};

pub(super) struct ChildFunctions {
    pub(super) image_view_functions: ImageViewFunctions,
    pub(super) pipeline_layout_functions: PipelineLayoutFunctions,
    pub(super) render_pass_functions: RenderPassFunctions,
    pub(super) shader_module_functions: ShaderModuleFunctions,

    pub(super) swapchain_functions: Option<SwapchainFunctions>,
}

impl ChildFunctions {
    pub(crate) fn load<L: Loader>(
        instance: &Instance<L>,
        device: VkDevice,
        extension_list: &[String],
    ) -> Result<Self> {
        let image_view_functions = ImageViewFunctions::load(instance, device)?;
        let pipeline_layout_functions = PipelineLayoutFunctions::load(instance, device)?;
        let render_pass_functions = RenderPassFunctions::load(instance, device)?;
        let shader_module_functions = ShaderModuleFunctions::load(instance, device)?;

        let mut swapchain_functions = None;

        for extension in extension_list {
            match extension.as_str() {
                "VK_KHR_swapchain" => {
                    swapchain_functions = Some(SwapchainFunctions::load(instance, device)?)
                }
                _ => {}
            }
        }

        Ok(ChildFunctions {
            image_view_functions,
            pipeline_layout_functions,
            render_pass_functions,
            shader_module_functions,

            swapchain_functions,
        })
    }
}
