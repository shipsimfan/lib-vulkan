use crate::{Instance, Loader, Result, VkCreatePipelineLayout, VkDestroyPipelineLayout, VkDevice};

pub(crate) struct PipelineLayoutFunctions {
    pub(super) create_pipeline_layout: VkCreatePipelineLayout,
    pub(super) destroy_pipeline_layout: VkDestroyPipelineLayout,
}

macro_rules! load_function {
    ($instance: ident, $device: ident, $name: literal) => {
        $instance
            .get_device_proc_addr($device, $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(crate::VkResult::IncompatibleDriver)
    };
}

impl PipelineLayoutFunctions {
    pub(crate) fn load<L: Loader>(instance: &Instance<L>, device: VkDevice) -> Result<Self> {
        let create_pipeline_layout = load_function!(instance, device, "vkCreatePipelineLayout")?;
        let destroy_pipeline_layout = load_function!(instance, device, "vkDestroyPipelineLayout")?;

        Ok(PipelineLayoutFunctions {
            create_pipeline_layout,
            destroy_pipeline_layout,
        })
    }
}
