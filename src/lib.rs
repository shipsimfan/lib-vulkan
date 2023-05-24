#![feature(extern_types)]
#![feature(const_cstr_methods)]

mod device;
mod fence;
mod instance;
mod loader;
mod macros;
mod physical_device;
mod queue;
mod semaphore;
mod surface;
mod swapchain;
mod vulkan;

pub mod bindings;

pub(self) use macros::*;
pub(self) use physical_device::VkPhysicalDeviceFunctions;
pub(self) use surface::VkSurfaceKHRFunctions;

#[cfg(target_os = "windows")]
pub(self) use surface::VkWin32SurfaceKHRFunctions;

pub use crate::vulkan::Vulkan;
pub use device::VkDevice;
pub use fence::VkFence;
pub use instance::VkInstance;
pub use loader::{Loader, NativeLoader};
pub use physical_device::VkPhysicalDevice;
pub use queue::VkQueue;
pub use semaphore::VkSemaphore;
pub use surface::VkSurfaceKHR;
pub use swapchain::VkSwapchainKHR;

pub use bindings::{
    VkApplicationInfo, VkColorSpaceKHR, VkCompositeAlphaFlagBitsKHR, VkCompositeAlphaFlagsKHR,
    VkDeviceCreateInfo, VkDeviceQueueCreateFlagBits, VkDeviceQueueCreateFlags,
    VkDeviceQueueCreateInfo, VkExtensionProperties, VkExtent2D, VkExtent3D, VkFormat, VkImage,
    VkImageLayout, VkImageUsageFlagBits, VkImageUsageFlags, VkInstanceCreateFlagBits,
    VkInstanceCreateFlags, VkInstanceCreateInfo, VkLayerProperties, VkOffset2D,
    VkPhysicalDeviceFeatures, VkPhysicalDeviceLimits, VkPhysicalDeviceProperties,
    VkPhysicalDeviceSparseProperties, VkPhysicalDeviceType, VkPresentModeKHR,
    VkQueueFamilyProperties, VkQueueFlagBits, VkQueueFlags, VkRect2D, VkResult,
    VkSampleCountFlagBits, VkSampleCountFlags, VkSharingMode, VkSurfaceCapabilitiesKHR,
    VkSurfaceFormatKHR, VkSurfaceTransformFlagBitsKHR, VkSurfaceTransformFlagsKHR,
    VkSwapchainCreateFlagsKHR, VkSwapchainCreateInfoKHR, VkVersion, VK_API_VERSION_1_0,
    VK_API_VERSION_1_1, VK_API_VERSION_1_2, VK_API_VERSION_1_3, VK_HEADER_VERSION,
    VK_HEADER_VERSION_COMPLETE, VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME,
    VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION, VK_KHR_SURFACE_EXTENSION_NAME,
    VK_KHR_SURFACE_SPEC_VERSION, VK_KHR_SWAPCHAIN_EXTENSION_NAME, VK_KHR_SWAPCHAIN_SPEC_VERSION,
    VK_UUID_SIZE,
};

#[cfg(target_os = "windows")]
pub use bindings::{
    VkWin32SurfaceCreateInfoKHR, VK_KHR_WIN32_SURFACE_EXTENSION_NAME,
    VK_KHR_WIN32_SURFACE_SPEC_VERSION,
};

pub type Result<T> = std::result::Result<T, VkResult>;
