use crate::{
    VkAllocationCallbacks, VkDevice, VkExtensionProperties, VkPhysicalDevice, VkQueue, VkResult,
};
use std::ffi::c_char;

pub(crate) type VkDestroyDevice =
    extern "system" fn(device: VkDevice, p_allocator: *const VkAllocationCallbacks);

pub(crate) type VkEnumerateDeviceExtensionProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_layer_name: *const c_char,
    p_property_count: *mut u32,
    p_properties: *mut VkExtensionProperties,
) -> VkResult;

pub(crate) type VkGetDeviceQueue = extern "system" fn(
    device: VkDevice,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: *mut Option<VkQueue>,
);
