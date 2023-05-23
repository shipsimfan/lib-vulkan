use crate::{
    bindings::{VkInstance, VkInstanceCreateInfo, VkResult},
    VkExtensionProperties, VkLayerProperties, VkVersion,
};
use std::{
    ffi::{c_char, c_void},
    ptr::NonNull,
};

pub type VkEnumerateInstanceLayerProperties = extern "system" fn(
    p_property_count: &mut u32,
    p_properties: Option<NonNull<VkLayerProperties>>,
) -> VkResult;

pub type VkEnumerateInstanceExtensionProperties = extern "system" fn(
    p_layer_name: Option<*const c_char>,
    p_property_count: &mut u32,
    p_properties: Option<NonNull<VkExtensionProperties>>,
) -> VkResult;

pub type VkEnumerateInstanceVersion = extern "system" fn(p_api_version: &mut VkVersion) -> VkResult;

pub type VkCreateInstance = extern "system" fn(
    p_create_info: &VkInstanceCreateInfo,
    p_allocator: Option<NonNull<c_void>>,
    p_instance: &mut Option<VkInstance>,
) -> VkResult;
