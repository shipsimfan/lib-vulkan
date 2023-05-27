use crate::{Instance, Loader, Result, VkCreateSwapchainKHR, VkDestroySwapchainKHR, VkDevice};

pub(crate) struct SwapchainFunctions {
    pub(super) create_swapchain: VkCreateSwapchainKHR,
    pub(super) destroy_swapchain: VkDestroySwapchainKHR,
}

macro_rules! load_function {
    ($instance: ident, $device: ident, $name: literal) => {
        $instance
            .get_device_proc_addr($device, $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(crate::VkResult::IncompatibleDriver)
    };
}

impl SwapchainFunctions {
    pub(crate) fn load<L: Loader>(instance: &Instance<L>, device: VkDevice) -> Result<Self> {
        let create_swapchain = load_function!(instance, device, "vkCreateSwapchainKHR")?;
        let destroy_swapchain = load_function!(instance, device, "vkDestroySwapchainKHR")?;

        Ok(SwapchainFunctions {
            create_swapchain,
            destroy_swapchain,
        })
    }
}
