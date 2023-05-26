use crate::{
    Device, DeviceCreateInfo, Instance, Loader, NativeLoader, QueueFamilyProperties, Result,
    VkPhysicalDevice, VkPhysicalDeviceFeatures, VkPhysicalDeviceProperties,
};
use std::{ptr::null_mut, sync::Arc};

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

    pub fn create_device(&self, create_info: DeviceCreateInfo) -> Result<Arc<Device<L>>> {
        Device::create_device(
            self.handle,
            self.instance.clone(),
            self.instance.physical_device_functions().create_device,
            create_info,
        )
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

    pub fn get_queue_family_properties(&self) -> Vec<QueueFamilyProperties> {
        let mut count = 0;
        (self
            .instance
            .physical_device_functions()
            .get_queue_family_properties)(self.handle, &mut count, null_mut());

        let mut queue_family_properties = Vec::with_capacity(count as usize);
        (self
            .instance
            .physical_device_functions()
            .get_queue_family_properties)(
            self.handle,
            &mut count,
            queue_family_properties.as_mut_ptr(),
        );

        unsafe { queue_family_properties.set_len(count as usize) };
        queue_family_properties
    }
}
