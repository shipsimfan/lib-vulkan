use crate::{Instance, Loader, Result, VkCreateCommandPool, VkDestroyCommandPool, VkDevice};

pub(crate) struct CommandPoolFunctions {
    pub(super) create_command_pool: VkCreateCommandPool,
    pub(super) destroy_command_pool: VkDestroyCommandPool,
}

macro_rules! load_function {
    ($instance: ident, $device: ident, $name: literal) => {
        $instance
            .get_device_proc_addr($device, $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(crate::VkResult::IncompatibleDriver)
    };
}

impl CommandPoolFunctions {
    pub(crate) fn load<L: Loader>(instance: &Instance<L>, device: VkDevice) -> Result<Self> {
        let create_command_pool = load_function!(instance, device, "vkCreateCommandPool")?;
        let destroy_command_pool = load_function!(instance, device, "vkDestroyCommandPool")?;

        Ok(CommandPoolFunctions {
            create_command_pool,
            destroy_command_pool,
        })
    }
}
