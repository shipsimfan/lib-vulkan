use std::ptr::null_mut;

use crate::{
    bindings::VkEnumerateInstanceLayerProperties, Loader, Result, VkLayerProperties, VkResult,
};

pub struct Vulkan<L: Loader> {
    _loader: L,

    enumerate_instance_layer_properties: VkEnumerateInstanceLayerProperties,
}

impl<L: Loader> Vulkan<L> {
    pub fn new(loader: L) -> Option<Self> {
        let enumerate_instance_layer_properties = unsafe {
            std::mem::transmute(
                loader.get_instance_proc_addr(None, "vkEnumerateInstanceLayerProperties\0")?,
            )
        };

        Some(Vulkan {
            _loader: loader,
            enumerate_instance_layer_properties,
        })
    }

    pub fn enumerate_instance_layer_properties(&self) -> Result<Vec<VkLayerProperties>> {
        let mut count = 0;
        let mut layers = Vec::new();

        loop {
            let old_count = count;

            match (self.enumerate_instance_layer_properties)(
                &mut count,
                if count == 0 {
                    null_mut()
                } else {
                    layers.as_mut_ptr()
                },
            ) {
                VkResult::Success => {
                    if old_count == 0 {
                        layers.reserve(count as usize);
                    } else {
                        unsafe { layers.set_len(count as usize) };
                        return Ok(layers);
                    }
                }
                VkResult::Incomplete => {
                    layers.reserve(count as usize);
                }
                result => return Err(result),
            }
        }
    }
}
