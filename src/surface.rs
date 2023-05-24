use crate::{
    bindings::{
        self, VkCreateWin32SurfaceKHR, VkDestroySurfaceKHR,
        VkGetPhysicalDeviceSurfaceCapabilitiesKHR, VkGetPhysicalDeviceSurfaceFormatsKHR,
        VkGetPhysicalDeviceSurfacePresentModesKHR, VkGetPhysicalDeviceSurfaceSupportKHR,
        VkGetPhysicalDeviceWin32PresentationSupportKHR,
    },
    get_instance_proc_addr, Loader, NativeLoader, Result, VkInstance, VkPhysicalDevice,
    VkPresentModeKHR, VkResult, VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR,
};
use std::{ptr::NonNull, sync::Arc};

pub struct VkSurfaceKHR<L: Loader = NativeLoader> {
    inner: bindings::VkSurfaceKHR,
    instance: Arc<VkInstance<L>>,
}

pub(crate) struct VkSurfaceKHRFunctions {
    destroy_surface: VkDestroySurfaceKHR,
    get_physical_device_surface_capabilities: VkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    get_physical_device_surface_formats: VkGetPhysicalDeviceSurfaceFormatsKHR,
    get_physical_device_surface_present_modes: VkGetPhysicalDeviceSurfacePresentModesKHR,
    get_physical_device_surface_support: VkGetPhysicalDeviceSurfaceSupportKHR,
}

#[cfg(target_os = "windows")]
pub(crate) struct VkWin32SurfaceKHRFunctions {
    pub(crate) create_win32_surface: VkCreateWin32SurfaceKHR,
    pub(crate) get_physical_device_win32_presentation_support:
        VkGetPhysicalDeviceWin32PresentationSupportKHR,
}

impl<L: Loader> VkSurfaceKHR<L> {
    pub(crate) fn new(inner: bindings::VkSurfaceKHR, instance: Arc<VkInstance<L>>) -> Result<Self> {
        Ok(VkSurfaceKHR { inner, instance })
    }

    pub fn get_physical_device_surface_capabilities(
        &self,
        physical_device: &VkPhysicalDevice<L>,
    ) -> Result<VkSurfaceCapabilitiesKHR> {
        assert_eq!(
            Arc::as_ptr(&self.instance),
            Arc::as_ptr(physical_device.instance())
        );

        let mut surface_capabilities = VkSurfaceCapabilitiesKHR::null();
        match (self
            .instance
            .surface_functions()
            .unwrap()
            .get_physical_device_surface_capabilities)(
            physical_device.inner(),
            self.inner,
            &mut surface_capabilities,
        ) {
            VkResult::Success => Ok(surface_capabilities),
            result => Err(result),
        }
    }

    pub fn get_physical_device_surface_formats(
        &self,
        physical_device: &VkPhysicalDevice<L>,
    ) -> Result<Vec<VkSurfaceFormatKHR>> {
        assert_eq!(
            Arc::as_ptr(&self.instance),
            Arc::as_ptr(physical_device.instance())
        );

        let mut count = 0;
        match (self
            .instance
            .surface_functions()
            .unwrap()
            .get_physical_device_surface_formats)(
            physical_device.inner(),
            self.inner,
            &mut count,
            None,
        ) {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut surface_formats = Vec::with_capacity(count as usize);
        match (self
            .instance
            .surface_functions()
            .unwrap()
            .get_physical_device_surface_formats)(
            physical_device.inner(),
            self.inner,
            &mut count,
            Some(unsafe { NonNull::new_unchecked(surface_formats.as_mut_ptr()) }),
        ) {
            VkResult::Success | VkResult::Incomplete => {
                unsafe { surface_formats.set_len(count as usize) };
                Ok(surface_formats)
            }
            result => Err(result),
        }
    }

    pub fn get_physical_device_surface_present_modes(
        &self,
        physical_device: &VkPhysicalDevice<L>,
    ) -> Result<Vec<VkPresentModeKHR>> {
        assert_eq!(
            Arc::as_ptr(&self.instance),
            Arc::as_ptr(physical_device.instance())
        );

        let mut count = 0;
        match (self
            .instance
            .surface_functions()
            .unwrap()
            .get_physical_device_surface_present_modes)(
            physical_device.inner(),
            self.inner,
            &mut count,
            None,
        ) {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut present_modes = Vec::with_capacity(count as usize);
        match (self
            .instance
            .surface_functions()
            .unwrap()
            .get_physical_device_surface_present_modes)(
            physical_device.inner(),
            self.inner,
            &mut count,
            Some(unsafe { NonNull::new_unchecked(present_modes.as_mut_ptr()) }),
        ) {
            VkResult::Success | VkResult::Incomplete => {
                unsafe { present_modes.set_len(count as usize) };
                Ok(present_modes)
            }
            result => Err(result),
        }
    }

    pub fn get_physical_device_surface_support(
        &self,
        physical_device: &VkPhysicalDevice<L>,
        queue_family_index: u32,
    ) -> Result<bool> {
        assert_eq!(
            Arc::as_ptr(&self.instance),
            Arc::as_ptr(physical_device.instance())
        );

        let mut supported = 0;
        match (self
            .instance
            .surface_functions()
            .unwrap()
            .get_physical_device_surface_support)(
            physical_device.inner(),
            queue_family_index,
            self.inner,
            &mut supported,
        ) {
            VkResult::Success => Ok(supported != 0),
            result => Err(result),
        }
    }
}

impl<L: Loader> Drop for VkSurfaceKHR<L> {
    fn drop(&mut self) {
        (self.instance.surface_functions().unwrap().destroy_surface)(
            self.instance.inner(),
            self.inner,
            None,
        );
    }
}

impl VkSurfaceKHRFunctions {
    pub(crate) fn get<L: Loader>(instance: bindings::VkInstance, loader: &L) -> Result<Self> {
        let destroy_surface =
            get_instance_proc_addr!(loader, Some(instance), "vkDestroySurfaceKHR")?;
        let get_physical_device_surface_capabilities = get_instance_proc_addr!(
            loader,
            Some(instance),
            "vkGetPhysicalDeviceSurfaceCapabilitiesKHR"
        )?;
        let get_physical_device_surface_formats = get_instance_proc_addr!(
            loader,
            Some(instance),
            "vkGetPhysicalDeviceSurfaceFormatsKHR"
        )?;
        let get_physical_device_surface_present_modes = get_instance_proc_addr!(
            loader,
            Some(instance),
            "vkGetPhysicalDeviceSurfacePresentModesKHR"
        )?;
        let get_physical_device_surface_support = get_instance_proc_addr!(
            loader,
            Some(instance),
            "vkGetPhysicalDeviceSurfaceSupportKHR"
        )?;

        Ok(VkSurfaceKHRFunctions {
            destroy_surface,
            get_physical_device_surface_capabilities,
            get_physical_device_surface_formats,
            get_physical_device_surface_present_modes,
            get_physical_device_surface_support,
        })
    }
}

#[cfg(target_os = "windows")]
impl VkWin32SurfaceKHRFunctions {
    pub(crate) fn get<L: Loader>(instance: bindings::VkInstance, loader: &L) -> Result<Self> {
        let create_win32_surface =
            get_instance_proc_addr!(loader, Some(instance), "vkCreateWin32SurfaceKHR")?;
        let get_physical_device_win32_presentation_support = get_instance_proc_addr!(
            loader,
            Some(instance),
            "vkGetPhysicalDeviceWin32PresentationSupportKHR"
        )?;

        Ok(VkWin32SurfaceKHRFunctions {
            create_win32_surface,
            get_physical_device_win32_presentation_support,
        })
    }
}
