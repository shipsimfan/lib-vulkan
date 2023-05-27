#![feature(extern_types)]
#![feature(concat_idents)]

mod bindings;
mod core;
mod cstring_util;
mod extensions;
mod library;
mod loader;

pub use self::core::{
    ApplicationInfo, Device, DeviceCreateInfo, DeviceQueueCreateInfo, ExtensionProperties,
    Instance, InstanceCreateInfo, LayerProperties, PhysicalDevice, PhysicalDeviceFeatures,
    PhysicalDeviceLimits, PhysicalDeviceProperties, PhysicalDeviceSparseProperties, Queue,
};
pub use bindings::{
    VkColorSpaceKHR as ColorSpace, VkCompositeAlphaFlagBitsKHR as CompositeAlphaFlagBits,
    VkCompositeAlphaFlagsKHR as CompositeAlphaFlags, VkExtent2D as Extent2D,
    VkExtent3D as Extent3D, VkFormat as Format, VkImageUsageFlagBits as ImageUsageFlagBits,
    VkImageUsageFlags as ImageUsageFlags, VkInstance, VkPhysicalDeviceType as PhysicalDeviceType,
    VkPresentModeKHR as PresentMode, VkQueueFamilyProperties as QueueFamilyProperties,
    VkQueueFlagBits as QueueFlagBits, VkQueueFlags as QueueFlags, VkResult,
    VkSampleCountFlagBits as SampleCountFlagBits, VkSampleCountFlags as SampleCountFlags,
    VkSharingMode as SharingMode, VkSurfaceCapabilitiesKHR as SurfaceCapabilities,
    VkSurfaceFormatKHR as SurfaceFormat, VkSurfaceTransformFlagBitsKHR as SurfaceTransformFlagBits,
    VkSurfaceTransformFlagsKHR as SurfaceTransformFlags, VkVersion as Version, VK_API_VERSION_1_0,
    VK_API_VERSION_1_1, VK_API_VERSION_1_2, VK_API_VERSION_1_3, VK_HEADER_VERSION,
    VK_HEADER_VERSION_COMPLETE, VK_UUID_SIZE,
};
#[cfg(target_os = "windows")]
pub use extensions::Win32SurfaceCreateInfo;
pub use extensions::{Surface, Swapchain, SwapchainCreateInfo};
pub use library::Library;
pub use loader::{Loader, NativeLoader};

pub(crate) use self::core::*;
pub(crate) use bindings::*;
pub(crate) use cstring_util::*;
pub(crate) use extensions::*;

pub type Result<T> = std::result::Result<T, VkResult>;
