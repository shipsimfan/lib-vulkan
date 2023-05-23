use crate::{
    bindings::{
        self, VkDestroyInstance, VkEnumeratePhysicalDevices, VkGetDeviceProcAddr,
        VK_KHR_SURFACE_EXTENSION_NAME,
    },
    get_instance_proc_addr,
    surface::VkSurfaceKHRFunctions,
    Loader, NativeLoader, Result, VkPhysicalDevice, VkPhysicalDeviceFunctions, VkResult,
};
use std::{
    ffi::{c_char, CStr},
    ptr::NonNull,
    sync::Arc,
};

pub struct VkInstance<L: Loader = NativeLoader> {
    inner: bindings::VkInstance,
    #[allow(unused)]
    loader: Arc<L>,

    // Direct instance functions
    destroy_instance: VkDestroyInstance,
    enumerate_physical_devices: VkEnumeratePhysicalDevices,

    // Special device function
    get_device_proc_addr: VkGetDeviceProcAddr,

    // Indirect instance functions
    physical_device_functions: VkPhysicalDeviceFunctions,
    surface_functions: Option<VkSurfaceKHRFunctions>,
}

impl<L: Loader> VkInstance<L> {
    pub(crate) fn new(
        inner: bindings::VkInstance,
        loader: Arc<L>,
        extensions: Option<&[*const c_char]>,
    ) -> Result<Arc<Self>> {
        // Get Instance functions
        let destroy_instance = get_instance_proc_addr!(loader, Some(inner), "vkDestroyInstance")?;
        let enumerate_physical_devices =
            get_instance_proc_addr!(loader, Some(inner), "vkEnumeratePhysicalDevices")?;
        let get_device_proc_addr =
            get_instance_proc_addr!(loader, Some(inner), "vkGetDeviceProcAddr")?;

        // Get core non-instance functions
        let physical_device_functions = VkPhysicalDeviceFunctions::get(inner, loader.as_ref())?;

        // Get extension functions
        let mut surface_functions = None;
        if let Some(extensions) = extensions {
            for extension in extensions {
                let extension = unsafe { CStr::from_ptr(*extension) };

                if extension == VK_KHR_SURFACE_EXTENSION_NAME {
                    surface_functions = Some(VkSurfaceKHRFunctions::get(inner, loader.as_ref())?);
                }
            }
        }

        Ok(Arc::new(VkInstance {
            inner,
            loader,

            destroy_instance,
            enumerate_physical_devices,

            get_device_proc_addr,

            physical_device_functions,
            surface_functions,
        }))
    }

    pub fn enumerate_physical_devices(self: &Arc<Self>) -> Result<Vec<VkPhysicalDevice<L>>> {
        let mut count = 0;
        match (self.enumerate_physical_devices)(self.inner, &mut count, None) {
            VkResult::Success => {}
            result => return Err(result),
        }

        if count == 0 {
            return Ok(Vec::new());
        }

        let mut physical_devices = Vec::with_capacity(count as usize);
        match (self.enumerate_physical_devices)(
            self.inner,
            &mut count,
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

    pub(crate) fn surface_functions(&self) -> Option<&VkSurfaceKHRFunctions> {
        self.surface_functions.as_ref()
    }

    pub(crate) fn inner(&self) -> bindings::VkInstance {
        self.inner
    }
}

impl<L: Loader> Drop for VkInstance<L> {
    fn drop(&mut self) {
        (self.destroy_instance)(self.inner, None);
    }
}
