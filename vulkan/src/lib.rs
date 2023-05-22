mod bindings;
mod instance;
mod macros;
mod physical_device;
mod version;
mod vulkan;

pub(self) use macros::*;
pub(self) use physical_device::VkPhysicalDeviceFunctions;

pub use crate::vulkan::Vulkan;
pub use bindings::{
    VkApplicationInfo, VkExtensionProperties, VkExtent3D, VkInstanceCreateFlagBits,
    VkInstanceCreateFlags, VkInstanceCreateInfo, VkLayerProperties, VkPhysicalDeviceFeatures,
    VkPhysicalDeviceLimits, VkPhysicalDeviceProperties, VkPhysicalDeviceSparseProperties,
    VkPhysicalDeviceType, VkQueueFamilyProperties, VkQueueFlagBits, VkQueueFlags, VkResult,
    VkSampleCountFlagBits, VkSampleCountFlags, VK_UUID_SIZE,
};
pub use instance::VkInstance;
pub use loader::Loader;
pub use native::NativeLoader;
pub use physical_device::VkPhysicalDevice;
pub use version::{
    VkVersion, VK_API_VERSION_1_0, VK_API_VERSION_1_1, VK_API_VERSION_1_2, VK_API_VERSION_1_3,
    VK_HEADER_VERSION, VK_HEADER_VERSION_COMPLETE,
};

pub type Result<T> = std::result::Result<T, VkResult>;
