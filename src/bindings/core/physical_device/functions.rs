use crate::{
    VkPhysicalDevice, VkPhysicalDeviceFeatures, VkPhysicalDeviceProperties, VkQueueFamilyProperties,
};

pub(crate) type VkGetPhysicalDeviceFeatures = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_features: *mut VkPhysicalDeviceFeatures,
);

pub(crate) type VkGetPhysicalDeviceProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_properties: *mut VkPhysicalDeviceProperties,
);

pub(crate) type VkGetPhysicalDeviceQueueFamilyProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut VkQueueFamilyProperties,
);
