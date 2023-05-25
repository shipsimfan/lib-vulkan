use crate::{
    Loader, Result, VkGetPhysicalDeviceFeatures, VkGetPhysicalDeviceProperties,
    VkGetPhysicalDeviceQueueFamilyProperties, VkInstance,
};

pub(crate) struct PhysicalDeviceFunctions {
    pub(super) get_features: VkGetPhysicalDeviceFeatures,
    pub(super) get_properties: VkGetPhysicalDeviceProperties,
    pub(super) get_queue_family_properties: VkGetPhysicalDeviceQueueFamilyProperties,
}

macro_rules! load_function {
    ($loader: ident, $instance: ident, $name: literal) => {
        $loader
            .get_instance_proc_addr(Some($instance), $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(crate::VkResult::IncompatibleDriver)
    };
}

impl PhysicalDeviceFunctions {
    pub(crate) fn load<L: Loader>(loader: &L, instance: VkInstance) -> Result<Self> {
        let get_features = load_function!(loader, instance, "vkGetPhysicalDeviceFeatures")?;
        let get_properties = load_function!(loader, instance, "vkGetPhysicalDeviceProperties")?;
        let get_queue_family_properties =
            load_function!(loader, instance, "vkGetPhysicalDeviceQueueFamilyProperties")?;

        Ok(PhysicalDeviceFunctions {
            get_features,
            get_properties,
            get_queue_family_properties,
        })
    }
}
