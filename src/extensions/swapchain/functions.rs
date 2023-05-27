use crate::{
    Instance, Loader, Result, VkCreateSwapchainKHR, VkDestroySwapchainKHR, VkDevice,
    VkGetSwapchainImagesKHR,
};

pub(crate) struct SwapchainFunctions {
    pub(super) create_swapchain: VkCreateSwapchainKHR,
    pub(super) destroy_swapchain: VkDestroySwapchainKHR,
    pub(super) get_swapchain_images: VkGetSwapchainImagesKHR,
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
        let get_swapchain_images = load_function!(instance, device, "vkGetSwapchainImagesKHR")?;

        Ok(SwapchainFunctions {
            create_swapchain,
            destroy_swapchain,
            get_swapchain_images,
        })
    }
}
