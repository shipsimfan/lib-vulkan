use crate::{
    bindings::{VkInstance, VkInstanceCreateInfo, VkResult},
    VkExtensionProperties, VkLayerProperties, VkVersion,
};
use std::{ffi::c_void, ptr::NonNull};

pub type VkEnumerateInstanceLayerProperties = extern "system" fn(
    p_property_count: NonNull<u32>,
    p_properties: Option<NonNull<VkLayerProperties>>,
) -> VkResult;

pub type VkEnumerateInstanceExtensionProperties = extern "system" fn(
    p_layer_name: Option<NonNull<u8>>,
    p_property_count: NonNull<u32>,
    p_properties: Option<NonNull<VkExtensionProperties>>,
) -> VkResult;

pub type VkEnumerateInstanceVersion =
    extern "system" fn(p_api_version: NonNull<VkVersion>) -> VkResult;

pub type VkCreateInstance = extern "system" fn(
    p_create_info: NonNull<VkInstanceCreateInfo>,
    p_allocator: Option<NonNull<c_void>>,
    p_instance: NonNull<Option<VkInstance>>,
) -> VkResult;
