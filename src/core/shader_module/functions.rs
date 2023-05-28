use crate::{Instance, Loader, Result, VkCreateShaderModule, VkDestroyShaderModule, VkDevice};

pub(crate) struct ShaderModuleFunctions {
    pub(super) create_shader_module: VkCreateShaderModule,
    pub(super) destroy_shader_module: VkDestroyShaderModule,
}

macro_rules! load_function {
    ($instance: ident, $device: ident, $name: literal) => {
        $instance
            .get_device_proc_addr($device, $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(crate::VkResult::IncompatibleDriver)
    };
}

impl ShaderModuleFunctions {
    pub(crate) fn load<L: Loader>(instance: &Instance<L>, device: VkDevice) -> Result<Self> {
        let create_shader_module = load_function!(instance, device, "vkCreateShaderModule")?;
        let destroy_shader_module = load_function!(instance, device, "vkDestroyShaderModule")?;

        Ok(ShaderModuleFunctions {
            create_shader_module,
            destroy_shader_module,
        })
    }
}
