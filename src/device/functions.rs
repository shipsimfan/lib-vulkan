use crate::{Instance, Loader, Result, VkDestroyDevice, VkDevice, VkGetDeviceQueue};

pub(super) struct DeviceFunctions {
    pub(super) destroy_device: VkDestroyDevice,
    pub(super) get_queue: VkGetDeviceQueue,
}

macro_rules! load_function {
    ($instance: ident, $device: ident, $name: literal) => {
        $instance
            .get_device_proc_addr($device, $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(crate::VkResult::IncompatibleDriver)
    };
}

impl DeviceFunctions {
    pub(super) fn load<L: Loader>(instance: &Instance<L>, device: VkDevice) -> Result<Self> {
        let destroy_device = load_function!(instance, device, "vkDestroyDevice")?;
        let get_queue = load_function!(instance, device, "vkGetDeviceQueue")?;

        Ok(DeviceFunctions {
            destroy_device,
            get_queue,
        })
    }
}
