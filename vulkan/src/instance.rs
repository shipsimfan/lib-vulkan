use crate::{
    bindings::{self, VkDestroyInstance, VkEnumeratePhysicalDevices, VkGetDeviceProcAddr},
    get_instance_proc_addr, Result, VkPhysicalDevice, VkPhysicalDeviceFunctions, VkResult,
};
use loader::Loader;
use native::NativeLoader;
use std::{ptr::NonNull, sync::Arc};

pub struct VkInstance<L: Loader = NativeLoader> {
    inner: bindings::VkInstance,
    _loader: Arc<L>,

    // Direct instance functions
    destroy_instance: VkDestroyInstance,
    enumerate_physical_devices: VkEnumeratePhysicalDevices,

    // Special device function
    get_device_proc_addr: VkGetDeviceProcAddr,

    // Indirect instance functions
    physical_device_functions: VkPhysicalDeviceFunctions,
}

impl<L: Loader> VkInstance<L> {
    pub(crate) fn new(inner: bindings::VkInstance, loader: Arc<L>) -> Result<Arc<Self>> {
        let destroy_instance = get_instance_proc_addr!(loader, Some(inner), "vkDestroyInstance")?;
        let enumerate_physical_devices =
            get_instance_proc_addr!(loader, Some(inner), "vkEnumeratePhysicalDevices")?;

        let get_device_proc_addr =
            get_instance_proc_addr!(loader, Some(inner), "vkGetDeviceProcAddr")?;

        let physical_device_functions = VkPhysicalDeviceFunctions::get(inner, loader.as_ref())?;

        Ok(Arc::new(VkInstance {
            inner,
            _loader: loader,

            destroy_instance,
            enumerate_physical_devices,

            get_device_proc_addr,

            physical_device_functions,
        }))
    }

    pub fn enumerate_physical_devices(self: &Arc<Self>) -> Result<Vec<VkPhysicalDevice<L>>> {
        let mut count = 0;
        match (self.enumerate_physical_devices)(
            self.inner,
            unsafe { NonNull::new_unchecked(&mut count) },
            None,
        ) {
            VkResult::Success => {}
            result => return Err(result),
        }

        if count == 0 {
            return Ok(Vec::new());
        }

        let mut physical_devices = Vec::with_capacity(count as usize);
        match (self.enumerate_physical_devices)(
            self.inner,
            unsafe { NonNull::new_unchecked(&mut count) },
            Some(unsafe { NonNull::new_unchecked(physical_devices.as_mut_ptr()) }),
        ) {
            VkResult::Success | VkResult::Incomplete => {
                unsafe { physical_devices.set_len(count as usize) };
                Ok(physical_devices
                    .into_iter()
                    .map(|physical_device| VkPhysicalDevice::new(physical_device, self.clone()))
                    .collect())
            }
            result => Err(result),
        }
    }

    pub(crate) fn get_device_proc_addr(&self) -> VkGetDeviceProcAddr {
        self.get_device_proc_addr
    }

    pub(crate) fn physical_device_functions(&self) -> &VkPhysicalDeviceFunctions {
        &self.physical_device_functions
    }
}

impl<L: Loader> Drop for VkInstance<L> {
    fn drop(&mut self) {
        (self.destroy_instance)(self.inner, None);
    }
}
