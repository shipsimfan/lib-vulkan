mod allocation_callbacks;
mod constants;
mod device;
mod extension_properties;
mod extent_3d;
mod global_functions;
mod instance;
mod layer_properties;
mod physical_device;
mod queue;
mod result;
mod structure_type;
mod types;
mod version;

pub use constants::VK_UUID_SIZE;
pub use extent_3d::VkExtent3D;
pub use instance::VkInstance;
pub use physical_device::{
    VkPhysicalDeviceType, VkQueueFamilyProperties, VkQueueFlagBits, VkQueueFlags,
    VkSampleCountFlagBits, VkSampleCountFlags,
};
pub use result::VkResult;
pub use version::{
    VkVersion, VK_API_VERSION_1_0, VK_API_VERSION_1_1, VK_API_VERSION_1_2, VK_API_VERSION_1_3,
    VK_HEADER_VERSION, VK_HEADER_VERSION_COMPLETE,
};

#[allow(unused)]
pub(crate) use allocation_callbacks::{
    VkAllocationCallbacks, VkAllocationFunction, VkFreeFunction, VkInternalAllocationNotification,
    VkInternalAllocationType, VkInternalFreeNotification, VkReallocationFunction,
    VkSystemAllocationScope,
};
pub(crate) use constants::{
    VK_MAX_DESCRIPTION_SIZE, VK_MAX_EXTENSION_NAME_SIZE, VK_MAX_PHYSICAL_DEVICE_NAME_SIZE,
};
pub(crate) use device::{
    VkDestroyDevice, VkDevice, VkDeviceCreateFlags, VkDeviceCreateInfo, VkDeviceQueueCreateFlags,
    VkDeviceQueueCreateInfo, VkGetDeviceQueue,
};
pub(crate) use extension_properties::VkExtensionProperties;
pub(crate) use global_functions::{
    VkCreateInstance, VkEnumerateInstanceExtensionProperties, VkEnumerateInstanceLayerProperties,
};
pub(crate) use instance::{
    VkApplicationInfo, VkDestroyInstance, VkEnumeratePhysicalDevice, VkGetDeviceProcAddr,
    VkInstanceCreateFlags, VkInstanceCreateInfo,
};
pub(crate) use layer_properties::VkLayerProperties;
pub(crate) use physical_device::{
    VkCreateDevice, VkGetPhysicalDeviceFeatures, VkGetPhysicalDeviceProperties,
    VkGetPhysicalDeviceQueueFamilyProperties, VkPhysicalDevice, VkPhysicalDeviceFeatures,
    VkPhysicalDeviceLimits, VkPhysicalDeviceProperties, VkPhysicalDeviceSparseProperties,
};
pub(crate) use queue::VkQueue;
pub(crate) use structure_type::VkStructureType;
pub(crate) use types::{VkBool32, VkDeviceSize, VkFlags};
