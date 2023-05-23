use crate::{
    bindings::{VkDevice, VkPhysicalDevice, VkPhysicalDeviceProperties},
    VkDeviceCreateInfo, VkPhysicalDeviceFeatures, VkQueueFamilyProperties, VkResult,
};
use std::{ffi::c_void, ptr::NonNull};

pub type VkGetPhysicalDeviceProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_properties: &mut VkPhysicalDeviceProperties,
);

pub type VkGetPhysicalDeviceFeatures = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_features: &mut VkPhysicalDeviceFeatures,
);

pub type VkGetPhysicalDeviceQueueFamilyProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_queue_family_property_count: &mut u32,
    p_queue_family_properties: Option<NonNull<VkQueueFamilyProperties>>,
);

pub type VkCreateDevice = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_create_info: &VkDeviceCreateInfo,
    p_allocator: Option<NonNull<c_void>>,
    p_device: &mut Option<VkDevice>,
) -> VkResult;
