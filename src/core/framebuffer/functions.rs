use crate::{Instance, Loader, Result, VkCreateFramebuffer, VkDestroyFramebuffer, VkDevice};

pub(crate) struct FramebufferFunctions {
    pub(super) create_framebuffer: VkCreateFramebuffer,
    pub(super) destroy_framebuffer: VkDestroyFramebuffer,
}

macro_rules! load_function {
    ($instance: ident, $device: ident, $name: literal) => {
        $instance
            .get_device_proc_addr($device, $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(crate::VkResult::IncompatibleDriver)
    };
}

impl FramebufferFunctions {
    pub(crate) fn load<L: Loader>(instance: &Instance<L>, device: VkDevice) -> Result<Self> {
        let create_framebuffer = load_function!(instance, device, "vkCreateFramebuffer")?;
        let destroy_framebuffer = load_function!(instance, device, "vkDestroyFramebuffer")?;

        Ok(FramebufferFunctions {
            create_framebuffer,
            destroy_framebuffer,
        })
    }
}
