use crate::{bindings::VkResult, VkLayerProperties};

pub type VkEnumerateInstanceLayerProperties = extern "system" fn(
    p_property_count: *mut u32,
    p_properties: *mut VkLayerProperties,
) -> VkResult;
