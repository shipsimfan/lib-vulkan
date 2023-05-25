use crate::{Loader, Result, VkDestroyInstance, VkEnumeratePhysicalDevice, VkInstance, VkResult};

pub(super) struct InstanceFunctions {
    pub(super) destroy_instance: VkDestroyInstance,
    pub(super) enumerate_physical_devices: VkEnumeratePhysicalDevice,
}

macro_rules! load_function {
    ($loader: ident, $instance: ident, $name: literal) => {
        $loader
            .get_instance_proc_addr(Some($instance), $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(VkResult::IncompatibleDriver)
    };
}

impl InstanceFunctions {
    pub(super) fn load<L: Loader>(loader: &L, instance: VkInstance) -> Result<Self> {
        let destroy_instance = load_function!(loader, instance, "vkDestroyInstance")?;
        let enumerate_physical_devices =
            load_function!(loader, instance, "vkEnumeratePhysicalDevices")?;

        Ok(InstanceFunctions {
            destroy_instance,
            enumerate_physical_devices,
        })
    }
}
