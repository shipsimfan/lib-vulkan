use crate::{bindings::VkResult, VkExtensionProperties, VkLayerProperties};

pub type VkEnumerateInstanceLayerProperties = extern "system" fn(
    p_property_count: *mut u32,
    p_properties: *mut VkLayerProperties,
) -> VkResult;

pub type VkEnumerateInstanceExtensionProperties = extern "system" fn(
    p_layer_name: *const u8,
    p_property_count: *mut u32,
    p_properties: *mut VkExtensionProperties,
) -> VkResult;
