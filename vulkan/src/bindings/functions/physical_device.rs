use crate::{
    bindings::{VkPhysicalDevice, VkPhysicalDeviceProperties},
    VkPhysicalDeviceFeatures, VkQueueFamilyProperties,
};
use std::ptr::NonNull;

pub type VkGetPhysicalDeviceProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_properties: NonNull<VkPhysicalDeviceProperties>,
);

pub type VkGetPhysicalDeviceFeatures = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_features: NonNull<VkPhysicalDeviceFeatures>,
);

pub type VkGetPhysicalDeviceQueueFamilyProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_queue_family_property_count: NonNull<u32>,
    p_queue_family_properties: Option<NonNull<VkQueueFamilyProperties>>,
);
