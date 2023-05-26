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
    VkExtent3D as Extent3D, VkInstance, VkPhysicalDeviceType as PhysicalDeviceType,
    VkQueueFamilyProperties as QueueFamilyProperties, VkQueueFlagBits as QueueFlagBits,
    VkQueueFlags as QueueFlags, VkResult, VkSampleCountFlagBits as SampleCountFlagBits,
    VkSampleCountFlags as SampleCountFlags, VkVersion as Version, VK_API_VERSION_1_0,
    VK_API_VERSION_1_1, VK_API_VERSION_1_2, VK_API_VERSION_1_3, VK_HEADER_VERSION,
    VK_HEADER_VERSION_COMPLETE, VK_UUID_SIZE,
};
pub use extensions::Surface;
#[cfg(target_os = "windows")]
pub use extensions::Win32SurfaceCreateInfo;
pub use library::Library;
pub use loader::{Loader, NativeLoader};

pub(crate) use self::core::*;
pub(crate) use bindings::*;
pub(crate) use cstring_util::*;
pub(crate) use extensions::*;

pub type Result<T> = std::result::Result<T, VkResult>;
