use crate::{
    Device, DeviceCreateInfo, ExtensionProperties, Instance, Loader, NativeLoader, PresentMode,
    QueueFamilyProperties, Result, Surface, SurfaceCapabilities, SurfaceFormat, VkPhysicalDevice,
    VkPhysicalDeviceFeatures, VkPhysicalDeviceProperties, VkResult,
};
use std::{
    ffi::{CStr, CString},
    ptr::{null, null_mut},
    sync::Arc,
};

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

    pub fn enumerate_extension_properties(
        &self,
        layer_name: Option<&str>,
    ) -> Result<Vec<ExtensionProperties>> {
        let layer_name = layer_name.map(|name| CString::new(name).unwrap());
        let p_layer_name = layer_name
            .as_ref()
            .map(|name| name.as_ptr())
            .unwrap_or(null());

        let mut count = 0;
        match (self
            .instance
            .physical_device_functions()
            .enumerate_extension_properties)(
            self.handle, p_layer_name, &mut count, null_mut()
        ) {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut extensions = Vec::with_capacity(count as usize);
        loop {
            match (self
                .instance
                .physical_device_functions()
                .enumerate_extension_properties)(
                self.handle,
                p_layer_name,
                &mut count,
                extensions.as_mut_ptr(),
            ) {
                VkResult::Success => {
                    unsafe { extensions.set_len(count as usize) };
                    return Ok(extensions
                        .into_iter()
                        .map(|extension| ExtensionProperties {
                            name: CStr::from_bytes_until_nul(&extension.extension_name)
                                .unwrap()
                                .to_string_lossy()
                                .to_string(),
                            spec_version: extension.spec_version.into(),
                        })
                        .collect());
                }
                VkResult::Incomplete => {
                    extensions.reserve(count as usize);
                }
                result => return Err(result),
            }
        }
    }

    pub fn get_surface_capabilities(&self, surface: &Surface<L>) -> Result<SurfaceCapabilities> {
        let mut surface_capabilites = SurfaceCapabilities::default();
        match (self
            .instance
            .surface_functions()
            .get_physical_device_surface_capabilities)(
            self.handle,
            surface.handle(),
            &mut surface_capabilites,
        ) {
            VkResult::Success => Ok(surface_capabilites),
            result => Err(result),
        }
    }

    pub fn get_surface_formats(&self, surface: &Surface<L>) -> Result<Vec<SurfaceFormat>> {
        let mut count = 0;
        match (self
            .instance
            .surface_functions()
            .get_physical_device_surface_formats)(
            self.handle,
            surface.handle(),
            &mut count,
            null_mut(),
        ) {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut extensions = Vec::with_capacity(count as usize);
        loop {
            match (self
                .instance
                .surface_functions()
                .get_physical_device_surface_formats)(
                self.handle,
                surface.handle(),
                &mut count,
                extensions.as_mut_ptr(),
            ) {
                VkResult::Success => {
                    unsafe { extensions.set_len(count as usize) };
                    return Ok(extensions);
                }
                VkResult::Incomplete => {
                    extensions.reserve(count as usize);
                }
                result => return Err(result),
            }
        }
    }

    pub fn get_surface_present_modes(&self, surface: &Surface<L>) -> Result<Vec<PresentMode>> {
        let mut count = 0;
        match (self
            .instance
            .surface_functions()
            .get_physical_device_surface_present_modes)(
            self.handle,
            surface.handle(),
            &mut count,
            null_mut(),
        ) {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut present_modes = Vec::with_capacity(count as usize);
        loop {
            match (self
                .instance
                .surface_functions()
                .get_physical_device_surface_present_modes)(
                self.handle,
                surface.handle(),
                &mut count,
                present_modes.as_mut_ptr(),
            ) {
                VkResult::Success => {
                    unsafe { present_modes.set_len(count as usize) };
                    return Ok(present_modes);
                }
                VkResult::Incomplete => {
                    present_modes.reserve(count as usize);
                }
                result => return Err(result),
            }
        }
    }

    pub fn get_surface_support(
        &self,
        queue_family_index: u32,
        surface: &Surface<L>,
    ) -> Result<bool> {
        let mut result = 0;
        match (self
            .instance
            .surface_functions()
            .get_physical_device_surface_support)(
            self.handle,
            queue_family_index,
            surface.handle(),
            &mut result,
        ) {
            VkResult::Success => Ok(result != 0),
            result => Err(result),
        }
    }
}
