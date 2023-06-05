use crate::{Instance, Loader, Result, VkCreateGraphicsPipelines, VkDestroyPipeline, VkDevice};

pub(crate) struct PipelineFunctions {
    pub(super) create_pipelines: VkCreateGraphicsPipelines,
    pub(super) destroy_pipeline: VkDestroyPipeline,
}

macro_rules! load_function {
    ($instance: ident, $device: ident, $name: literal) => {
        $instance
            .get_device_proc_addr($device, $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(crate::VkResult::IncompatibleDriver)
    };
}

impl PipelineFunctions {
    pub(crate) fn load<L: Loader>(instance: &Instance<L>, device: VkDevice) -> Result<Self> {
        let create_pipelines = load_function!(instance, device, "vkCreateGraphicsPipelines")?;
        let destroy_pipeline = load_function!(instance, device, "vkDestroyPipeline")?;

        Ok(PipelineFunctions {
            create_pipelines,
            destroy_pipeline,
        })
    }
}
