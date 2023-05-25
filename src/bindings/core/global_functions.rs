use crate::{
    VkAllocationCallbacks, VkExtensionProperties, VkInstance, VkInstanceCreateInfo,
    VkLayerProperties, VkResult,
};
use std::ffi::c_char;

pub(crate) type VkCreateInstance = extern "system" fn(
    p_create_info: *const VkInstanceCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_instance: *mut Option<VkInstance>,
) -> VkResult;

pub(crate) type VkEnumerateInstanceExtensionProperties = extern "system" fn(
    p_layer_name: *const c_char,
    p_property_count: *mut u32,
    p_properties: *mut VkExtensionProperties,
) -> VkResult;

pub(crate) type VkEnumerateInstanceLayerProperties = extern "system" fn(
    p_property_count: *mut u32,
    p_properties: *mut VkLayerProperties,
) -> VkResult;
