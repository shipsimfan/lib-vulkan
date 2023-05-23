#![feature(extern_types)]
#![feature(const_cstr_methods)]

mod device;
mod instance;
mod loader;
mod macros;
mod physical_device;
mod queue;
mod surface;
mod vulkan;

pub mod bindings;

pub(self) use macros::*;
pub(self) use physical_device::VkPhysicalDeviceFunctions;

pub use crate::vulkan::Vulkan;
pub use device::VkDevice;
pub use instance::VkInstance;
pub use loader::{Loader, NativeLoader};
pub use physical_device::VkPhysicalDevice;
pub use queue::VkQueue;
pub use surface::VkSurfaceKHR;

pub use bindings::{
    VkApplicationInfo, VkColorSpaceKHR, VkCompositeAlphaFlagBitsKHR, VkCompositeAlphaFlagsKHR,
    VkDeviceCreateInfo, VkDeviceQueueCreateFlagBits, VkDeviceQueueCreateFlags,
    VkDeviceQueueCreateInfo, VkExtensionProperties, VkExtent2D, VkExtent3D, VkFormat,
    VkImageUsageFlagBits, VkImageUsageFlags, VkInstanceCreateFlagBits, VkInstanceCreateFlags,
    VkInstanceCreateInfo, VkLayerProperties, VkPhysicalDeviceFeatures, VkPhysicalDeviceLimits,
    VkPhysicalDeviceProperties, VkPhysicalDeviceSparseProperties, VkPhysicalDeviceType,
    VkPresentModeKHR, VkQueueFamilyProperties, VkQueueFlagBits, VkQueueFlags, VkResult,
    VkSampleCountFlagBits, VkSampleCountFlags, VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR,
    VkSurfaceTransformFlagBitsKHR, VkSurfaceTransformFlagsKHR, VkVersion, VK_API_VERSION_1_0,
    VK_API_VERSION_1_1, VK_API_VERSION_1_2, VK_API_VERSION_1_3, VK_HEADER_VERSION,
    VK_HEADER_VERSION_COMPLETE, VK_UUID_SIZE,
};

pub type Result<T> = std::result::Result<T, VkResult>;
