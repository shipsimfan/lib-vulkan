use crate::{
    Loader, Result, VkDestroyInstance, VkEnumerateInstanceExtensionProperties, VkInstance, VkResult,
};

pub(super) struct InstanceFunctions {
    pub(super) destroy_instance: VkDestroyInstance,
    pub(super) enumerate_extension_properties: VkEnumerateInstanceExtensionProperties,
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
        let enumerate_extension_properties =
            load_function!(loader, instance, "vkEnumerateInstanceExtensionProperties")?;

        Ok(InstanceFunctions {
            destroy_instance,
            enumerate_extension_properties,
        })
    }
}
