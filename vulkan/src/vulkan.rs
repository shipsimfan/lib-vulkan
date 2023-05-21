use crate::{
    bindings::{VkEnumerateInstanceExtensionProperties, VkEnumerateInstanceLayerProperties},
    Loader, Result, VkExtensionProperties, VkLayerProperties, VkResult,
};
use std::ptr::{null, null_mut};

pub struct Vulkan<L: Loader> {
    _loader: L,

    enumerate_instance_layer_properties: VkEnumerateInstanceLayerProperties,
    enumerate_instance_extension_properties: VkEnumerateInstanceExtensionProperties,
}

impl<L: Loader> Vulkan<L> {
    pub fn new(loader: L) -> Option<Self> {
        let enumerate_instance_layer_properties = unsafe {
            std::mem::transmute(
                loader.get_instance_proc_addr(None, "vkEnumerateInstanceLayerProperties\0")?,
            )
        };

        let enumerate_instance_extension_properties = unsafe {
            std::mem::transmute(
                loader.get_instance_proc_addr(None, "vkEnumerateInstanceExtensionProperties\0")?,
            )
        };

        Some(Vulkan {
            _loader: loader,
            enumerate_instance_layer_properties,
            enumerate_instance_extension_properties,
        })
    }

    pub fn enumerate_instance_layer_properties(&self) -> Result<Vec<VkLayerProperties>> {
        let mut count = 0;
        match (self.enumerate_instance_layer_properties)(&mut count, null_mut()) {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut layers = Vec::with_capacity(count as usize);
        match (self.enumerate_instance_layer_properties)(&mut count, layers.as_mut_ptr()) {
            VkResult::Success | VkResult::Incomplete => {
                unsafe { layers.set_len(count as usize) };
                Ok(layers)
            }
            result => Err(result),
        }
    }

    pub fn enumerate_instance_extension_properties(
        &self,
        layer_name: Option<&str>,
    ) -> Result<Vec<VkExtensionProperties>> {
        let p_layer_name = layer_name.map(|str| str.as_ptr()).unwrap_or(null());

        let mut count = 0;
        match (self.enumerate_instance_extension_properties)(p_layer_name, &mut count, null_mut()) {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut layers = Vec::with_capacity(count as usize);
        match (self.enumerate_instance_extension_properties)(
            p_layer_name,
            &mut count,
            layers.as_mut_ptr(),
        ) {
            VkResult::Success | VkResult::Incomplete => {
                unsafe { layers.set_len(count as usize) };
                Ok(layers)
            }
            result => Err(result),
        }
    }
}
