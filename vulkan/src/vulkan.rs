use crate::{
    assert_null_terminated,
    bindings::{
        VkCreateInstance, VkEnumerateInstanceExtensionProperties,
        VkEnumerateInstanceLayerProperties, VkEnumerateInstanceVersion,
    },
    get_instance_proc_addr, get_instance_proc_addr_opt, Loader, Result, VkExtensionProperties,
    VkInstance, VkInstanceCreateInfo, VkLayerProperties, VkResult, VkVersion,
};
use std::ptr::{null, null_mut};

pub struct Vulkan<L: Loader> {
    loader: L,

    create_instance: VkCreateInstance,
    enumerate_instance_layer_properties: VkEnumerateInstanceLayerProperties,
    enumerate_instance_extension_properties: VkEnumerateInstanceExtensionProperties,
    enumerate_instance_version: Option<VkEnumerateInstanceVersion>,
}

impl<L: Loader> Vulkan<L> {
    pub fn new(loader: L) -> Result<Self> {
        let create_instance = get_instance_proc_addr!(loader, None, "vkCreateInstance")?;

        let enumerate_instance_layer_properties =
            get_instance_proc_addr!(loader, None, "vkEnumerateInstanceLayerProperties")?;

        let enumerate_instance_extension_properties =
            get_instance_proc_addr!(loader, None, "vkEnumerateInstanceExtensionProperties")?;

        let enumerate_instance_version =
            get_instance_proc_addr_opt!(loader, None, "vkEnumerateInstanceVersion");

        Ok(Vulkan {
            loader,
            create_instance,
            enumerate_instance_layer_properties,
            enumerate_instance_extension_properties,
            enumerate_instance_version,
        })
    }

    pub fn create_instance(&self, create_info: &VkInstanceCreateInfo) -> Result<VkInstance> {
        let mut vk_instance = None;
        match (self.create_instance)(create_info, null(), &mut vk_instance) {
            VkResult::Success => VkInstance::new(vk_instance.unwrap(), &self.loader),
            result => Err(result),
        }
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
        if let Some(layer_name) = layer_name {
            assert_null_terminated!(layer_name);
        }

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

    pub fn enumerate_instance_version(&self) -> Result<VkVersion> {
        if let Some(enumerate_instance_version) = self.enumerate_instance_version {
            let mut version = VkVersion::new(0, 0, 0, 0);
            match (enumerate_instance_version)(&mut version) {
                VkResult::Success => Ok(version),
                result => Err(result),
            }
        } else {
            Err(VkResult::ErrorIncompatibleDriver)
        }
    }
}
