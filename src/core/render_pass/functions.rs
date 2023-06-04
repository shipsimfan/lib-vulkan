use crate::{Instance, Loader, Result, VkCreateRenderPass, VkDestroyRenderPass, VkDevice};

pub(crate) struct RenderPassFunctions {
    pub(super) create_render_pass: VkCreateRenderPass,
    pub(super) destroy_render_pass: VkDestroyRenderPass,
}

macro_rules! load_function {
    ($instance: ident, $device: ident, $name: literal) => {
        $instance
            .get_device_proc_addr($device, $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(crate::VkResult::IncompatibleDriver)
    };
}

impl RenderPassFunctions {
    pub(crate) fn load<L: Loader>(instance: &Instance<L>, device: VkDevice) -> Result<Self> {
        let create_render_pass = load_function!(instance, device, "vkCreateRenderPass")?;
        let destroy_render_pass = load_function!(instance, device, "vkDestroyRenderPass")?;

        Ok(RenderPassFunctions {
            create_render_pass,
            destroy_render_pass,
        })
    }
}
