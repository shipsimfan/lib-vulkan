use crate::{
    Loader, Result, VkDestroyInstance, VkEnumeratePhysicalDevice, VkGetDeviceProcAddr, VkInstance,
};

pub(super) struct InstanceFunctions {
    pub(super) destroy_instance: VkDestroyInstance,
    pub(super) enumerate_physical_devices: VkEnumeratePhysicalDevice,
    pub(super) get_device_proc_addr: VkGetDeviceProcAddr,
}

macro_rules! load_function {
    ($loader: ident, $instance: ident, $name: literal) => {
        $loader
            .get_instance_proc_addr(Some($instance), $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(crate::VkResult::IncompatibleDriver)
    };
}

impl InstanceFunctions {
    pub(super) fn load<L: Loader>(loader: &L, instance: VkInstance) -> Result<Self> {
        let destroy_instance = load_function!(loader, instance, "vkDestroyInstance")?;
        let enumerate_physical_devices =
            load_function!(loader, instance, "vkEnumeratePhysicalDevices")?;
        let get_device_proc_addr = load_function!(loader, instance, "vkGetDeviceProcAddr")?;

        Ok(InstanceFunctions {
            destroy_instance,
            enumerate_physical_devices,
            get_device_proc_addr,
        })
    }
}
