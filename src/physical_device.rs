use crate::{
    bindings::{
        self, VkCreateDevice, VkEnumerateDeviceExtensionProperties, VkGetPhysicalDeviceFeatures,
        VkGetPhysicalDeviceProperties, VkGetPhysicalDeviceQueueFamilyProperties,
        VkPhysicalDeviceProperties,
    },
    get_instance_proc_addr, Loader, NativeLoader, Result, VkDevice, VkDeviceCreateInfo,
    VkExtensionProperties, VkInstance, VkPhysicalDeviceFeatures, VkQueueFamilyProperties, VkResult,
    VK_KHR_SWAPCHAIN_EXTENSION_NAME,
};
use std::{
    ffi::CStr,
    ptr::{null, NonNull},
    sync::Arc,
};

pub struct VkPhysicalDevice<L: Loader = NativeLoader> {
    inner: bindings::VkPhysicalDevice,
    instance: Arc<VkInstance<L>>,
}

pub(crate) struct VkPhysicalDeviceFunctions {
    get_physical_device_properties: VkGetPhysicalDeviceProperties,
    get_physical_device_features: VkGetPhysicalDeviceFeatures,
    get_physical_device_queue_family_properties: VkGetPhysicalDeviceQueueFamilyProperties,
    enumerate_device_extension_properties: VkEnumerateDeviceExtensionProperties,
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
            .get_physical_device_properties)(self.inner, &mut properties);
        properties
    }

    pub fn get_features(&self) -> VkPhysicalDeviceFeatures {
        let mut features = VkPhysicalDeviceFeatures::default();
        (self
            .instance
            .physical_device_functions()
            .get_physical_device_features)(self.inner, &mut features);
        features
    }

    pub fn get_queue_family_properties(&self) -> Vec<VkQueueFamilyProperties> {
        let mut count = 0;
        (self
            .instance
            .physical_device_functions()
            .get_physical_device_queue_family_properties)(self.inner, &mut count, None);

        if count == 0 {
            return Vec::new();
        }

        let mut queue_families = Vec::with_capacity(count as usize);
        (self
            .instance
            .physical_device_functions()
            .get_physical_device_queue_family_properties)(
            self.inner,
            &mut count,
            Some(unsafe { NonNull::new_unchecked(queue_families.as_mut_ptr()) }),
        );

        unsafe { queue_families.set_len(count as usize) };
        queue_families
    }

    pub fn create_device(&self, create_info: &VkDeviceCreateInfo) -> Result<Arc<VkDevice<L>>> {
        let mut swapchain_enabled = false;
        if let Some(extensions) = create_info.enabled_extensions() {
            for extension in extensions {
                let extension = unsafe { CStr::from_ptr(*extension) };

                if extension == VK_KHR_SWAPCHAIN_EXTENSION_NAME {
                    swapchain_enabled = true;
                    break;
                }
            }
        }

        let mut device = None;
        match (self.instance.physical_device_functions().create_device)(
            self.inner,
            &create_info,
            None,
            &mut device,
        ) {
            VkResult::Success => {
                VkDevice::new(device.unwrap(), self.instance.clone(), swapchain_enabled)
            }
            result => Err(result),
        }
    }

    pub fn get_physical_device_win32_presentation_support(&self, queue_family_index: u32) -> bool {
        (self
            .instance
            .win32_surface_functions()
            .unwrap()
            .get_physical_device_win32_presentation_support)(self.inner, queue_family_index)
            != 0
    }

    pub fn enumerate_device_extension_properties(
        &self,
        layer_name: Option<&CStr>,
    ) -> Result<Vec<VkExtensionProperties>> {
        let mut count = 0;
        match (self
            .instance()
            .physical_device_functions()
            .enumerate_device_extension_properties)(
            self.inner,
            layer_name
                .map(|layer_name| layer_name.as_ptr())
                .unwrap_or(null()),
            &mut count,
            None,
        ) {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut extensions = Vec::with_capacity(count as usize);
        match (self
            .instance
            .physical_device_functions()
            .enumerate_device_extension_properties)(
            self.inner,
            layer_name
                .map(|layer_name| layer_name.as_ptr())
                .unwrap_or(null()),
            &mut count,
            Some(unsafe { NonNull::new_unchecked(extensions.as_mut_ptr()) }),
        ) {
            VkResult::Success | VkResult::Incomplete => {
                unsafe { extensions.set_len(count as usize) };
                Ok(extensions)
            }
            result => Err(result),
        }
    }

    pub(crate) fn instance(&self) -> &Arc<VkInstance<L>> {
        &self.instance
    }

    pub(crate) fn inner(&self) -> bindings::VkPhysicalDevice {
        self.inner
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
        let enumerate_device_extension_properties = get_instance_proc_addr!(
            loader,
            Some(instance),
            "vkEnumerateDeviceExtensionProperties"
        )?;

        Ok(VkPhysicalDeviceFunctions {
            get_physical_device_properties,
            get_physical_device_features,
            get_physical_device_queue_family_properties,
            enumerate_device_extension_properties,
            create_device,
        })
    }
}
