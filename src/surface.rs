use crate::{
    bindings::{
        self, VkDestroySurfaceKHR, VkGetPhysicalDeviceSurfaceCapabilitiesKHR,
        VkGetPhysicalDeviceSurfaceFormatsKHR, VkGetPhysicalDeviceSurfacePresentModesKHR,
        VkGetPhysicalDeviceSurfaceSupportKHR,
    },
    get_instance_proc_addr, Loader, NativeLoader, Result, VkInstance, VkPhysicalDevice,
    VkPresentModeKHR, VkResult, VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR,
};
use std::{ptr::NonNull, sync::Arc};

pub struct VkSurfaceKHR<L: Loader = NativeLoader> {
    inner: bindings::VkSurfaceKHR,
    instance: Arc<VkInstance<L>>,

    destroy_surface: VkDestroySurfaceKHR,
    get_physical_device_surface_capabilities: VkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    get_physical_device_surface_formats: VkGetPhysicalDeviceSurfaceFormatsKHR,
    get_physical_device_surface_present_modes: VkGetPhysicalDeviceSurfacePresentModesKHR,
    get_physical_device_surface_support: VkGetPhysicalDeviceSurfaceSupportKHR,
}

impl<L: Loader> VkSurfaceKHR<L> {
    pub(crate) fn new(inner: bindings::VkSurfaceKHR, instance: Arc<VkInstance<L>>) -> Result<Self> {
        let destroy_surface = get_instance_proc_addr!(
            instance.loader(),
            Some(instance.inner()),
            "vkDestroySurfaceKHR"
        )?;
        let get_physical_device_surface_capabilities = get_instance_proc_addr!(
            instance.loader(),
            Some(instance.inner()),
            "vkGetPhysicalDeviceSurfaceCapabilitiesKHR"
        )?;
        let get_physical_device_surface_formats = get_instance_proc_addr!(
            instance.loader(),
            Some(instance.inner()),
            "vkGetPhysicalDeviceSurfaceFormatsKHR"
        )?;
        let get_physical_device_surface_present_modes = get_instance_proc_addr!(
            instance.loader(),
            Some(instance.inner()),
            "vkGetPhysicalDeviceSurfacePresentModesKHR"
        )?;
        let get_physical_device_surface_support = get_instance_proc_addr!(
            instance.loader(),
            Some(instance.inner()),
            "vkGetPhysicalDeviceSurfaceSupportKHR"
        )?;

        Ok(VkSurfaceKHR {
            inner,
            instance,

            destroy_surface,
            get_physical_device_surface_capabilities,
            get_physical_device_surface_formats,
            get_physical_device_surface_present_modes,
            get_physical_device_surface_support,
        })
    }

    pub fn get_physical_device_surface_capabilities(
        &self,
        physical_device: &VkPhysicalDevice<L>,
    ) -> Result<VkSurfaceCapabilitiesKHR> {
        assert_eq!(self.instance.inner(), physical_device.instance().inner());

        let mut surface_capabilities = VkSurfaceCapabilitiesKHR::null();
        match (self.get_physical_device_surface_capabilities)(
            physical_device.inner(),
            self.inner,
            unsafe { NonNull::new_unchecked(&mut surface_capabilities) },
        ) {
            VkResult::Success => Ok(surface_capabilities),
            result => Err(result),
        }
    }

    pub fn get_physical_device_surface_formats(
        &self,
        physical_device: &VkPhysicalDevice<L>,
    ) -> Result<Vec<VkSurfaceFormatKHR>> {
        assert_eq!(self.instance.inner(), physical_device.instance().inner());

        let mut count = 0;
        match (self.get_physical_device_surface_formats)(
            physical_device.inner(),
            self.inner,
            unsafe { NonNull::new_unchecked(&mut count) },
            None,
        ) {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut surface_formats = Vec::with_capacity(count as usize);
        match (self.get_physical_device_surface_formats)(
            physical_device.inner(),
            self.inner,
            unsafe { NonNull::new_unchecked(&mut count) },
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
        assert_eq!(self.instance.inner(), physical_device.instance().inner());

        let mut count = 0;
        match (self.get_physical_device_surface_present_modes)(
            physical_device.inner(),
            self.inner,
            unsafe { NonNull::new_unchecked(&mut count) },
            None,
        ) {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut present_modes = Vec::with_capacity(count as usize);
        match (self.get_physical_device_surface_present_modes)(
            physical_device.inner(),
            self.inner,
            unsafe { NonNull::new_unchecked(&mut count) },
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
        let mut supported = 0;
        match (self.get_physical_device_surface_support)(
            physical_device.inner(),
            queue_family_index,
            self.inner,
            unsafe { NonNull::new_unchecked(&mut supported) },
        ) {
            VkResult::Success => Ok(supported != 0),
            result => Err(result),
        }
    }
}

impl<L: Loader> Drop for VkSurfaceKHR<L> {
    fn drop(&mut self) {
        (self.destroy_surface)(self.instance.inner(), self.inner, None);
    }
}
