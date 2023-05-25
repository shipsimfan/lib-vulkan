use crate::{VkAllocationCallbacks, VkExtensionProperties, VkInstance, VkResult};
use std::ffi::c_char;

pub(crate) type VkDestroyInstance =
    extern "system" fn(instance: VkInstance, p_allocator: *const VkAllocationCallbacks);

pub(crate) type VkEnumerateInstanceExtensionProperties = extern "system" fn(
    p_layer_name: *const c_char,
    p_property_count: *mut u32,
    p_properties: *mut VkExtensionProperties,
) -> VkResult;
