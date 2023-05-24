use crate::{
    bindings::{VkDevice, VkPhysicalDevice, VkQueue},
    VkExtensionProperties, VkResult,
};
use std::{
    ffi::{c_char, c_void},
    ptr::NonNull,
};

pub type VkGetDeviceProcAddr =
    extern "system" fn(device: VkDevice, p_name: *const c_char) -> Option<extern "system" fn()>;

pub type VkDestroyDevice =
    extern "system" fn(device: VkDevice, p_allocator: Option<NonNull<c_void>>);

pub type VkGetDeviceQueue = extern "system" fn(
    device: VkDevice,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: &mut Option<VkQueue>,
);

pub type VkEnumerateDeviceExtensionProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_layer_name: *const c_char,
    p_property_count: &mut u32,
    p_properties: Option<NonNull<VkExtensionProperties>>,
) -> VkResult;
