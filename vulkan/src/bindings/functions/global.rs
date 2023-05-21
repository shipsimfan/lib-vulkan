use crate::{
    bindings::{VkInstance, VkInstanceCreateInfo, VkResult},
    VkExtensionProperties, VkLayerProperties, VkVersion,
};
use std::ffi::c_void;

pub type VkEnumerateInstanceLayerProperties = extern "system" fn(
    p_property_count: *mut u32,
    p_properties: *mut VkLayerProperties,
) -> VkResult;

pub type VkEnumerateInstanceExtensionProperties = extern "system" fn(
    p_layer_name: *const u8,
    p_property_count: *mut u32,
    p_properties: *mut VkExtensionProperties,
) -> VkResult;

pub type VkEnumerateInstanceVersion = extern "system" fn(p_api_version: *mut VkVersion) -> VkResult;

pub type VkCreateInstance = extern "system" fn(
    p_create_info: *const VkInstanceCreateInfo,
    p_allocator: *const c_void,
    p_instance: *mut Option<VkInstance>,
) -> VkResult;
