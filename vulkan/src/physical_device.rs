use crate::{
    bindings::{
        self, VkCreateDevice, VkGetPhysicalDeviceFeatures, VkGetPhysicalDeviceProperties,
        VkGetPhysicalDeviceQueueFamilyProperties, VkPhysicalDeviceProperties,
    },
    get_instance_proc_addr, Result, VkDevice, VkDeviceCreateInfo, VkInstance,
    VkPhysicalDeviceFeatures, VkQueueFamilyProperties, VkResult,
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
    create_device: VkCreateDevice,
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
        let mut features = VkPhysicalDeviceFeatures::default();
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

    pub fn create_device(&self, create_info: &VkDeviceCreateInfo) -> Result<VkDevice<L>> {
        let mut device = None;
        match (self.instance.physical_device_functions().create_device)(
            self.inner,
            unsafe { NonNull::new_unchecked(create_info as *const _ as _) },
            None,
            unsafe { NonNull::new_unchecked(&mut device) },
        ) {
            VkResult::Success => VkDevice::new(device.unwrap(), self.instance.clone()),
            result => Err(result),
        }
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
        let create_device = get_instance_proc_addr!(loader, Some(instance), "vkCreateDevice")?;

        Ok(VkPhysicalDeviceFunctions {
            get_physical_device_properties,
            get_physical_device_features,
            get_physical_device_queue_family_properties,
            create_device,
        })
    }
}
