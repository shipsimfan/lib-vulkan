use crate::{
    Instance, Loader, NativeLoader, VkPhysicalDevice, VkPhysicalDeviceFeatures,
    VkPhysicalDeviceProperties,
};
use std::sync::Arc;

mod features;
mod functions;
mod limits;
mod properties;
mod sparse_properties;

pub use features::PhysicalDeviceFeatures;
pub use limits::PhysicalDeviceLimits;
pub use properties::PhysicalDeviceProperties;
pub use sparse_properties::PhysicalDeviceSparseProperties;

pub(crate) use functions::PhysicalDeviceFunctions;

pub struct PhysicalDevice<L: Loader = NativeLoader> {
    handle: VkPhysicalDevice,
    instance: Arc<Instance<L>>,
}

impl<L: Loader> PhysicalDevice<L> {
    pub(crate) fn new(handle: VkPhysicalDevice, instance: Arc<Instance<L>>) -> Self {
        PhysicalDevice { handle, instance }
    }

    pub fn get_features(&self) -> PhysicalDeviceFeatures {
        let mut features = VkPhysicalDeviceFeatures::default();
        (self.instance.physical_device_functions().get_features)(self.handle, &mut features);
        features.into()
    }

    pub fn get_properties(&self) -> PhysicalDeviceProperties {
        let mut properties = VkPhysicalDeviceProperties::default();
        (self.instance.physical_device_functions().get_properties)(self.handle, &mut properties);
        properties.into()
    }
}
