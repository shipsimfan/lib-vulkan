use crate::{
    bindings::{
        self, VkGetPhysicalDeviceFeatures, VkGetPhysicalDeviceProperties,
        VkGetPhysicalDeviceQueueFamilyProperties, VkPhysicalDeviceProperties,
    },
    get_instance_proc_addr, Result, VkInstance, VkPhysicalDeviceFeatures, VkQueueFamilyProperties,
};
use loader::Loader;
use native::NativeLoader;
use std::{ptr::NonNull, sync::Arc};

pub struct VkPhysicalDevice<L: Loader = NativeLoader> {
    inner: bindings::VkPhysicalDevice,
    instance: Arc<VkInstance<L>>,
}

pub(crate) struct VkPhysicalDeviceFunctions {
    get_physical_device_properties: VkGetPhysicalDeviceProperties,
    get_physical_device_features: VkGetPhysicalDeviceFeatures,
    get_physical_device_queue_family_properties: VkGetPhysicalDeviceQueueFamilyProperties,
}

impl<L: Loader> VkPhysicalDevice<L> {
    pub(crate) fn new(inner: bindings::VkPhysicalDevice, instance: Arc<VkInstance<L>>) -> Self {
        VkPhysicalDevice { inner, instance }
    }

    pub fn get_properties(&self) -> VkPhysicalDeviceProperties {
        let mut properties = VkPhysicalDeviceProperties::null();
        (self
            .instance
            .physical_device_functions()
            .get_physical_device_properties)(self.inner, unsafe {
            NonNull::new_unchecked(&mut properties)
        });
        properties
    }

    pub fn get_features(&self) -> VkPhysicalDeviceFeatures {
        let mut features = VkPhysicalDeviceFeatures::null();
        (self
            .instance
            .physical_device_functions()
            .get_physical_device_features)(self.inner, unsafe {
            NonNull::new_unchecked(&mut features)
        });
        features
    }

    pub fn get_queue_family_properties(&self) -> Vec<VkQueueFamilyProperties> {
        let mut count = 0;
        (self
            .instance
            .physical_device_functions()
            .get_physical_device_queue_family_properties)(
            self.inner,
            unsafe { NonNull::new_unchecked(&mut count) },
            None,
        );

        if count == 0 {
            return Vec::new();
        }

        let mut queue_families = Vec::with_capacity(count as usize);
        (self
            .instance
            .physical_device_functions()
            .get_physical_device_queue_family_properties)(
            self.inner,
            unsafe { NonNull::new_unchecked(&mut count) },
            Some(unsafe { NonNull::new_unchecked(queue_families.as_mut_ptr()) }),
        );

        unsafe { queue_families.set_len(count as usize) };
        queue_families
    }
}

impl VkPhysicalDeviceFunctions {
    pub(crate) fn get<L: Loader>(instance: bindings::VkInstance, loader: &L) -> Result<Self> {
        let get_physical_device_properties =
            get_instance_proc_addr!(loader, Some(instance), "vkGetPhysicalDeviceProperties")?;
        let get_physical_device_features =
            get_instance_proc_addr!(loader, Some(instance), "vkGetPhysicalDeviceFeatures")?;
        let get_physical_device_queue_family_properties = get_instance_proc_addr!(
            loader,
            Some(instance),
            "vkGetPhysicalDeviceQueueFamilyProperties"
        )?;

        Ok(VkPhysicalDeviceFunctions {
            get_physical_device_properties,
            get_physical_device_features,
            get_physical_device_queue_family_properties,
        })
    }
}
